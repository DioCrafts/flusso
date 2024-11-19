use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/plugins")
            .route("/status", web::get().to(|| async { "Plugin status endpoint" })), // Ejemplo
    );
}
