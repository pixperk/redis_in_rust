use crate::{store::Database, types::RedisValue};

impl Database{
    pub fn incr(&mut self, key: &str) -> Result<i64, &'static str> {
        self.incr_by(key, 1)
    }
    pub fn incr_by(&mut self, key: &str, by: i64) -> Result<i64, &'static str> {
        // Even for incr, check expiry first.
        self.is_expired(key);
        let val = self.store_mut().entry(key.to_string()).or_insert(RedisValue::String("0".to_string()));
        match val {
            RedisValue::String(ref mut s) => {
                let current_value: i64 = s.parse().map_err(|_| "Value is not an integer")?;
                let new_value = current_value + by;
                *s = new_value.to_string();
                Ok(new_value)
            }
            _ => Err("Value is not an integer"),
        }
    }

    
}