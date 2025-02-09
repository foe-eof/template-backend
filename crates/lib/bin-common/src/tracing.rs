//! Contains the `tracing` utilities.

use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

/// Initializes the default tracing subscriber.
pub fn init() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .try_init();
}
