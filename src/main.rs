extern crate openssl;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use crate::server::serv;

mod awc;
mod config;
mod constants;
mod database;
mod errors;
pub mod handlers;
mod helpers;
mod models;
mod routes;
mod schema;
mod security;
mod server;
mod state;
mod tests;
mod validate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
