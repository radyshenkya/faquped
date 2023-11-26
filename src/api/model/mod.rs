use once_cell::sync::Lazy;
use surrealdb::{opt::auth::Root, Surreal};

use crate::env_args::ARGS;

mod structs;
pub use structs::*;

pub static DB: Lazy<Surreal<surrealdb::engine::any::Any>> = Lazy::new(Surreal::init);

pub async fn init_db() -> Result<(), surrealdb::Error> {
    DB.connect(&ARGS.db_host).await?;

    DB.signin(Root {
        username: &ARGS.db_user,
        password: &ARGS.db_password,
    })
    .await?;
    DB.use_ns(&ARGS.db_ns).use_db(&ARGS.db_db).await?;

    Ok(())
}
