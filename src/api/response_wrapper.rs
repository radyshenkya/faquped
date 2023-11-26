use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ApiResponseWrapper<T: Serialize>(T);

impl<T: Serialize> From<T> for ApiResponseWrapper<T> {
    fn from(value: T) -> Self {
        ApiResponseWrapper(value)
    }
}

impl<T: Serialize> IntoResponse for ApiResponseWrapper<T> {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::OK,
            Json(json!({
                "status": true,
                "data": self.0
            })),
        )
            .into_response()
    }
}
