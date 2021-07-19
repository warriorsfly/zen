mod article;
mod comment;
mod pagination;
mod user;

pub use self::{article::*, comment::*, user::*};
use crate::config::CONFIG;
use actix_web::web::{self, Data};
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

pub type DatabaseConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool<'a>(database_url: &'a str) -> Result<DatabaseConnectionPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = init_pool(&CONFIG.database_url).expect("Failed to create connection pool");
    cfg.app_data(Data::new(pool));
}
