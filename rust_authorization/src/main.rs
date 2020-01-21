#![allow(unused_must_use)]

#[macro_use]
extern crate serde_json;
extern crate actix_rt;

#[actix_rt::main]
fn main() -> io::Result<()>
{
    println!("Hello, world!");
}
