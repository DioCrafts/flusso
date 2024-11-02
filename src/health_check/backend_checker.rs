// Comprobación de salud para los backends
// src/health_check/backend_checker.rs

use reqwest::Client;
use std::time::Duration;
use tokio::time::timeout;
use log::{info, error};

/// Configuración para los parámetros de health check de los backends.
pub struct HealthCheckConfig {
    pub timeout_duration: Duration,
    pub retry_count: u8,
}

/// Realiza una comprobación de salud en un backend especificado por su URL.
/// Devuelve `true` si el backend responde exitosamente dentro del tiempo límite, `false` en caso contrario.
pub async fn check_backend_health(client: &Client, url: &str, config: &HealthCheckConfig) -> bool {
    for attempt in 1..=config.retry_count {
        let result = timeout(config.timeout_duration, client.get(url).send()).await;

        match result {
            Ok(Ok(response)) if response.status().is_success() => {
                info!("Health check succeeded for backend: {}", url);
                return true;
            }
            Ok(Ok(response)) => {
                error!(
                    "Health check failed with status {} for backend: {} (attempt {}/{})",
                    response.status(),
                    url,
                    attempt,
                    config.retry_count
                );
            }
            Ok(Err(e)) => {
                error!(
                    "Health check request error for backend {}: {} (attempt {}/{})",
                    url,
                    e,
                    attempt,
                    config.retry_count
                );
            }
            Err(_) => {
                error!(
                    "Health check timed out for backend {} (attempt {}/{})",
                    url,
                    attempt,
                    config.retry_count
                );
            }
        }
    }

    info!("Health check failed for backend after {} attempts: {}", config.retry_count, url);
    false
}
