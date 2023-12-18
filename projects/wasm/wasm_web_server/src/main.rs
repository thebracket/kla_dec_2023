use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use serde::Serialize;

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_json() -> axum::Json<HelloJson> {
    axum::Json(HelloJson {
        message: "Hello, World!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/json", get(say_hello_json))
        .fallback_service(ServeDir::new("web"));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
