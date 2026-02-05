use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::Mutex};

use crate::pubsub::PubSub;

pub async fn handle_subscribe(
    parts: Vec<&str>,
    writer: Arc<Mutex<OwnedWriteHalf>>,
    pubsub: Arc<PubSub>,
) {
    if parts.len() < 2 {
        let mut w = writer.lock().await;
        let _ = w.write_all(b"-ERR usage: SUBSCRIBE <channel> [channel ...]\r\n").await;
        let _ = w.flush().await;
        return;
    }

    for (i, channel_name) in parts[1..].iter().enumerate() {
        let channel = channel_name.to_string();
        let mut rx = pubsub.subscribe(&channel).await;

        // Send subscription confirmation
        {
            let mut w = writer.lock().await;
            let sub_type = "subscribe";
            let confirmation = format!(
                "*3\r\n${}\r\n{}\r\n${}\r\n{}\r\n:{}\r\n",
                sub_type.len(),
                sub_type,
                channel.len(),
                channel,
                i + 1
            );
            let _ = w.write_all(confirmation.as_bytes()).await;
            let _ = w.flush().await;
        }

        // Spawn listener for published messages
        let writer_clone = Arc::clone(&writer);
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let mut w = writer_clone.lock().await;
                println!("Received message: {} on channel: {}", msg, channel);
                let msg_type = "message";

                let response = format!(
                    "*3\r\n${}\r\n{}\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
                    msg_type.len(),
                    msg_type,
                    channel.len(),
                    channel,
                    msg.len(),
                    msg
                );

                if let Err(e) = w.write_all(response.as_bytes()).await {
                    eprintln!("Failed to write message: {:?}", e);
                    break;
                }
                if let Err(e) = w.flush().await {
                    eprintln!("Failed to flush: {:?}", e);
                    break;
                }
            }
        });
    }
}
