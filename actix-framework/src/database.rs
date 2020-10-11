//! Database-related functions
use crate::config::{Config, CONFIG};
use actix_web::web;
use diesel::{
    mysql::MysqlConnection,
    pg::PgConnection,
    r2d2::{ConnectionManager, PoolError},
    sqlite::SqliteConnection,
    Connection,
};

#[serde(untagged)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
pub enum DatabaseConnection {
    Cockroach,
    Mysql,
    Postgres,
    Sqlite,
}

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type CockroachConnectionPool = Pool<PgConnection>;
pub type MysqlConnectionPool = Pool<MysqlConnection>;
pub type PostgresConnectionPool = Pool<PgConnection>;
pub type SqliteConnectionPool = Pool<SqliteConnection>;

#[cfg(feature = "cockraoch")]
pub type PoolType = CockroachConnectionPool;

#[cfg(feature = "mysql")]
pub type PoolType = MysqlConnectionPool;

#[cfg(feature = "postgres")]
pub type PoolType = PostgresConnectionPool;

#[cfg(feature = "sqlite")]
pub type PoolType = SqliteConnectionPool;

/// database connection pool
#[derive(Clone)]
pub enum DatabaseConnectionPool {
    Cockroach(CockroachConnectionPool),
    Mysql(MysqlConnectionPool),
    Postgres(PostgresConnectionPool),
    Sqlite(SqliteConnectionPool),
}

impl DatabaseConnectionPool {
    pub fn init_pool(config: Config) -> Result<Self, r2d2::Error> {
        match config.database {
            DatabaseConnection::Cockroach => {
                init_pool::<PgConnection>(config).map(InferPool::Cockroach)
            }
            DatabaseConnection::Mysql => init_pool::<MysqlConnection>(config).map(InferPool::Mysql),
            DatabaseConnection::Postgres => {
                init_pool::<PgConnection>(config).map(InferPool::Postgres)
            }
            DatabaseConnection::Sqlite => {
                init_pool::<SqliteConnection>(config).map(InferPool::Sqlite)
            }
        }
        .map_err(Into::into)
    }
}

pub fn init_pool<T>(config: Config) -> Result<Pool<T>, PoolError>
where
    T: Connection + 'static,
{
    let manager = ConnectionManager::<T>::new(config.database_url);
    Pool::builder().build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = DatabaseConnectionPool::init_pool(CONFIG.clone())
        .expect("Failed to create connection pool");
    match pool {
        DatabaseConnectionPool::Cockroach(cockroach_pool) => cfg.data(cockroach_pool),
        DatabaseConnectionPool::Mysql(mysql_pool) => cfg.data(mysql_pool),
        DatabaseConnectionPool::Postgres(postgres_pool) => cfg.data(postgres_pool),
        DatabaseConnectionPool::Sqlite(sqlite_pool) => cfg.data(sqlite_pool),
    };
}
