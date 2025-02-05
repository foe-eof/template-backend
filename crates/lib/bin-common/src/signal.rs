//! Signal handling utilities

/// Handle SIGINT (Ctrl-C) signal
pub async fn ctrl_c() {
    tokio::signal::ctrl_c().await.unwrap();
    tracing::info!("received SIGINT, shutting down");
}
