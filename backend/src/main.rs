use axum::{
    extract::Json,
    routing::post,
    Router,
    http::StatusCode,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;

#[derive(Deserialize)]
struct TextPayload {
    text: String,
}

#[axum::debug_handler]
async fn submit(Json(payload): Json<TextPayload>) -> (StatusCode, Json<serde_json::Value>) {
    println!("Received message: {}", payload.text);
    (
        StatusCode::OK,
        Json(serde_json::json!({ "status": "Received" })),
    )
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/submit", post(submit))
        .layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Backend listening on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
