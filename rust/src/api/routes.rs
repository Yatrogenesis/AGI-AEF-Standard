use actix_web::web;
use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health and info endpoints
        .route("/health", web::get().to(handlers::health_check))
        .route("/version", web::get().to(handlers::get_version))

        // Dimension information
        .route("/api/v1/dimensions", web::get().to(handlers::get_dimensions))

        // Assessment endpoints
        .route("/api/v1/assessments", web::post().to(handlers::submit_assessment))
        .route("/api/v1/assessments/{id}", web::get().to(handlers::get_assessment_result));
}
