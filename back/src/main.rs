use axum::{Json, Router, routing::post};
use serde::Deserialize;
use sqlx::MySqlPool;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
}

#[tokio::main]
async fn main() {
    let pool =
        MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();

    let app = Router::new()
        .route("/register", post(register))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let port = std::env::var("PORT").expect("PORT must be set");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn register(
    axum::extract::State(pool): axum::extract::State<MySqlPool>,
    Json(req): Json<RegisterRequest>,
) -> &'static str {
    let _ = sqlx::query("INSERT INTO registrations (email) VALUES (?)")
        .bind(req.email)
        .execute(&pool)
        .await;

    "ok"
}
