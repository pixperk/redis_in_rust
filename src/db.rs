use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

pub enum RedisValue {
    String(String),
    List(Vec<String>),
    Set(HashSet<String>),
    Hash(HashMap<String, String>),
}

pub struct Database {
    store: HashMap<String, RedisValue>, // key: value
    expiry: HashMap<String, Instant>,     // key: expiry time
}

impl Database {
    pub fn new() -> Self {
        Database {
            store: HashMap::new(),
            expiry: HashMap::new(),
        }
    }

    // Checks expiration; if expired, removes the key from both store and expiry.
    fn is_expired(&mut self, key: &str) -> bool {
        if let Some(&expire_time) = self.expiry.get(key) {
            if Instant::now() >= expire_time {
                self.store.remove(key);
                self.expiry.remove(key);
                return true;
            }
        }
        false
    }

    pub fn set(&mut self, key: &str, value: String, ttl: Option<u64>) {
        self.store
            .insert(key.to_string(), RedisValue::String(value));
        if let Some(seconds) = ttl {
            self.expiry
                .insert(key.to_string(), Instant::now() + Duration::from_secs(seconds));
        } else {
            self.expiry.remove(key);
        }
    }

    // Now returns None if key is expired.
    pub fn get(&mut self, key: &str) -> Option<String> {
        if self.is_expired(key) {
            return None;
        }
        match self.store.get(key) {
            Some(RedisValue::String(value)) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn delete(&mut self, keys: &[String]) -> usize {
        let mut removed = 0;
        for key in keys {
            self.is_expired(key);
            if self.store.remove(key).is_some() {
                removed += 1;
            }
        }
        removed
    }

    pub fn exists(&mut self, keys: &[String]) -> usize {
        keys.iter()
            .filter(|key| {
                self.is_expired(key);
                self.store.contains_key(*key)
            })
            .count()
    }

    pub fn incr(&mut self, key: &str) -> Result<i64, &'static str> {
        self.incr_by(key, 1)
    }
    pub fn incr_by(&mut self, key: &str, by: i64) -> Result<i64, &'static str> {
        // Even for incr, check expiry first.
        self.is_expired(key);
        let val = self.store.entry(key.to_string()).or_insert(RedisValue::String("0".to_string()));
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

    // Return only non expired keys.
    pub fn keys(&mut self) -> Vec<String> {
        // Check all keys for expiry first.
        let current_keys: Vec<String> = self.store.keys().cloned().collect();
        for key in current_keys.iter() {
            self.is_expired(key);
        }
        self.store.keys().cloned().collect()
    }

    pub fn flushdb(&mut self) {
        self.store.clear();
        self.expiry.clear();
    }

    pub fn lpush(&mut self, key: &str, values: &[String]) -> usize {
        self.is_expired(key);
        let entry = self
            .store
            .entry(key.to_string())
            .or_insert(RedisValue::List(vec![]));
        if let RedisValue::List(list) = entry {
            for value in values.iter().rev() {
                list.insert(0, value.clone());
            }
            list.len()
        } else {
            0
        }
    }

    pub fn rpush(&mut self, key: &str, values: &[String]) -> usize {
        self.is_expired(key);
        let entry = self
            .store
            .entry(key.to_string())
            .or_insert(RedisValue::List(vec![]));
        if let RedisValue::List(list) = entry {
            for value in values.iter() {
                list.push(value.clone());
            }
            list.len()
        } else {
            0
        }
    }

    // Returns None if the key is expired.
    pub fn lpop(&mut self, key: &str) -> Option<String> {
        if self.is_expired(key) {
            return None;
        }
        match self.store.get_mut(key) {
            Some(RedisValue::List(ref mut list)) => {
                if list.is_empty() {
                    None
                } else {
                    Some(list.remove(0))
                }
            }
            _ => None,
        }
    }

    // Returns None if the key is expired.
    pub fn rpop(&mut self, key: &str) -> Option<String> {
        if self.is_expired(key) {
            return None;
        }
        match self.store.get_mut(key) {
            Some(RedisValue::List(ref mut list)) => list.pop(),
            _ => None,
        }
    }

    pub fn llen(&mut self, key: &str) -> usize {
        if self.is_expired(key) {
            return 0;
        }
        match self.store.get(key) {
            Some(RedisValue::List(list)) => list.len(),
            _ => 0,
        }
    }

    pub fn lindex(&mut self, key: &str, index: usize) -> Option<String> {
        if self.is_expired(key) {
            return None;
        }
        match self.store.get(key) {
            Some(RedisValue::List(list)) => list.get(index).cloned(),
            _ => None,
        }
    }

    pub fn lset(&mut self, key: &str, index: usize, value: String) -> Result<(), &'static str> {
        if self.is_expired(key) {
            return Err("Key does not exist");
        }
        match self.store.get_mut(key) {
            Some(RedisValue::List(ref mut list)) => {
                if index < list.len() {
                    list[index] = value;
                    Ok(())
                } else {
                    Err("Index out of range")
                }
            }
            _ => Err("Key does not exist or is not a list"),
        }
    }

    pub fn lrange(&mut self, key: &str, start: isize, end: isize) -> Vec<String> {
        if self.is_expired(key) {
            return vec![];
        }
        match self.store.get(key) {
            Some(RedisValue::List(list)) => {
                let len = list.len() as isize;
                let start = if start < 0 { len + start } else { start };
                let end = if end < 0 { len + end } else { end };
                if start < 0 || end < 0 || start >= len || end >= len || start > end {
                    vec![]
                } else {
                    list[start as usize..=end as usize].to_vec()
                }
            }
            _ => vec![],
        }
    }

    pub fn sadd(&mut self, key: &str, values: &[String]) -> usize {
        
        let entry = self
            .store
            .entry(key.to_string())
            .or_insert(RedisValue::Set(HashSet::new()));
        if let RedisValue::Set(set) = entry {
            for value in values.iter() {
                set.insert(value.clone());
            }
            set.len()
        } else {
            0
        }
    }

    pub fn srem(&mut self, key: &str, values: &[String]) -> usize {
        if self.is_expired(key) {
            return 0;
        }
        match self.store.get_mut(key) {
            Some(RedisValue::Set(ref mut set)) => {
                let mut removed = 0;
                for value in values.iter() {
                    if set.remove(value) {
                        removed += 1;
                    }
                }
                removed
            }
            _ => 0,
        }
    }

    pub fn smembers(&mut self, key: &str) -> Vec<String> {
        if self.is_expired(key) {
            return vec![];
        }
        match self.store.get(key) {
            Some(RedisValue::Set(set)) => set.iter().cloned().collect(),
            _ => vec![],
        }
    }

    pub fn sismember(&mut self, key: &str, value: &str) -> bool {
        if self.is_expired(key) {
            return false;
        }
        match self.store.get(key) {
            Some(RedisValue::Set(set)) => set.contains(value),
            _ => false,
        }
    }

    pub fn scard(&mut self, key: &str) -> usize {
        if self.is_expired(key) {
            return 0;
        }
        match self.store.get(key) {
            Some(RedisValue::Set(set)) => set.len(),
            _ => 0,
        }
    }

    pub fn hset(&mut self, key: &str, field: &str, value: &str) -> usize {
       
        let entry = self
            .store
            .entry(key.to_string())
            .or_insert(RedisValue::Hash(HashMap::new()));
        if let RedisValue::Hash(hash) = entry {
            hash.insert(field.to_string(), value.to_string());
            hash.len()
        } else {
            0
        }
    }

    pub fn hget(&mut self, key: &str, field: &str) -> Option<String> {
        if self.is_expired(key) {
            return None;
        }
        match self.store.get(key) {
            Some(RedisValue::Hash(hash)) => hash.get(field).cloned(),
            _ => None,
        }
    }

    pub fn hdel(&mut self, key: &str, fields: &[String]) -> usize {
        if self.is_expired(key) {
            return 0;
        }
        match self.store.get_mut(key) {
            Some(RedisValue::Hash(ref mut hash)) => {
                let mut removed = 0;
                for field in fields.iter() {
                    if hash.remove(field).is_some() {
                        removed += 1;
                    }
                }
                removed
            }
            _ => 0,
        }
    }

    pub fn hkeys(&mut self, key: &str) -> Vec<String> {
        if self.is_expired(key) {
            return vec![];
        }
        match self.store.get(key) {
            Some(RedisValue::Hash(hash)) => hash.keys().cloned().collect(),
            _ => vec![],
        }
    }

    pub fn hvals(&mut self, key: &str) -> Vec<String> {
        if self.is_expired(key) {
            return vec![];
        }
        match self.store.get(key) {
            Some(RedisValue::Hash(hash)) => hash.values().cloned().collect(),
            _ => vec![],
        }
    }

    pub fn hlen(&mut self, key: &str) -> usize {
        if self.is_expired(key) {
            return 0;
        }
        match self.store.get(key) {
            Some(RedisValue::Hash(hash)) => hash.len(),
            _ => 0,
        }
    }

    pub fn hgetall(&mut self, key: &str) -> HashMap<String, String> {
        if self.is_expired(key) {
            return HashMap::new();
        }
        match self.store.get(key) {
            Some(RedisValue::Hash(hash)) => hash.clone(),
            _ => HashMap::new(),
        }
    }

    pub fn hexists(&mut self, key: &str, field: &str) -> bool {
        if self.is_expired(key) {
            return false;
        }
        match self.store.get(key) {
            Some(RedisValue::Hash(hash)) => hash.contains_key(field),
            _ => false,
        }
    }

    pub fn expire(&mut self, key: &str, seconds: u64) -> usize {
        if self.is_expired(key) || !self.store.contains_key(key) {
            0
        } else {
            self.expiry.insert(key.to_string(), Instant::now() + Duration::from_secs(seconds));
            1
        }
    }

    pub fn ttl(&mut self, key: &str) -> isize {
        if self.is_expired(key) || !self.store.contains_key(key) {
            -2  // Redis returns -2 for non-existing key
        } else if let Some(expire_time) = self.expiry.get(key) {
            let dur = expire_time.saturating_duration_since(Instant::now());
            dur.as_secs() as isize
        } else {
            -1  // key exists but no expiry
        }
    }


    pub fn persist(&mut self, key: &str) -> usize {
    if self.is_expired(key) || !self.store.contains_key(key) {
        0
    } else if self.expiry.remove(key).is_some() {
        1
    } else {
        0
    }
}

}
