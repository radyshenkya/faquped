use axum::extract::FromRef;

use crate::api::state::ApiState;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub api_state: ApiState,
}
