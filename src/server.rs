use std::{net::TcpListener, sync::{Arc, Mutex}};
use crate::{client, store::db::Database };

pub fn run(addr: &str, db: Arc<Mutex<Database>>) {
    let listener = TcpListener::bind(addr)
        .expect("Failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("🔗 Accepted new connection");

                let db = Arc::clone(&db);
                std::thread::spawn(move || {
                    client::handle_connection(stream, db);
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {e}"),
        }
    }
}
