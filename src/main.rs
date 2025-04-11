use std::sync::Arc;

mod server;
mod store;
mod resp;
mod types;
mod client;
mod persistence;
mod utils;

use crate::persistence::JsonPersister;


const REDIS_PORT: &str = "127.0.0.1:6379";

fn main() {
    println!("🚀 Redis (Rust Edition) listening on {REDIS_PORT}");

    let persister = Arc::new(JsonPersister::new("db.json"));
   

    server::run(REDIS_PORT, persister);
}
