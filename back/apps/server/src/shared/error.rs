use actix_web::{HttpResponse, ResponseError};
use std::fmt;

/// Application-level errors
#[derive(Debug)]
pub enum AppError {
    /// Unexpected server errors, panics, or unrecoverable states
    Internal(String),
    /// Resource not found (user, record, file, etc.)
    NotFound(String),
    /// Invalid input, validation failures, malformed requests
    BadRequest(String),
    /// Resource already exists or state conflict (e.g., duplicate phone number)
    Conflict(String),
    /// Database connection, query, or transaction errors
    Database(String),
    /// Authentication required or invalid credentials
    Unauthorized(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Internal(msg) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "internal_error",
                    "message": msg
                }))
            }
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "not_found",
                    "message": msg
                }))
            }
            AppError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "bad_request",
                    "message": msg
                }))
            }
            AppError::Conflict(msg) => {
                HttpResponse::Conflict().json(serde_json::json!({
                    "error": "conflict",
                    "message": msg
                }))
            }
            AppError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "database_error",
                    "message": "An internal error occurred"
                }))
            }
            AppError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "unauthorized",
                    "message": msg
                }))
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}
