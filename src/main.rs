extern crate openssl;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate validator_derive;

// mod auth;
use crate::server::serv;

mod auth;
mod awc;
mod config;
mod constants;
mod database;
// mod db;
mod errors;
mod extractors;
mod handlers;
mod helpers;
mod middleware;
// mod mdels;
mod routes;
mod schema;
mod schemas;
mod server;
mod state;
mod tests;
mod validate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
