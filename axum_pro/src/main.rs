use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
    serve::Listener,
};
use tokio::net;

async fn health_check() -> &'static str {
    "it's health"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(health_check));
    let listener = tokio::net::TcpListener::bind("localhost:9090")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
