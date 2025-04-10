#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use store::db::Database;
use resp::resp::handle_command;

mod resp;
mod store;
mod types;





fn main() {
    println!("Redis (Rust Edition) starting on port 6379");

    const REDIS_PORT: &str = "127.0.0.1:6379";
    let listener = TcpListener::bind(REDIS_PORT).unwrap();
    let database = Arc::new(Mutex::new(Database::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection!");

                let db = Arc::clone(&database);

                thread::spawn(move || {
                    let mut buf = [0; 512];

                    loop {
                        let bytes_read = match stream.read(&mut buf) {
                            Ok(0) => {
                                println!("Connection closed by client.");
                                break;
                            }
                            Ok(n) => n,
                            Err(e) => {
                                println!("Error reading from stream: {}", e);
                                break;
                            }
                        };

                        let input = String::from_utf8_lossy(&buf[..bytes_read]);
                        println!("Received: {}", input);

                        let mut db = match db.lock() {
                            Ok(db_guard) => db_guard,
                            Err(e) => {
                                println!("Failed to acquire database lock: {:?}", e);
                                break;
                            }
                        };
                        let response = handle_command(&input, &mut db);

                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            println!("Failed to write to client: {:?}", e);
                            break;
                        }
                    }
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
