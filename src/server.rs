use crate::pubsub::PubSub;
use crate::{client, persistence::Persister, store::db::Database, utils::start_expiry_worker};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub async fn run(addr: &str, persister: Arc<dyn Persister + Send + Sync>, pubsub: Arc<PubSub>) {
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

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

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("🔗 Accepted new connection");

                let db = Arc::clone(&db);
                let persister = Arc::clone(&persister);
                let pubsub = Arc::clone(&pubsub);

                tokio::spawn(async move {
                    client::handle_connection(stream, db, persister, pubsub).await;
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {e}"),
        }
    }
}
