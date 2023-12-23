use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod app;
pub mod auth_backend;
pub mod entities;

pub enum ApiError {
    RecordNotFound,
    ParamError(String),
    DatabaseError(sea_orm::DbErr),
    IoError(std::io::Error),
    UnknownError(String),
    ConflictError(String),
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            ApiError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An unexpected exception has occured: {}", err),
            ),
            ApiError::RecordNotFound => (StatusCode::NOT_FOUND, r#"Record not found"#.to_string()),
            ApiError::IoError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO Error: {}", err),
            ),
            ApiError::UnknownError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unknown error: {}", err),
            ),
            ApiError::ParamError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Missing: {}", err),
            ),
            ApiError::ConflictError(err) => (StatusCode::CONFLICT, err),
        };

        (
            status_code,
            Json(ErrorResponse {
                error: error_message,
            }),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        let error = err.into().to_string();
        Self::UnknownError(error)
    }
}
