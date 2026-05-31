use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(msg) => HttpResponse::NotFound().json(serde_json::json!({
                "error": msg
            })),
            AppError::Auth(msg) => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": msg
            })),
            AppError::Validation(msg) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": msg
            })),
            _ => {
                tracing::error!("Internal error: {:?}", self);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                }))
            }
        }
    }
}
