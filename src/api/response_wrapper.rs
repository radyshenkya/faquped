use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub struct ApiResponseWrapper<T: Serialize>(T, StatusCode);

impl<T: Serialize> From<T> for ApiResponseWrapper<T> {
    fn from(value: T) -> Self {
        ApiResponseWrapper(value, StatusCode::OK)
    }
}

impl<T: Serialize> From<(T, StatusCode)> for ApiResponseWrapper<T> {
    fn from(value: (T, StatusCode)) -> Self {
        ApiResponseWrapper(value.0, value.1)
    }
}

impl<T: Serialize> IntoResponse for ApiResponseWrapper<T> {
    fn into_response(self) -> axum::response::Response {
        (
            self.1,
            Json(json!({
                "status": true,
                "data": self.0
            })),
        )
            .into_response()
    }
}
