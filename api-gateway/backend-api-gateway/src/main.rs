use actix_cors::Cors;
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::sync::Arc;

mod config;
mod handlers; // Importa el módulo handlers
mod middleware;
mod models;
mod services;

use handlers::auth::AuthHandler;
use handlers::health_check; // Importa la función health_check
use config::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configurar logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .compact()
        .init();

    info!("Starting API Gateway");

    // Cargar configuración
    let settings = Arc::new(Settings::new().expect("Failed to load settings"));

    // Crear instancia de AuthHandler
    let auth_handler = web::Data::new(AuthHandler::new(settings.clone()));

    // Configurar servidor
    HttpServer::new(move || {
        // Configurar CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(actix_middleware::Logger::default())
            .app_data(auth_handler.clone())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(AuthHandler::login))
                            .route("/logout", web::post().to(AuthHandler::logout))
                    )
            )
            .service(health_check) // Usa la función health_check
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
