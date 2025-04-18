use crate::pubsub::PubSub;
use crate::{
    persistence::Persister, resp::handler::handle_command, store::db::Database,
    utils::is_mutating_command,
};

use std::sync::Arc;
use tokio::io::BufReader;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

pub async fn handle_connection(
    stream: TcpStream,
    db: Arc<Mutex<Database>>,
    persister: Arc<dyn Persister + Send + Sync>,
    pubsub: Arc<PubSub>,
) {
    let (reader, writer) = stream.into_split();
    let writer = Arc::new(Mutex::new(writer));
    let mut reader = BufReader::new(reader);
    let mut buf = [0; 512];

    loop {
        let bytes_read = match reader.read(&mut buf).await {
            Ok(0) => {
                println!("⚠️  Client disconnected");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Read error: {e}");
                break;
            }
        };

        let input = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("📥 Received:\n{input}");

        let command_name = input
            .lines()
            .find(|line| line.starts_with('$'))
            .map(|_| input.lines().nth(2).unwrap_or(""))
            .unwrap_or("")
            .to_uppercase();

        /* if command_name == "SUBSCRIBE"{
            continue;
        } */

        let mut db = db.lock().await;

        let response = handle_command(
            &input,
            &mut db,
            Arc::clone(&pubsub),
            Arc::clone(&writer),
        )
        .await;

       

       

        // Save to disk if mutating
        if is_mutating_command(&command_name) {
            if let Err(e) = persister.save(&db) {
                eprintln!("❌ Failed to save database: {e}");
            } else {
                println!("💾 Database saved to disk");
            }
        }

        let mut s = writer.lock().await;
        if let Err(e) = s.write_all(response.as_bytes()).await {
            eprintln!("❌ Write error: {e}");
            break;
        }
    }
}
