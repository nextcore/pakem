use pakem::prelude::*;
use serde_json::json;
use tokio::sync::OnceCell;

static DB_INIT: OnceCell<()> = OnceCell::const_new();

/// Benchmark Server Pakem Framework (Reproducible Reference)
/// Jalankan dengan:
///   cargo run --release --example bench_server
/// Endpoints:
///   GET /plaintext -> "Hello, World!"
///   GET /json      -> {"message":"Hello, World!","framework":"pakem"}
///   GET /db-users  -> Query SQLite in-memory (mengukur overhead ORM/DB pool)
#[tokio::main]
async fn main() {
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        std::env::set_var("APP_ENV", "production");
        std::env::set_var("RATE_LIMIT_MAX", "10000000");
    }

    let router = Router::new()
        .get("/plaintext", || async { "Hello, World!" })
        .get("/json", || async {
            Json(json!({
                "message": "Hello, World!",
                "framework": "pakem"
            }))
        })
        .get("/db-users", |ExtractDb(db): ExtractDb| async move {
            DB_INIT
                .get_or_init(|| async {
                    let _ = db
                        .execute_raw("CREATE TABLE IF NOT EXISTS bench_users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
                        .await;
                    for i in 1..=10 {
                        let sql = format!("INSERT INTO bench_users (name) VALUES ('User {}')", i);
                        let _ = db.execute_raw(&sql).await;
                    }
                })
                .await;

            let rows: Vec<(i64, String)> = sqlx::query_as("SELECT id, name FROM bench_users LIMIT 10")
                .fetch_all(db.raw_pool())
                .await
                .unwrap_or_default();
            Json(json!({ "count": rows.len(), "users": rows }))
        });

    let app = Application::new()
        .without_default_middleware()
        .with_web(router);

    println!("🚀 Pakem Benchmark Server listening at http://127.0.0.1:8080");
    app.serve("127.0.0.1:8080").await;
}
