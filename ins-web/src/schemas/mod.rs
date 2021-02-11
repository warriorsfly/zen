use actix_web::web;
use ins_database::DatabasePool;

use self::root::init_schema;

pub mod root;
pub mod user;
pub struct DataSource {
    pub database: DatabasePool,
}

impl juniper::Context for DataSource {}

pub fn add_graphql(cfg: &mut web::ServiceConfig) {
    let schema = init_schema();
    cfg.data(schema);
}
