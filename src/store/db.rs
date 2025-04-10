use std::{
    collections::HashMap,
    time::Instant,
};

use crate::types::RedisValue;



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

    pub fn store_ref(&self) -> &HashMap<String, RedisValue> {
        &self.store
    }

    pub fn expiry_ref(&self) -> &HashMap<String, Instant> {
        &self.expiry
    }

    pub fn store_mut(&mut self) -> &mut HashMap<String, RedisValue> {
        &mut self.store
    }

    pub fn expiry_mut(&mut self) -> &mut HashMap<String, Instant> {
        &mut self.expiry
    }


    pub fn flushdb(&mut self) {
        self.store.clear();
        self.expiry.clear();
    }

    
}
