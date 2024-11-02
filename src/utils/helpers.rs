// Funciones auxiliares comunes
// src/utils/helpers.rs

use std::time::{SystemTime, UNIX_EPOCH};

/// Devuelve el timestamp actual en segundos desde el UNIX epoch.
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs()
}

/// Convierte una dirección en cadena (como "127.0.0.1:8080") en un `SocketAddr`.
/// Devuelve `None` si la conversión falla.
pub fn parse_address(addr: &str) -> Option<std::net::SocketAddr> {
    addr.parse().ok()
}

/// Retorna una representación amigable de los bytes como "KB", "MB", etc.
pub fn human_readable_bytes(bytes: u64) -> String {
    let mut size = bytes as f64;
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut i = 0;

    while size >= 1024.0 && i < units.len() - 1 {
        size /= 1024.0;
        i += 1;
    }
    format!("{:.2} {}", size, units[i])
}
