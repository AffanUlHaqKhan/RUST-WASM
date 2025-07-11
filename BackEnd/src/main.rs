use axum::{
    routing::get,
    Router,
    response::Json,
    http::Method,
};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn hello() -> Json<Message> {
    Json(Message {
        message: "Sample Minimal Backend Communication Code!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/api/hello", get(hello))
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Backend listening on 0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}
