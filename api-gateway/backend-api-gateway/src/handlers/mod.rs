pub mod auth;
pub mod proxy;

use actix_web::error::Error;
use actix_web::{get, HttpResponse};
use serde::Serialize;
use chrono::Utc;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            data: None,
        }
    }
}

// Añade la función health_check
#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339()
    }))
}
