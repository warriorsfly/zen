#[macro_use]
extern crate diesel;
/// if using redis, then use redis_async
#[cfg(feature = "redis")]
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;

pub mod awc;
pub mod cache;
pub mod config;
pub mod database;
pub mod errors;
extern crate actix;
extern crate actix_web;
extern crate serde;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
