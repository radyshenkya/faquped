use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum ApiError {
    #[error("Signing up failed. This user is already registered.")]
    UserAlreadyRegistered,
    #[error("Logging in failed. Invalid user credentials")]
    InvalidUserCredentials,
    #[error(transparent)]
    ValidationFailed(#[from] serde_valid::validation::Errors),
    // TODO: Remove it
    #[error(transparent)]
    Database(#[from] surrealdb::Error),
    #[error("Unknown internal error. {0}")]
    Internal(String),
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::UserAlreadyRegistered => StatusCode::CONFLICT,
            ApiError::InvalidUserCredentials => StatusCode::UNAUTHORIZED,
            ApiError::ValidationFailed(_) => StatusCode::BAD_REQUEST,
            ApiError::Database(_err) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Internal(_err) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code(),
            Json(json!({
                "status": false,
                "err": self
            })),
        )
            .into_response()
    }
}
