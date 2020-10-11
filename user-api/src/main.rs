extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

// mod auth;
use crate::server::server;

mod auth;
mod constants;
mod database;
mod db;
mod extractors;
pub mod handlers;
mod helpers;
mod middleware;
mod models;
mod routes;
mod schema;
mod server;
mod state;
mod tests;
mod validate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
