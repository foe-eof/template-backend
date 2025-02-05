use axum::{Router, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    bin_common::init();

    let web_server::Config { socket_addr } = cfg::Loader::default().load();

    let listener = TcpListener::bind(socket_addr).await.unwrap();
    let router = Router::new().route("/healthcheck", routing::get(|| async { "OK" }));

    tokio::select! {
        result = axum::serve(listener, router) => {
            result.unwrap();
        }
        _ = bin_common::signal::ctrl_c() => {}
    };
}
