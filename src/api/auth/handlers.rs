use axum::{extract::State, Json};
use jsonwebtoken::get_current_timestamp;
use log::warn;
use serde::Deserialize;
use serde_valid::Validate;

use crate::api::{
    auth::UserClaims, error::ApiError, state::ApiState, table_name, ApiResponse, User, DB,
};

use crate::api::util::hash_password;

pub async fn signup(
    // State(api_state): State<ApiState>,
    Json(payload): Json<RegisterPayload>,
) -> ApiResponse<()> {
    payload.validate()?;

    let user: Option<User> = DB
        .create((table_name::USER, &payload.username))
        .content(User {
            username: payload.username,
            password: hash_password(&payload.password),
        })
        .await?;

    if let None = user {
        return Err(ApiError::UserAlreadyRegistered);
    }

    Ok(().into())
}

pub async fn login(
    State(api_state): State<ApiState>,
    Json(payload): Json<LoginPayload>,
) -> ApiResponse<String> {
    payload.validate()?;

    let user: Option<User> = DB
        .query("SELECT * FROM type::table($table) WHERE username = $username LIMIT 1")
        .bind(("table", "user"))
        .bind(("username", &payload.username))
        .await?
        .take(0)?;

    if let None = user {
        return Err(ApiError::InvalidUserCredentials);
    }

    let user = user.unwrap();

    if user.password != hash_password(&payload.password) {
        return Err(ApiError::InvalidUserCredentials);
    }

    let user_claims = UserClaims {
        exp: get_current_timestamp() + crate::env_args::ARGS.jwt_secret_timeout_seconds,
        username: user.username.to_string(),
    };

    match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &user_claims,
        &api_state.jwt_state.encoding_key,
    ) {
        Ok(token) => Ok(token.into()),
        Err(e) => {
            warn!("{e:?}");
            Err(ApiError::Internal(String::from("")))
        }
    }
}

#[derive(Debug, Validate, Clone, Deserialize)]
pub struct RegisterPayload {
    #[validate(min_length = 3)]
    #[validate(max_length = 128)]
    #[validate(custom(validate_alphanumeric))]
    pub username: String,
    #[validate(min_length = 8)]
    #[validate(max_length = 128)]
    pub password: String,
}

#[derive(Debug, Validate, Clone, Deserialize)]
pub struct LoginPayload {
    #[validate(min_length = 3)]
    #[validate(max_length = 128)]
    pub username: String,
    #[validate(min_length = 8)]
    #[validate(max_length = 128)]
    pub password: String,
}

pub fn validate_alphanumeric(string: &str) -> Result<(), serde_valid::validation::Error> {
    if !string.chars().all(|x| x.is_alphanumeric()) {
        return Err(serde_valid::validation::Error::Custom(String::from(
            "Must be alphanumeric!",
        )));
    }

    Ok(())
}
