use axum::{routing::get, Router};
use std::{net::SocketAddr, path::Path};
use axum::response::Html;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_html));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn say_hello_html() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
