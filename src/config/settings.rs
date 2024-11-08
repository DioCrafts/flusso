// src/config/settings.rs

use serde::Deserialize;
use config::{Config, ConfigError};
use dotenv::dotenv;
use std::env;

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

        let mut settings = Config::default();
        settings.merge(config::Environment::default())?;

        // Usa try_deserialize en lugar de try_into
        settings.try_deserialize()
    }
}

