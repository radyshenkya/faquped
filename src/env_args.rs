use envy;
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static ARGS: Lazy<EnvArgs> = Lazy::new(|| {
    dotenv::dotenv().unwrap();
    envy::from_env::<EnvArgs>().expect("Env args is incorrect")
});

#[derive(Debug, Clone, Deserialize)]
pub struct EnvArgs {
    pub server_ip: String,
    pub jwt_secret: String,
    pub jwt_secret_timeout_seconds: u64,
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_ns: String,
    pub db_db: String,
}
