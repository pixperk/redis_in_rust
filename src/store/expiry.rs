use std::time::Instant;

use super::db::Database;

impl Database{
    
// Checks expiration; if expired, removes the key from both store and expiry.
pub fn is_expired(&mut self, key: &str) -> bool {
    if let Some(&expire_time) = self.expiry_mut().get(key) {
        if Instant::now() >= expire_time {
           self.store_mut().remove(key);
            self.expiry_mut().remove(key);
            return true;
        }
    }
    false
}
}