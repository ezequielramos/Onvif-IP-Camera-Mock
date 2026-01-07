use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> impl IntoResponse {
    Json(HealthResponse { status: "ok" })
}

pub async fn start_http_server() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    eprintln!("HTTP server listening on {}", addr);

    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind HTTP server");

    axum::serve(listener, app)
        .await
        .expect("HTTP server crashed");
}
