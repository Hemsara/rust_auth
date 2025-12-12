extern crate redis;
use redis::{Client, Commands};

pub fn init(url: &str) -> Client {
    let client = redis::Client::open(url).unwrap();
    client
}
