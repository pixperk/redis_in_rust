use std::collections::HashMap;


pub struct Database{
    store : HashMap<String, String>, //key : value
}

impl Database {
    pub fn new() -> Self {
        Database { store: HashMap::new() }
    }

    pub fn set(&mut self, key : String, value : String){
        self.store.insert(key, value);
    }

    pub fn get(&self, key : &str) -> Option<&String> {
        self.store.get(key)
    }
}