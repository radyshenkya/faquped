use crate::api::model::*;
use crate::api::ApiResponse;

use axum::Json;
use axum::extract::Path;
use serde::Deserialize;

pub fn create_faq(Json(payload): Json<FaqPageCreatePayload>) -> ApiResponse<FaqPage> {
    todo!();
}

#[derive(Debug, Deserialize)]
pub struct FaqPageCreatePayload {
    name: String,
}

pub fn delete_faq(Path((faq_id)): Path<(String)>) -> ApiResponse<()> {
    todo!();
}

pub fn update_faq(Path((faq_id)): Path<(String)>) -> ApiResponse<()> {
    todo!()
}
