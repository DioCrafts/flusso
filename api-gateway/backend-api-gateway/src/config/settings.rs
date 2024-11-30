use std::str::FromStr;
use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use std::env;
use tracing::{info, error};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub jwt: JwtSettings,
    pub cors: CorsSettings,
    pub metrics: MetricsSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_workers")]
    pub workers: usize,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtSettings {
    pub secret: String,
    #[serde(default = "default_jwt_expiration")]
    pub expiration: i64,
    #[serde(default = "default_jwt_refresh_expiration")]
    pub refresh_expiration: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsSettings {
    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Vec<String>,
    #[serde(default = "default_allowed_methods")]
    pub allowed_methods: Vec<String>,
    #[serde(default = "default_allowed_headers")]
    pub allowed_headers: Vec<String>,
    #[serde(default = "default_max_age")]
    pub max_age: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsSettings {
    #[serde(default = "default_metrics_enabled")]
    pub enabled: bool,
    #[serde(default = "default_metrics_path")]
    pub path: String,
}

// Funciones por defecto
fn default_host() -> String { "0.0.0.0".to_string() }
fn default_port() -> u16 { 3000 }
fn default_workers() -> usize { num_cpus::get() }
fn default_timeout() -> u64 { 30 }
fn default_jwt_expiration() -> i64 { 3600 }
fn default_jwt_refresh_expiration() -> i64 { 86400 }
fn default_allowed_origins() -> Vec<String> { vec!["*".to_string()] }
fn default_allowed_methods() -> Vec<String> {
    vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()]
}
fn default_allowed_headers() -> Vec<String> {
    vec!["Content-Type".to_string(), "Authorization".to_string()]
}
fn default_max_age() -> u32 { 3600 }
fn default_metrics_enabled() -> bool { true }
fn default_metrics_path() -> String { "/metrics".to_string() }

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        info!("Loading settings...");

        let builder = Config::builder()
            .add_source(Environment::with_prefix("APP").separator("_"));

        match builder.build() {
            Ok(config) => {
                info!("Configuration loaded successfully");
                let mut settings: Settings = config.try_deserialize().map_err(|e| {
                    error!("Failed to deserialize settings: {}", e);
                    e
                })?;

                // Procesar listas desde variables de entorno
                if let Ok(origins) = env::var("APP_CORS_ALLOWED_ORIGINS") {
                    settings.cors.allowed_origins = origins
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }

                if let Ok(methods) = env::var("APP_CORS_ALLOWED_METHODS") {
                    settings.cors.allowed_methods = methods
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }

                if let Ok(headers) = env::var("APP_CORS_ALLOWED_HEADERS") {
                    settings.cors.allowed_headers = headers
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }

                info!("Settings loaded successfully: {:?}", settings);
                Ok(settings)
            }
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                error!("Falling back to default settings");
                Ok(Settings::default())
            }
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        info!("Creating default settings");
        Self {
            server: ServerSettings {
                host: default_host(),
                port: default_port(),
                workers: default_workers(),
                timeout: default_timeout(),
            },
            jwt: JwtSettings {
                secret: std::env::var("APP_JWT_SECRET")
                    .unwrap_or_else(|_| "default-secret-key".to_string()),
                expiration: default_jwt_expiration(),
                refresh_expiration: default_jwt_refresh_expiration(),
            },
            cors: CorsSettings {
                allowed_origins: default_allowed_origins(),
                allowed_methods: default_allowed_methods(),
                allowed_headers: default_allowed_headers(),
                max_age: default_max_age(),
            },
            metrics: MetricsSettings {
                enabled: default_metrics_enabled(),
                path: default_metrics_path(),
            },
        }
    }
}