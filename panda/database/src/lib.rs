use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    PgConnection,
};

pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;
pub type ConnectionPool = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<DatabasePool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}
