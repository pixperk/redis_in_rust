mod server;
mod store;
mod resp;
mod types;
mod client;
mod persistence;

use store::db::Database;
use std::sync::{Arc, Mutex};

const REDIS_PORT: &str = "127.0.0.1:6379";

fn main() {
    println!("🚀 Redis (Rust Edition) listening on {REDIS_PORT}");

    let db = Arc::new(Mutex::new(Database::new()));

    server::run(REDIS_PORT, db);
}
