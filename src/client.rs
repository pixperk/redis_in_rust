use crate::pubsub::PubSub;
use crate::{
    persistence::Persister, resp::handler::handle_command, store::db::Database,
    utils::is_mutating_command,
};

use std::sync::Arc;
use tokio::io::BufReader;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
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

    loop {
        let mut first_line = String::new();
        match reader.read_line(&mut first_line).await {
            Ok(0) => {
                println!("⚠️  Client disconnected");
                break;
            }
            Err(e) => {
                eprintln!("Read error: {e}");
                break;
            }
            _ => {}
        }

        let first_line = first_line.trim().to_string();
        if first_line.is_empty() {
            continue;
        }

        let parts = if first_line.starts_with('*') {
            // RESP protocol: *N\r\n followed by N bulk strings
            let count: usize = match first_line[1..].parse() {
                Ok(n) => n,
                Err(_) => {
                    let mut w = writer.lock().await;
                    let _ = w.write_all(b"-ERR invalid RESP array\r\n").await;
                    let _ = w.flush().await;
                    continue;
                }
            };

            let mut parts = Vec::with_capacity(count);
            let mut valid = true;
            for _ in 0..count {
                // Read $N line
                let mut len_line = String::new();
                if reader.read_line(&mut len_line).await.unwrap_or(0) == 0 {
                    valid = false;
                    break;
                }
                // Read the value line
                let mut value_line = String::new();
                if reader.read_line(&mut value_line).await.unwrap_or(0) == 0 {
                    valid = false;
                    break;
                }
                parts.push(
                    value_line
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string(),
                );
            }

            if !valid {
                println!("⚠️  Client disconnected mid-command");
                break;
            }
            parts
        } else {
            // Inline command
            first_line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        };

        if parts.is_empty() {
            continue;
        }

        let command_name = parts[0].to_uppercase();
        println!("📥 Command: {}", command_name);

        // Lock db, process command, persist, then release lock before writing
        let response = {
            let mut db = db.lock().await;

            let response = handle_command(
                &parts,
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

            response
        }; // db lock dropped here

        if command_name != "SUBSCRIBE" {
            let mut s = writer.lock().await;
            if let Err(e) = s.write_all(response.as_bytes()).await {
                eprintln!("❌ Write error: {e}");
                break;
            }
            if let Err(e) = s.flush().await {
                eprintln!("❌ Flush error: {e}");
                break;
            }
        }
    }
}
