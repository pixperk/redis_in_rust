use std::time::{Duration, Instant};

use crate::store::Database;

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
            self.expiry_mut().insert(key.to_string(), Instant::now() + Duration::from_secs(seconds));
            1
        }
    }

    pub fn ttl(&mut self, key: &str) -> isize {
        if self.is_expired(key) || !self.store_ref().contains_key(key) {
            -2  // Redis returns -2 for non-existing key
        } else if let Some(expire_time) = self.expiry_ref().get(key) {
            let dur = expire_time.saturating_duration_since(Instant::now());
            dur.as_secs() as isize
        } else {
            -1  // key exists but no expiry
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