use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

pub mod pagination;

pub type DatabaseConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool<'a>(database_url: &'a str) -> Result<DatabaseConnectionPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}
