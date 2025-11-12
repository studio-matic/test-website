use axum::{Json, Router, extract::State, routing};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_governor::{GovernorLayer, governor::GovernorConfig};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .unwrap();

    let app = Router::new()
        .route("/register", routing::post(register))
        .route("/health", routing::get(health))
        .with_state(pool)
        .layer(GovernorLayer::new(GovernorConfig::default()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .into_make_service_with_connect_info::<SocketAddr>();

    let port = env::var("PORT").expect("PORT must be set");
    let listener = TcpListener::bind(format!("[::]:{port}"))
        .await
        .unwrap_or_else(|_| panic!("Unable to bind http://[::]:{port} and 0.0.0.0:{port}"));
    println!("Listening on http://[::]:{port} and http://0.0.0.0:{port} ...");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
}

async fn register(State(pool): State<MySqlPool>, Json(req): Json<RegisterRequest>) -> &'static str {
    let _ = sqlx::query("INSERT INTO registrations (email) VALUES (?)")
        .bind(req.email)
        .execute(&pool)
        .await;

    "ok"
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}
