

use crate::{store::Database, utils::current_unix_timestamp};

impl Database{
    pub fn exists(&mut self, keys: &[String]) -> usize {
        keys.iter()
            .filter(|key| {
                self.is_expired(key);
                self.store_ref().contains_key(*key)
            })
            .count()
    }

    // Return only non expired keys.
    pub fn keys(&mut self) -> Vec<String> {
        // Check all keys for expiry first.
        let current_keys: Vec<String> = self.store_ref().keys().cloned().collect();
        for key in current_keys.iter() {
            self.is_expired(key);
        }
        self.store_ref().keys().cloned().collect()
    }

    pub fn expire(&mut self, key: &str, seconds: u64) -> usize {
        if self.is_expired(key) || !self.store_ref().contains_key(key) {
            0
        } else {
            let now = current_unix_timestamp();
            let expire_at = now + seconds;
            self.expiry_mut().insert(key.to_string(), expire_at);
            1
        }
    }
    

    pub fn ttl(&mut self, key: &str) -> isize {
        if self.is_expired(key) || !self.store_ref().contains_key(key) {
            -2  // Key does not exist
        } else if let Some(&expire_at) = self.expiry_ref().get(key) {
            let now = current_unix_timestamp();
            let ttl = expire_at.saturating_sub(now);
            ttl as isize
        } else {
            -1  // Key exists, no expiry
        }
    }
    


    pub fn persist(&mut self, key: &str) -> usize {
    if self.is_expired(key) || !self.store_mut().contains_key(key) {
        0
    } else if self.expiry_mut().remove(key).is_some() {
        1
    } else {
        0
    }
}
}