use std::collections::HashMap;

use crate::{store::Database, types::RedisValue};

impl Database{
    
    pub fn hset(&mut self, key: &str, field: &str, value: &str) -> usize {
       
        let entry = self
            .store_mut()
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
        match self.store_ref().get(key) {
            Some(RedisValue::Hash(hash)) => hash.get(field).cloned(),
            _ => None,
        }
    }

    pub fn hdel(&mut self, key: &str, fields: &[String]) -> usize {
        if self.is_expired(key) {
            return 0;
        }
        match self.store_mut().get_mut(key) {
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
        match self.store_ref().get(key) {
            Some(RedisValue::Hash(hash)) => hash.keys().cloned().collect(),
            _ => vec![],
        }
    }

    pub fn hvals(&mut self, key: &str) -> Vec<String> {
        if self.is_expired(key) {
            return vec![];
        }
        match self.store_ref().get(key) {
            Some(RedisValue::Hash(hash)) => hash.values().cloned().collect(),
            _ => vec![],
        }
    }

    pub fn hlen(&mut self, key: &str) -> usize {
        if self.is_expired(key) {
            return 0;
        }
        match self.store_ref().get(key) {
            Some(RedisValue::Hash(hash)) => hash.len(),
            _ => 0,
        }
    }

    pub fn hgetall(&mut self, key: &str) -> HashMap<String, String> {
        if self.is_expired(key) {
            return HashMap::new();
        }
        match self.store_ref().get(key) {
            Some(RedisValue::Hash(hash)) => hash.clone(),
            _ => HashMap::new(),
        }
    }

    pub fn hexists(&mut self, key: &str, field: &str) -> bool {
        if self.is_expired(key) {
            return false;
        }
        match self.store_ref().get(key) {
            Some(RedisValue::Hash(hash)) => hash.contains_key(field),
            _ => false,
        }
    }

    

}