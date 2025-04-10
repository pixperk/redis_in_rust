use std::collections::HashSet;

use crate::{store::Database, types::RedisValue};

impl Database{
    

    pub fn sadd(&mut self, key: &str, values: &[String]) -> usize {
        
        let entry = self
            .store_mut()
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
        match self.store_mut().get_mut(key) {
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
        match self.store_ref().get(key) {
            Some(RedisValue::Set(set)) => set.iter().cloned().collect(),
            _ => vec![],
        }
    }

    pub fn sismember(&mut self, key: &str, value: &str) -> bool {
        if self.is_expired(key) {
            return false;
        }
        match self.store_ref().get(key) {
            Some(RedisValue::Set(set)) => set.contains(value),
            _ => false,
        }
    }

    pub fn scard(&mut self, key: &str) -> usize {
        if self.is_expired(key) {
            return 0;
        }
        match self.store_ref().get(key) {
            Some(RedisValue::Set(set)) => set.len(),
            _ => 0,
        }
    }

}