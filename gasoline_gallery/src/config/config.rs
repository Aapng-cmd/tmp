use crate::utils::utils;
use lazy_static::lazy_static;

pub struct Config {
    pub database_url: String,
    pub jwt_secret_key: String,
    pub hash_salt: String,
}

impl Config {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret_key = utils::generate_random_string(64);
        let hash_salt = utils::generate_random_string(64);

        Config {
            database_url,
            jwt_secret_key,
            hash_salt,
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}