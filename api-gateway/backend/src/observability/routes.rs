use actix_web::web;
use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/observability")
            .route("/metrics", web::get().to(handlers::get_metrics))
            .route("/logs", web::get().to(handlers::get_logs))
            .route("/logs", web::post().to(handlers::add_log)) // Para agregar logs manualmente
    )
    .service(web::resource("/metrics").route(web::get().to(handlers::export_prometheus_metrics))); // Prometheus
}
