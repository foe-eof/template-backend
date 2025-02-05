//! File type for configuration files.

use serde::Deserialize;
use std::{fs, path::PathBuf};
use url::Url;

/// Represents a file that can be loaded from the local filesystem or a remote URL.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum File {
    /// Remote file
    Remote(Url),
    /// Local file
    Local(PathBuf),
}

impl File {
    /// Reads the data to RAM
    pub async fn read(&self) -> Vec<u8> {
        match self {
            Self::Remote(url) => {
                let response = reqwest::get(url.as_str()).await.unwrap();
                response.bytes().await.unwrap().to_vec()
            }
            Self::Local(path) => fs::read(path).unwrap(),
        }
    }
}
