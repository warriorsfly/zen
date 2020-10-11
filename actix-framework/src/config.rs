//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into a struct.
//!
//! dotenv allows environment variables to be augmented/overwriten by a
//! .env file.
//!
//! This file throws the Config struct into a CONFIG lazy_static to avoid
//! multiple processing.

use dotenv::dotenv;
use serde::Deserialize;
#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub auth_salt: String,
    pub database_url: String,
    pub jwt_expiration: i64,
    pub jwt_key: String,
    #[cfg(feature = "redis")]
    pub redis_url: String,
    pub backtrace: u8,
    pub log: String,
    pub server: String,
    // pub session_key: String,
    // pub session_name: String,
    // pub session_secure: bool,
    // pub session_timeout: i64,
}

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// get the server config
#[allow(dead_code)]
fn get_config() -> Config {
    dotenv().ok();
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error:{:#?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_a_config() {
        let config = get_config();
        assert_ne!(config.server, "".to_string());
    }
}
