use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Hello from Axum!"
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // load .env saat lokal

    let port = env::var("PORT").unwrap_or("8080".into());
    let addr = format!("0.0.0.0:{port}");

    println!("Server on http://{addr}");

    let app = Router::new().route("/api/hello", get(hello));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
