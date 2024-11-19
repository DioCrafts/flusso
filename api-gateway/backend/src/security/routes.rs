use actix_web::web;
use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/security")
            .route("/policies", web::get().to(handlers::list_policies))
            .route("/policies", web::post().to(handlers::add_policy))
            .route("/policies/{name}", web::delete().to(handlers::delete_policy))
            .route("/protected", web::get().to(handlers::protected_endpoint)),
    );
}
