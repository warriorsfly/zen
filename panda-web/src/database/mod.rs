use crate::config::CONFIG;
use actix_web::web;
use panda_database::init_pool;

mod article;
mod user;
pub use self::{article::*, user::*};

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = init_pool(&CONFIG.database_url).expect("Failed to create connection pool");
    cfg.data(pool);
}
