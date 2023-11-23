extern crate dotenv;

use lazy_static::lazy_static;
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub redis: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let redis = env::var("REDIS").expect("REDIS must be set");

        Config {
            redis,
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}