use std::{collections::HashMap, sync::{mpsc::Sender, Mutex}};
use uuid::Uuid;

pub struct PubSub{
    channels : Mutex<HashMap<String, Vec<(usize, Sender<String>)>>>,
    next_id : Mutex<usize>,
}

impl PubSub {
    pub fn new() -> Self{
        PubSub {
            channels: Mutex::new(HashMap::new()),
            next_id: Mutex::new(0),
        }
    }

    pub fn subscribe(&self, channel : &str,  sender : Sender<String>) -> usize{
        let mut channels = self.channels.lock().unwrap();
        let mut id_gen = self.next_id.lock().unwrap();

        let id = *id_gen;
        *id_gen += 1; //Increment the id for the next subscriber
        
        channels.entry(channel.to_string())
            .or_default()
            .push((id, sender));

        id
    }

    pub fn publish(&self, channel : &str, message : String) -> usize{
        let mut channels = self.channels.lock().unwrap();
        let mut failed = 0;
        if let Some(subscribers) = channels.get(channel){
            for (_, subscriber) in subscribers {
                if let Err(err) = subscriber.send(message.clone()) {
                    failed += 1;
                    println!("Failed to send message: {}", err);
                }
            }
            subscribers.len() - failed
        } else {
            0
        }
        }

        pub fn unsubscribe (&self, channel : &str, sender_id : usize){
            let mut channels = self.channels.lock().unwrap();
            if let Some(subscribers) = channels.get_mut(channel){
                subscribers.retain(|(id, _)| *id != sender_id);
                if subscribers.is_empty(){
                    channels.remove(channel);
                }
            }
        }

    }
    


