use std::fmt::Debug;

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::IntoResponse,
    RequestPartsExt, TypedHeader,
};
use hyper::StatusCode;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Clone)]
pub enum JwtError {
    TokenMissing,
    Expired,
    InvalidSignature,
    InvalidToken,
    Internal,
}

impl IntoResponse for JwtError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::TokenMissing => (StatusCode::UNAUTHORIZED, "Token is missing"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            Self::InvalidSignature => (StatusCode::UNAUTHORIZED, "Token with invalid signature"),
            Self::Expired => (StatusCode::UNAUTHORIZED, "Token expired"),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        }
        .into_response()
    }
}

#[derive(Clone, FromRef)]
pub struct JwtDecoderState {
    pub decoding_key: DecodingKey,
    pub encoding_key: EncodingKey,
    pub validation: Validation,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Claims<T>(T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Claims<T>
where
    JwtDecoderState: FromRef<S>,
    S: Send + Sync,
    Self: DeserializeOwned,
    T: Debug,
{
    type Rejection = JwtError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let authorization: TypedHeader<Authorization<Bearer>> = parts
            .extract()
            .await
            .map_err(|_| Self::Rejection::TokenMissing)?;

        let decoder_state = JwtDecoderState::from_ref(state);

        let token = decode::<Self>(
            &authorization.token(),
            &decoder_state.decoding_key,
            &decoder_state.validation,
        );

        let token = token.map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Self::Rejection::Expired,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => Self::Rejection::InvalidSignature,
            jsonwebtoken::errors::ErrorKind::InvalidToken => Self::Rejection::InvalidToken,
            _ => Self::Rejection::Internal,
        })?;

        Ok(token.claims)
    }
}
