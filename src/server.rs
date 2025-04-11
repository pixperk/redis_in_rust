use std::{net::TcpListener, sync::{Arc, Mutex}};
use crate::{client, persistence::Persister, store::db::Database };

pub fn run(addr: &str, persister: Arc<dyn Persister + Send + Sync>) {
    let listener = TcpListener::bind(addr)
        .expect("Failed to bind to address");

    let db = match persister.load() {
        Some(db) => {
            println!("🔄 Loaded database from file");
            db
        },
        None => {
            println!("🗄️  No database file found, starting with an empty database");
            Database::new()
        }
    };

    let db = Arc::new(Mutex::new(db));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("🔗 Accepted new connection");

                let db = Arc::clone(&db);
                let persister = Arc::clone(&persister);

                std::thread::spawn(move || {
                    client::handle_connection(stream, db, persister);
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {e}"),
        }
    }
}
