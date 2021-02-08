use crate::config::{Config, CONFIG};
use actix_web::web;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    PgConnection,
};

mod user;
pub use self::user::*;

pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;
pub type ConnectionPool = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(config: Config) -> Result<DatabasePool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    Pool::builder().build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = init_pool(CONFIG.clone()).expect("Failed to create connection pool");
    cfg.data(pool);
}
