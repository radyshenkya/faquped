mod auth;
pub mod error;
pub mod handlers;
mod model;
mod response_wrapper;
pub mod state;
pub mod util;

use axum::{middleware::from_extractor_with_state, routing::{post, delete, get}, Router};
pub use model::*;

use crate::{jwt::Claims, state::AppState};

use self::{auth::UserClaims, error::ApiError};

pub type ApiResponse<T> = Result<response_wrapper::ApiResponseWrapper<T>, ApiError>;

pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/faq", post(handlers::create_faq))
        .route("/faq/:id", delete(handlers::delete_faq))
        .route("/faq/:id", post(handlers::update_faq))
        .layer(from_extractor_with_state::<Claims<UserClaims>, AppState>(
            app_state,
        ))
        .route("/faq/:id", get(handlers::get_faq))
        .nest("/auth", auth::routes())
}
