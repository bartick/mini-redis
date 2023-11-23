use redis::{Client, Connection};

use crate::utils::CONFIG;

pub fn get_connection() -> Connection {
    Client::open(CONFIG.redis.as_str()).unwrap_or_else(|e| {
        panic!("Failed to connect to redis: {}", e);
    }).get_connection().unwrap_or_else(|e| {
        panic!("Failed to get redis connection: {}", e);
    })
}