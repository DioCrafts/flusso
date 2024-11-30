use actix_cors::Cors;
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;
use std::sync::Arc;

mod config;
mod handlers;
mod middleware;
mod models;
mod services;

use handlers::auth::AuthHandler;
use handlers::health_check;
use handlers::metrics::MetricsState;
use config::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configurar logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .compact()
        .init();

    info!("Starting API Gateway");

    // Cargar configuración con manejo de errores mejorado
    info!("Loading settings from config...");
    let settings = match Settings::new() {
        Ok(settings) => {
            info!("Settings loaded successfully");
            Arc::new(settings)
        },
        Err(e) => {
            error!("Failed to load settings: {}", e);
            std::process::exit(1);
        }
    };

    info!("Initializing auth handler...");
    let auth_handler = web::Data::new(AuthHandler::new(settings.clone()));

    info!("Initializing metrics state...");
    let metrics_state = match MetricsState::new().await {
        Ok(state) => {
            info!("Metrics state initialized successfully");
            web::Data::new(state)
        },
        Err(e) => {
            error!("Failed to initialize metrics state: {}", e);
            std::process::exit(1);
        }
    };

    info!("Starting HTTP server on 0.0.0.0:3000...");
    HttpServer::new(move || {
        info!("Configuring application...");
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(actix_middleware::Logger::default())
            .app_data(auth_handler.clone())
            .app_data(metrics_state.clone())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(AuthHandler::login))
                            .route("/logout", web::post().to(AuthHandler::logout))
                    )
                    .service(
                        web::scope("/metrics")
                            .service(handlers::metrics::get_gateway_metrics)
                            .service(handlers::metrics::get_services_health)
                    )
            )
            .service(health_check)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}