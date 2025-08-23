use std::net::IpAddr;

use config::{Config as C, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: IpAddr,
    pub port: u16,
}

pub fn load_config() -> Result<Config, ConfigError> {
    let settings = C::builder()
        .add_source(File::with_name("config.toml"))
        .build()?;

    settings.try_deserialize()
}
