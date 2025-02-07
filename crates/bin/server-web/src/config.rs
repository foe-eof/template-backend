use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub socket_addr: SocketAddr,
}

impl verdict::Config for Config {
    const DEFAULT_CONFIG_DIR: &'static str = "./config/server-web";
    const ENV_PREFIX: &'static str = "SERVER_WEB";
}
