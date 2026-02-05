

use crate::{persistence::Persister, utils::current_unix_timestamp};

use super::db::Database;

impl Database{
    
// Checks expiration; if expired, removes the key from both store and expiry.
pub fn is_expired(&mut self, key: &str) -> bool {
    if let Some(&expire_time) = self.expiry_mut().get(key) {
        if current_unix_timestamp() >= expire_time {
           self.store_mut().remove(key);
            self.expiry_mut().remove(key);
            return true;
        }
    }
    false
}

pub fn remove_expired_keys(&mut self, persister : &dyn Persister){
    let now = current_unix_timestamp();

    let expired_keys : Vec<String> = self.expiry_ref()
    .iter()
    .filter(|(_, &exp)| exp <= now)
    .map(|(key, _)| key.clone())
    .collect();

    if expired_keys.is_empty() {
        return;
    }

    for key in &expired_keys {
        self.store_mut().remove(key);
        self.expiry_mut().remove(key);
        println!("Key expired, thus removed: {}", key);
    }

    if let Err(e) = persister.save(self) {
        eprintln!("Failed to persist DB after expiry cleanup: {e}");
    }
}
}