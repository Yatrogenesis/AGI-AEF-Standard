/// Secure REST API for AGI Assessment
///
/// Features:
/// - JWT authentication
/// - Rate limiting
/// - CORS support
/// - Request validation
/// - Comprehensive error handling
/// - OpenAPI documentation

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::sync::Arc;

pub mod auth;
pub mod handlers;
pub mod middleware as api_middleware;
pub mod routes;
pub mod errors;

pub use handlers::*;
pub use errors::ApiError;

use crate::assessment::AssessmentEngine;
use crate::models::AssessmentConfig;

/// API Server configuration
pub struct ApiServer {
    host: String,
    port: u16,
    assessment_engine: Arc<AssessmentEngine>,
}

impl ApiServer {
    pub fn new(
        host: String,
        port: u16,
        config: AssessmentConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let engine = AssessmentEngine::new(config)?;

        Ok(Self {
            host,
            port,
            assessment_engine: Arc::new(engine),
        })
    }

    /// Start the API server
    pub async fn run(self) -> std::io::Result<()> {
        let engine = self.assessment_engine.clone();
        let bind_address = format!("{}:{}", self.host, self.port);

        tracing::info!("Starting AGI-AEF API server on {}", bind_address);

        HttpServer::new(move || {
            // Configure CORS
            let cors = Cors::default()
                .allowed_origin_fn(|origin, _req_head| {
                    origin.as_bytes().starts_with(b"https://")
                        || origin.as_bytes().starts_with(b"http://localhost")
                })
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::CONTENT_TYPE,
                ])
                .max_age(3600);

            App::new()
                .app_data(web::Data::new(engine.clone()))
                // Middleware
                .wrap(middleware::Logger::default())
                .wrap(middleware::Compress::default())
                .wrap(cors)
                .wrap(api_middleware::RateLimiter::new())
                // Routes
                .configure(routes::configure_routes)
        })
        .bind(&bind_address)?
        .run()
        .await
    }
}

/// API Response wrapper
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}
