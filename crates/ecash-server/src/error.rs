use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("eCash core error: {0}")]
    Ecash(#[from] ecash_core::EcashError),

    #[error("Invalid denomination: {0}")]
    InvalidDenomination(u64),

    #[error("Token already spent")]
    TokenAlreadySpent,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
            ApiError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error".to_string()),
            ApiError::Ecash(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            ApiError::InvalidDenomination(d) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid denomination: {}", d),
            ),
            ApiError::TokenAlreadySpent => {
                (StatusCode::CONFLICT, "Token already spent".to_string())
            }
            ApiError::TokenExpired => (StatusCode::BAD_REQUEST, "Token expired".to_string()),
            ApiError::InvalidSignature => {
                (StatusCode::BAD_REQUEST, "Invalid signature".to_string())
            }
            ApiError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
