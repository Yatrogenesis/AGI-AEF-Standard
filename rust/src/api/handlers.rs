use actix_web::{web, HttpResponse};
use crate::models::{AGIAEFResult, AssessmentConfig};
use super::{ApiResponse, ApiError};

/// Health check endpoint
pub async fn health_check() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().json(ApiResponse::success("OK")))
}

/// Get framework version
pub async fn get_version() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().json(ApiResponse::success(crate::FRAMEWORK_VERSION)))
}

/// Get all dimensions
pub async fn get_dimensions() -> Result<HttpResponse, ApiError> {
    use crate::models::dimension::DimensionType;
    let dimensions: Vec<_> = DimensionType::all()
        .into_iter()
        .map(|dt| {
            serde_json::json!({
                "name": dt.display_name(),
                "weight": dt.weight(),
                "description": dt.description(),
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(dimensions)))
}

/// Placeholder for assessment submission
pub async fn submit_assessment(
    _config: web::Json<AssessmentConfig>,
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Accepted().json(ApiResponse::success("Assessment queued")))
}

/// Placeholder for getting assessment results
pub async fn get_assessment_result(
    _id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    Err(ApiError::NotFound("Assessment not found".to_string()))
}
