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

    pub async fn subscribe(&self, channel: &str) -> mpsc::UnboundedReceiver<String>
        {
            let (tx, rx) = unbounded_channel();
            let mut channels = self.channels.lock().await;
            channels.entry(channel.to_string()).or_default().push(tx);
            rx
        }


    pub async fn publish(&self, channel: &str, message: String) -> usize {
        let mut channels = self.channels.lock().await;
        let mut delivered = 0;

        if let Some(subscribers) = channels.get_mut(channel) {
            subscribers.retain(|subscriber| {
                if subscriber.is_closed() {
                    false
                } else {
                    match subscriber.send(message.clone()) {
                        Ok(_) => {
                            delivered += 1;
                            true
                        }
                        Err(_) => false,
                    }
                }
            });
        }

        println!(
            "{{ \"channel\": \"{}\", \"message\": \"{}\", \"subscribers\": {} }}",
            channel, message, delivered
        );
        delivered
    }

   
}
