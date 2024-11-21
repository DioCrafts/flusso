//src/backends/routers.rs
use actix_web::web;
use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/backends")
            .route("", web::get().to(handlers::list_backends))
            .route("", web::post().to(handlers::add_backend))
            .route("/{id}", web::put().to(handlers::update_backend))
            .route("/{id}", web::delete().to(handlers::delete_backend))
            .route("/health-check", web::get().to(handlers::health_check)),
    );
}
