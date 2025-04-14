

use crate::{store::Database, types::RedisValue, utils::current_unix_timestamp};

impl Database{
    pub fn set(&mut self, key: &str, value: String, ttl: Option<u64>) {
        self.store_mut()
            .insert(key.to_string(), RedisValue::String(value));
    
        if let Some(seconds) = ttl {
            let expire_at = current_unix_timestamp() + seconds;
            self.expiry_mut().insert(key.to_string(), expire_at);
        } else {
            self.expiry_mut().remove(key);
        }
    }
    

    // Now returns None if key is expired.
    pub fn get(&mut self, key: &str) -> Option<String> {
        if self.is_expired(key) {
            return None;
        }
        match self.store_ref().get(key) {
            Some(RedisValue::String(value)) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn delete(&mut self, keys: &[String]) -> usize {
        let mut removed = 0;
        for key in keys {
            self.is_expired(key);
            if self.store_mut().remove(key).is_some() {
                removed += 1;
            }
        }
        removed
    }
}