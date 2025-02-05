use axum::{Router, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    bin_common::init();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let router = Router::new().route("/healthcheck", routing::get(|| async { "OK" }));
    axum::serve(listener, router).await.unwrap();
}
