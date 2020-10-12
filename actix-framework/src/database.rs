use crate::config::Config;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

pub type PoolType = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(config: Config) -> Result<PoolType, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    Pool::builder().build(manager)
}

// pub fn add_pool(cfg: &mut web::ServiceConfig) {
//     let pool = init_pool(CONFIG.clone()).expect("Failed to create connection pool");
//     cfg.data(pool);
// }
