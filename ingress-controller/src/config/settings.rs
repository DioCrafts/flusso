// src/config/settings.rs

use serde::Deserialize;
use config::{Config, ConfigError, Environment};
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server_addr: String,
    pub gui_port: Option<u16>,
    pub tls_enabled: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let settings = Config::builder()  // Uses ConfigBuilder internally, no explicit import needed
            .add_source(Environment::default())
            .build()?; // Builds the configuration

        settings.try_deserialize()
    }
}
