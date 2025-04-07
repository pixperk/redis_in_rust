use std::collections::HashMap;

pub struct Database {
    store: HashMap<String, String>, //key : value
}

impl Database {
    pub fn new() -> Self {
        Database {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
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
        let val = self.store.entry(key.to_string()).or_insert("0".to_string());
        match val.parse::<i64>() {
            Ok(num) => {
                let new_num = num + 1;
                *val = new_num.to_string();
                Ok(new_num)
            }
            Err(_) => Err("Value is not a number"),
        }
    }
    pub fn incr_by(&mut self, key: &str, by: i64) -> Result<i64, &'static str> {
        let val = self.store.entry(key.to_string()).or_insert("0".to_string());
        match val.parse::<i64>() {
            Ok(num) => {
                *val = (num + by).to_string();
                Ok(num + by)
            }
            Err(_) => Err("value is not an integer"),
        }
    }
    pub fn keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    pub fn flushdb(&mut self) {
        self.store.clear();
    }
}
