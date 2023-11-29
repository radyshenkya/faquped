use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: Thing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(serialize_with = "option_thing_serialize")]
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(serialize_with = "option_thing_serialize")]
    pub id: Option<Thing>,
    // #[serde(serialize_with = "thing_serialize")]
    pub creator: Thing,
    pub name: String,
    pub plates: Vec<FaqPlate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, serde_valid::Validate)]
pub struct FaqPlate {
    #[validate(min_length = 1)]
    #[validate(max_length = 400)]
    pub name: String,
    #[validate(min_length = 1)]
    #[validate(max_length = 1000)]
    pub content: String,
}

// fn thing_serialize<S>(x: &Thing, s: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer
// {
//     s.serialize_str(&x.to_raw())
// }
// 
// fn option_thing_serialize<S>(x: &Option<Thing>, s: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer
// {
//     if let Some(thing) = x {
//         s.serialize_str(&thing.to_raw())
//     } else {
//         s.serialize_none()
//     }
// }

pub mod table_name {
    pub const USER: &'static str = "user";
    pub const FAQ_PAGE: &'static str = "faq";
}
