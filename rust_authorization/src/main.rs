#![allow(unused_must_use)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_rt;
extern crate env_logger;
extern crate serde;
extern crate dotenv;
extern crate futures;
extern crate failure;
extern crate derive_more;
extern crate jsonwebtoken;
extern crate uuid;
extern crate bcrypt;
extern crate time;

mod api;
mod config;
mod constants;
mod error;
//mod middleware;
mod entity;
mod schema;
mod services;
mod utils;
// #[actix_rt::main]

fn main()
// fn main() -> io::Result<()>
{
    println!("Hello, world!");
}
