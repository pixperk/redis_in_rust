use crate::{client, persistence::Persister, store::db::Database, utils::start_expiry_worker};
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};

pub fn run(addr: &str, persister: Arc<dyn Persister + Send + Sync>) {
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");

    let db = match persister.load() {
        Some(db) => {
            println!("🔄 Loaded database from file");
            db
        }
        None => {
            println!("🗄️  No database file found, starting with an empty database");
            Database::new()
        }
    };

    let db = Arc::new(Mutex::new(db));

    let db_worker = Arc::clone(&db);
    let persister_worker = Arc::clone(&persister);

    start_expiry_worker(db_worker, persister_worker);

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
