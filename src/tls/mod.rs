// src/tls/mod.rs

pub mod certificate_manager;
pub mod acme;

use actix_web::HttpServer;
use rustls::{ServerConfig, Certificate, PrivateKey};
use std::sync::Arc;

pub struct TlsConfig {
    pub config: Arc<ServerConfig>,
}

impl TlsConfig {
    pub fn new(cert: Certificate, key: PrivateKey) -> Self {
        let mut server_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)
            .expect("TLS configuration failed");

        TlsConfig {
            config: Arc::new(server_config),
        }
    }
}

// Función para configurar el servidor HTTPS
pub fn configure_https(server: HttpServer, tls_config: TlsConfig) -> HttpServer {
    server.bind_rustls("0.0.0.0:8443", tls_config.config.clone())
          .expect("Failed to bind HTTPS server")
}
