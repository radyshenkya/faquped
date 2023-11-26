use axum::extract::FromRef;
use crate::jwt::JwtDecoderState;

#[derive(Clone, FromRef)]
pub struct ApiState {
    pub jwt_state: JwtDecoderState
}
