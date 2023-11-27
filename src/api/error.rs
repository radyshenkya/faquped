use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum ApiError {
    #[error("Signing up failed. This user is already registered.")]
    // #[serde(serialize_with = "display_serialize")]
    UserAlreadyRegistered,
    #[error("Logging in failed. Invalid user credentials")]
    // #[serde(serialize_with = "display_serialize")]
    InvalidUserCredentials,

    #[error("Cannot modify this resource")]
    CannotModifyResource,
    #[error("Not found")]
    ResourceNotFound,

    #[error(transparent)]
    ValidationFailed(#[from] serde_valid::validation::Errors),
    // TODO: Remove it
    #[error(transparent)]
    // #[serde(serialize_with = "display_serialize")]
    Database(#[from] surrealdb::Error),
    #[error("Unknown internal error.")]
    // #[serde(serialize_with = "display_serialize")]
    Internal,
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::CannotModifyResource => StatusCode::FORBIDDEN,
            ApiError::ResourceNotFound => StatusCode::NOT_FOUND,
            ApiError::UserAlreadyRegistered => StatusCode::CONFLICT,
            ApiError::InvalidUserCredentials => StatusCode::UNAUTHORIZED,
            ApiError::ValidationFailed(_) => StatusCode::BAD_REQUEST,
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
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
