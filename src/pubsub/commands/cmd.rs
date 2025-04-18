use std:: sync::Arc;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::Mutex};

use crate::pubsub::PubSub;

pub async fn handle_subscribe(
    parts: Vec<&str>,
    writer: Arc<Mutex<OwnedWriteHalf>>,
    pubsub: Arc<PubSub>,
) {
    if parts.len() != 2 {
        let mut w = writer.lock().await;
        let _ = w.write_all(b"-ERR usage: SUBSCRIBE <channel>\r\n").await;
        let _ = w.flush().await;
        return;
    }

    let channel = parts[1].to_string();
    let mut rx = pubsub.subscribe(&channel).await;

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

pub async fn handle_publish(
    parts: Vec<&str>,
    pubsub: Arc<PubSub>,
)  {
    

    let channel = parts[1];
    let message = parts[2..].join(" ");
    pubsub.publish(channel, message).await;

    
}
