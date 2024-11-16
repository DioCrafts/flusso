use actix_web::{web::Data, middleware::Logger, App, HttpResponse, HttpServer};
use std::sync::Arc;
use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer as Certificate, PrivateKeyDer as PrivateKey};

#[derive(Clone)]
pub struct TlsConfig {
    pub config: Arc<ServerConfig>,
}

impl TlsConfig {
    pub fn new(cert: Certificate<'static>, key: PrivateKey<'static>) -> Self {
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)
            .expect("TLS configuration failed");

        TlsConfig {
            config: Arc::new(config),
        }
    }
}

// Configuración del servidor HTTPS
pub async fn configure_https(tls_config: Arc<TlsConfig>) -> std::io::Result<()> {
    HttpServer::new(move || {
        // No se especifica el tipo explícito de retorno
        App::new()
            .wrap(Logger::default()) // Añadir Logger como middleware
            .app_data(Data::new(tls_config.clone()))
            .route("/", actix_web::web::get().to(|| async {
                HttpResponse::Ok().body("Hello, HTTPS!")
            }))
    })
    .bind(("0.0.0.0", 8443))?
    .run()
    .await
}

