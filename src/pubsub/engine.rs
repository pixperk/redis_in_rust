use std::{collections::HashMap, sync::Arc};

use tokio::sync::{mpsc::{self, unbounded_channel}, Mutex};


type Subscriber = mpsc::UnboundedSender<String>;


#[derive(Debug, Default)]
pub struct PubSub {
   channels : Mutex<HashMap<String, Vec<Subscriber>>>,
}

impl PubSub{
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn subscribe(&self, channel : &str) -> mpsc::UnboundedReceiver<String> {
        let (tx, rx) = unbounded_channel();
        let mut channels = self.channels.lock().await;
        channels.entry(channel.to_string()).or_default().push(tx);
        rx
    }

    pub async fn publish(&self, channel: &str, message: String) {
        let channels = self.channels.lock().await;
        if let Some(subscribers) = channels.get(channel) {
            for subscriber in subscribers {
                let _ = subscriber.send(message.clone());
            }
        }
    }
}