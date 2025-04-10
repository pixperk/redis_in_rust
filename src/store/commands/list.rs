use crate::{store::Database, types::RedisValue};

impl Database{
    pub fn lpush(&mut self, key: &str, values: &[String]) -> usize {
        self.is_expired(key);
        let entry = self
            .store_mut()
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
            .store_mut()
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
        match self.store_mut().get_mut(key) {
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
        match self.store_mut().get_mut(key) {
            Some(RedisValue::List(ref mut list)) => list.pop(),
            _ => None,
        }
    }

    pub fn llen(&mut self, key: &str) -> usize {
        if self.is_expired(key) {
            return 0;
        }
        match self.store_mut().get(key) {
            Some(RedisValue::List(list)) => list.len(),
            _ => 0,
        }
    }

    pub fn lindex(&mut self, key: &str, index: usize) -> Option<String> {
        if self.is_expired(key) {
            return None;
        }
        match self.store_ref().get(key) {
            Some(RedisValue::List(list)) => list.get(index).cloned(),
            _ => None,
        }
    }

    pub fn lset(&mut self, key: &str, index: usize, value: String) -> Result<(), &'static str> {
        if self.is_expired(key) {
            return Err("Key does not exist");
        }
        match self.store_mut().get_mut(key) {
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
        match self.store_ref().get(key) {
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
}