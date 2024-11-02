// Configuración y administración de logs
// src/utils/logger.rs

use log::{info, warn, error, debug, LevelFilter};
use env_logger::Builder;
use std::env;

/// Configura el sistema de logs usando `env_logger`.
/// Lee el nivel de logs desde la variable de entorno `RUST_LOG`, o usa un nivel por defecto.
pub fn init_logger() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    Builder::new()
        .parse_filters(&log_level)
        .init();

    info!("Logger initialized with level: {}", log_level);
}

/// Loggea un mensaje informativo sobre el inicio del servidor.
pub fn log_server_start(addr: &str) {
    info!("Server starting on {}", addr);
}

/// Loggea un mensaje de advertencia.
pub fn log_warning(message: &str) {
    warn!("Warning: {}", message);
}

/// Loggea un mensaje de error.
pub fn log_error(message: &str) {
    error!("Error: {}", message);
}

/// Loggea un mensaje de depuración.
pub fn log_debug(message: &str) {
    debug!("Debug: {}", message);
}
