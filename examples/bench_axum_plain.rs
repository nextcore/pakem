use axum::{extract::State, routing::get, Json, Router};
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(16)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect SQLite memory DB");

    sqlx::query("CREATE TABLE IF NOT EXISTS bench_users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
        .execute(&pool)
        .await
        .unwrap();

    for i in 1..=10 {
        let _ = sqlx::query("INSERT INTO bench_users (name) VALUES (?)")
            .bind(format!("User {}", i))
            .execute(&pool)
            .await;
    }

    let app = Router::new()
        .route("/plaintext", get(|| async { "Hello, World!" }))
        .route(
            "/json",
            get(|| async {
                Json(json!({
                    "message": "Hello, World!",
                    "framework": "axum-plain"
                }))
            }),
        )
        .route(
            "/db-users",
            get(|State(db): State<SqlitePool>| async move {
                let rows: Vec<(i64, String)> =
                    sqlx::query_as("SELECT id, name FROM bench_users LIMIT 10")
                        .fetch_all(&db)
                        .await
                        .unwrap_or_default();
                Json(json!({ "count": rows.len(), "users": rows }))
            }),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("🚀 Plain Axum 0.8 Benchmark Server listening at http://127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();
}
