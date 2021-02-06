use crate::database::DatabasePool;

pub mod root;
pub mod user;
pub struct DataSource {
    pub database: DatabasePool,
}

impl juniper::Context for DataSource {}
