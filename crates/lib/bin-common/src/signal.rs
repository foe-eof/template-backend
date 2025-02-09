//! Signal handling utilities

/// Handle SIGINT (Ctrl-C) signal
///
/// # Panics
///
/// Panics if the signal handler cannot be installed.
pub async fn ctrl_c() {
    tokio::signal::ctrl_c().await.unwrap();
    tracing::info!("received SIGINT, shutting down");
}
