//! Common utilities for the binary packages.

pub mod signal;
pub mod tracing;

/// Initializes all the systems used by binaries.
pub fn init() {
    let _ = dotenvy::dotenv();
    tracing::init();
}
