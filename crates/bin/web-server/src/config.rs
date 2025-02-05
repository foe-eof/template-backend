use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub socket_addr: SocketAddr,
}

impl cfg::Config for Config {
    const DEFAULT_CONFIG_DIR: &'static str = "./config/web-server";
    const ENV_PREFIX: &'static str = "WEB_SERVER";
}
