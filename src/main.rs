use std::sync::Arc;

mod server;
mod store;
mod resp;
mod types;
mod client;
mod persistence;
mod utils;
mod pubsub;

use crate::persistence::JsonPersister;

#[tokio::main]
async fn main(){
    const REDIS_PORT: &str = "127.0.0.1:6379";


    println!("🚀 Redis (Rust Edition) listening on {REDIS_PORT}");

    let persister = Arc::new(JsonPersister::new("db.json"));
   
    server::run(REDIS_PORT, persister).await;
}
