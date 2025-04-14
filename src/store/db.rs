use std:: collections::HashMap;


use serde::{Deserialize, Serialize};

use crate::types::RedisValue;


#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    store: HashMap<String, RedisValue>, // key: value
    expiry: HashMap<String, u64>,     // key: expiry time
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

    pub fn expiry_ref(&self) -> &HashMap<String, u64> {
        &self.expiry
    }

    pub fn store_mut(&mut self) -> &mut HashMap<String, RedisValue> {
        &mut self.store
    }

    pub fn expiry_mut(&mut self) -> &mut HashMap<String, u64> {
        &mut self.expiry
    }


    pub fn flushdb(&mut self) {
        self.store.clear();
        self.expiry.clear();
    }

    
}
