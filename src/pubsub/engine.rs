use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, unbounded_channel, UnboundedSender},
    Mutex,
};

type Subscriber = UnboundedSender<String>;

#[derive(Debug, Default)]
pub struct PubSub {
    channels: Mutex<HashMap<String, Vec<Subscriber>>>,
}

impl PubSub {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn subscribe(&self, channel: &str) -> mpsc::UnboundedReceiver<String> {
        let (tx, rx) = unbounded_channel();
        let mut channels = self.channels.lock().await;
        channels.entry(channel.to_string()).or_default().push(tx);
        rx
    }

    pub async fn publish(&self, channel: &str, message: String) {
        let mut channels = self.channels.lock().await;

        if let Some(subscribers) = channels.get_mut(channel) {
            // Retain only alive subscribers
            subscribers.retain(|subscriber| subscriber.send(message.clone()).is_ok());
        }
    }
}
