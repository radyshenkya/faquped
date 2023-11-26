use crate::state::AppState;
use axum::routing::post;
use axum::Router;
use serde::{Deserialize, Serialize};

mod handlers;

pub fn routes() -> Router<AppState> {
    Router::new().route("/login", post(handlers::login))
        .route("/signup", post(handlers::signup))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub exp: u64,
    pub username: String,
}
