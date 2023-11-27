use crate::api::error::ApiError;
use crate::api::model::*;
use crate::api::table_name;
use crate::api::ApiResponse;
use crate::jwt::Claims;

use axum::extract::Path;
use axum::Json;
use hyper::StatusCode;
use serde::Deserialize;
use surrealdb::sql::Thing;

use super::auth::UserClaims;

pub async fn create_faq(
    Claims(user): Claims<UserClaims>,
    Json(payload): Json<FaqPagePayload>,
) -> ApiResponse<FaqPage> {
    let faq_page: Vec<FaqPage> = DB
        .create(table_name::FAQ_PAGE)
        .content(FaqPage {
            id: None,
            creator: Thing {
                tb: table_name::USER.to_string(),
                id: user.username.into(),
            },
            name: payload.name,
            plates: vec![],
        })
        .await?;

    match faq_page.into_iter().next() {
        Some(record) => Ok((record, StatusCode::CREATED).into()),
        None => Err(ApiError::Internal),
    }
}

pub async fn get_faq(Path(faq_id): Path<String>) -> ApiResponse<FaqPage> {
    let faq_page: Option<FaqPage> = DB.select((table_name::FAQ_PAGE, faq_id)).await?;

    match faq_page {
        Some(faq_page) => Ok(faq_page.into()),
        None => Err(ApiError::ResourceNotFound),
    }
}

pub async fn delete_faq(
    Claims(user): Claims<UserClaims>,
    Path(faq_id): Path<String>,
) -> ApiResponse<FaqPage> {
    let faq_page_deleted: Option<FaqPage> = DB
        .query("DELETE type::thing($faq_table, $faq_id) WHERE creator = type::thing($user_table, $username) RETURN BEFORE")
        .bind(("faq_table", table_name::FAQ_PAGE))
        .bind(("faq_id", &faq_id))
        .bind(("user_table", &table_name::USER))
        .bind(("username", &user.username))
        .await?
        .take(0)?;

    if let None = faq_page_deleted {
        return Err(ApiError::CannotModifyResource);
    }

    Ok(faq_page_deleted.unwrap().into())
}

pub async fn update_faq(
    Claims(user): Claims<UserClaims>,
    Path(faq_id): Path<String>,
) -> ApiResponse<()> {
    todo!()
}

#[derive(Debug, Deserialize)]
pub struct FaqPagePayload {
    name: String,
    plates: Vec<FaqPlate>,
}
