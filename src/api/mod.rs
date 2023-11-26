mod auth;
mod response_wrapper;
mod model;
pub mod util;
pub mod error;
pub mod handlers;
pub mod state;

use axum::{middleware::from_extractor_with_state, response::IntoResponse, routing::get, Router};
use log::info;
pub use model::*;

use crate::{jwt::Claims, state::AppState};

use self::{state::ApiState, auth::UserClaims, error::ApiError};

pub type ApiResponse<T> = Result<response_wrapper::ApiResponseWrapper<T>, ApiError>;

pub fn routes(api_state: ApiState) -> Router<AppState> {
    Router::new()
        .route("/test", get(test_handler))
        .layer(from_extractor_with_state::<Claims<UserClaims>, ApiState>(
            api_state,
        ))
        .nest("/auth", auth::routes())
}

async fn test_handler() -> impl IntoResponse {
    info!("yeah");
    "ABOBA"
}
