use axum::{routing::post, Json, Router};
use serde::Deserialize;
use sqlx::MySqlPool;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
}

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect(env!("DATABASE_URL")).await.unwrap();

    let app = Router::new()
        .route("/register", post(register))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn register(
    axum::extract::State(pool): axum::extract::State<MySqlPool>,
    Json(req): Json<RegisterRequest>,
) -> &'static str {
    let _ = sqlx::query!("INSERT INTO registrations (email) VALUES (?)", req.email)
        .execute(&pool)
        .await;

    "ok"
}
