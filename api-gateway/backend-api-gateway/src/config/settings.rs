// src/config/settings.rs
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: Option<DatabaseSettings>,
    pub jwt: JwtSettings,
    pub cors: CorsSettings,
    pub metrics: MetricsSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtSettings {
    pub secret: String,
    pub expiration: i64,
    pub refresh_expiration: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsSettings {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsSettings {
    pub enabled: bool,
    pub path: String,
}


// src/config/settings.rs
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Cambiar la ruta para buscar en el directorio actual
            .add_source(File::with_name("./config/default"))
            .add_source(File::with_name(&format!("./config/{}", run_mode)).required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        s.try_deserialize()
    }




    pub fn from_env() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        s.try_deserialize()
    }

    pub fn is_development(&self) -> bool {
        env::var("RUN_MODE").unwrap_or_else(|_| "development".into()) == "development"
    }
}

// Implementación de valores por defecto
impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 3000,
                workers: num_cpus::get(),
                timeout: 30,
            },
            database: None,
            jwt: JwtSettings {
                secret: "your-secret-key".to_string(),
                expiration: 3600,
                refresh_expiration: 86400,
            },
            cors: CorsSettings {
                allowed_origins: vec!["http://localhost:5173".to_string()],
                allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
                allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
                max_age: 3600,
            },
            metrics: MetricsSettings {
                enabled: true,
                path: "/metrics".to_string(),
            },
        }
    }
}