use crate::config::Config;
use crate::config::CONFIG;
use actix_web::web;
use diesel::{
    // mysql::MysqlConnection,
    pg::PgConnection,
    r2d2::{ConnectionManager, PoolError},
    sqlite::SqliteConnection,
    Connection,
};
// 电脑未安装mysql,需移除mysql的依赖,否则报错
// #[serde(untagged)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
pub enum DatabaseConnection {
    // Cockroach,
    // Mysql,
    Postgres,
    Sqlite,
}

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
// pub type CockroachPool = Pool<PgConnection>;
// pub type MysqlPool = Pool<MysqlConnection>;

pub type PostgresPool = Pool<PgConnection>;
pub type SqlitePool = Pool<SqliteConnection>;

// #[cfg(feature = "mysql")]
// pub type PoolType = MysqlPool;

#[cfg(feature = "postgres")]
pub type PoolType = PostgresPool;

#[cfg(feature = "sqlite")]
pub type PoolType = SqlitePool;

#[derive(Clone)]
pub enum DatabasePool {
    // Mysql(MysqlPool),
    Postgres(PostgresPool),
    Sqlite(SqlitePool),
}

impl DatabasePool {
    pub fn init_pool(config: Config) -> Result<Self, r2d2::Error> {
        match config.database {
            // DatabaseConnection::Mysql => {
            //     init_pool::<MysqlConnection>(config).map(DatabasePool::Mysql)
            // }
            DatabaseConnection::Postgres => {
                init_pool::<PgConnection>(config).map(DatabasePool::Postgres)
            }
            DatabaseConnection::Sqlite => {
                init_pool::<SqliteConnection>(config).map(DatabasePool::Sqlite)
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
    let pool = DatabasePool::init_pool(CONFIG.clone()).expect("Failed to create connection pool");
    match pool {
        // DatabasePool::Mysql(mysql_pool) => cfg.data(mysql_pool),
        DatabasePool::Postgres(postgres_pool) => cfg.data(postgres_pool),
        DatabasePool::Sqlite(sqlite_pool) => cfg.data(sqlite_pool),
    };
}
