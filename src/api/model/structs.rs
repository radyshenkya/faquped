use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: Thing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqPage {
    pub name: String,
    pub plates: Vec<FaqPlate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqPlate {
    pub name: String,
    pub content: String,
}

pub mod table_name {
    pub const USER: &'static str = "user";
    pub const FAQ_PAGE: &'static str = "faq";
}
