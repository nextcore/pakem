use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

#[get("/plaintext")]
async fn plaintext() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/json")]
async fn json_ep() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Hello, World!",
        "framework": "actix-web"
    }))
}

#[get("/db-users")]
async fn db_users(db: web::Data<SqlitePool>) -> impl Responder {
    let rows: Vec<(i64, String)> = sqlx::query_as("SELECT id, name FROM bench_users LIMIT 10")
        .fetch_all(db.get_ref())
        .await
        .unwrap_or_default();
    HttpResponse::Ok().json(json!({ "count": rows.len(), "users": rows }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    println!("🚀 Actix-web Benchmark Server listening at http://127.0.0.1:8080");
    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .service(plaintext)
            .service(json_ep)
            .service(db_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
