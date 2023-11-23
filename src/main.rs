use redis::{Commands, Connection};
mod create_connection;
mod utils;
use create_connection::get_connection;

mod main_test;
fn main() {
    let mut conn: Connection = get_connection();

    let _ : () = conn.set("bartick", 42).unwrap_or_else(|e| {
        panic!("Failed to set key: {}", e);
    });

    let value: Option<i128> = conn.get("bartick").unwrap_or_else(|e| {
        panic!("Failed to get key: {}", e);
    });

    println!("Value: {}", value.unwrap());
}
