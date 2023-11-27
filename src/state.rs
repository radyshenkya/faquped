use axum::extract::FromRef;

use crate::jwt::JwtDecoderState;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub jwt_state: JwtDecoderState,
}
