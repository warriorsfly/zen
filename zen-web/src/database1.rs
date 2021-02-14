use crate::config::{Config, CONFIG};
use actix_web::web;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

pub type DatabaseConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(config: Config) -> Result<DatabaseConnectionPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    Pool::builder().build(manager)
}
