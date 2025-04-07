use std::collections::{HashMap, HashSet};

pub enum RedisValue{
    String(String),
    List(Vec<String>),
    Set(HashSet<String>),
    Hash(HashMap<String, String>),
}

pub struct Database {
    store: HashMap<String, RedisValue>, //key : value
}

impl Database {
    pub fn new() -> Self {
        Database {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, RedisValue::String(value));
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.store.get(key) {
            Some(RedisValue::String(value)) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn delete(&mut self, keys: &[String]) -> usize {
        let mut removed = 0;
        for key in keys {
            if self.store.remove(key).is_some() {
                removed += 1;
            }
        }
        removed
    }

    pub fn exists(&self, keys: &[String]) -> usize {
        keys.iter()
            .filter(|key| self.store.contains_key(*key))
            .count()
    }

    pub fn incr(&mut self, key: &str) -> Result<i64, &'static str> {
        self.incr_by(key, 1)
    }
    pub fn incr_by(&mut self, key: &str, by: i64) -> Result<i64, &'static str> {
        let val  = self.store.entry(key.to_string()).or_insert( RedisValue::String("0".to_string()));
        match val{
            RedisValue::String(ref mut s) => {
                let current_value: i64 = s.parse().map_err(|_| "Value is not an integer")?;
                let new_value = current_value + by;
                *s = new_value.to_string();
                Ok(new_value)
            }
            _ => Err("Value is not an integer"),
        }
    }
    pub fn keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    pub fn flushdb(&mut self) {
        self.store.clear();
    }
}
