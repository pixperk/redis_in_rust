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

    pub fn lpush(&mut self, key : &str, values : &[String])->usize{
       let entry = self.store.entry(key.to_string())
        .or_insert(RedisValue::List(vec![]));
         if let RedisValue::List(list) = entry{
            for value in values.iter().rev(){
                list.insert(0, value.clone());
            }
            list.len()
         }else{
            0
         }
    }

    pub fn rpush(&mut self, key : &str, values : &[String])->usize{
        let entry = self.store.entry(key.to_string())
        .or_insert(RedisValue::List(vec![]));
         if let RedisValue::List(list) = entry{
            for value in values.iter(){
                list.push(value.clone());
            }
            list.len()
            }else{
                0
            }
         }
    

    pub fn lpop(&mut self, key : &str) -> Option<String>{
        match self.store.get_mut(key){
            Some(RedisValue::List(ref mut list))=>{
                if list.is_empty(){
                    None
                }else{
                    Some(list.remove(0))
                }
            }
            _ => None,
        }
    }

    pub fn rpop(&mut self, key : &str) -> Option<String>{
        match self.store.get_mut(key){
            Some(RedisValue::List(ref mut list))=>{
                list.pop()
            }
            _ => None,
        }
    }

    pub fn llen (&self, key : &str) -> usize{
        match self.store.get(key){
            Some(RedisValue::List(list))=> list.len(),
            _ => 0,
        }
    }

    pub fn lindex(&self, key : &str, index : usize) -> Option<String>{
        match self.store.get(key){
            Some(RedisValue::List(list))=>{
                if index < list.len(){
                    Some(list[index].clone())
                }else{
                    None
                }
            }
            _ => None,
        }
    }

    pub fn lset(&mut self, key : &str, index : usize, value : String) -> Result<(), &'static str>{
        match self.store.get_mut(key){
            Some(RedisValue::List(ref mut list))=>{
                if index < list.len(){
                    list[index] = value;
                    Ok(())
                }else{
                    Err("Index out of range")
                }
            }
            _ => Err("Key does not exist or is not a list"),
        }
    }

    pub fn lrange(&self, key : &str, start : isize, end:isize) -> Vec<String>{
        match self.store.get(key){
            Some(RedisValue::List(list))=>{
                let start = if start < 0 { list.len() as isize + start } else { start };
                let end = if end < 0 { list.len() as isize + end } else { end };
                if start < 0 || end < 0 || start >= list.len() as isize || end >= list.len() as isize {
                    vec![]
                }else{
                    let start = start as usize;
                    let end = end as usize;
                    list[start..=end].to_vec()
                }
            }
            _ => vec![],
        }
    }

    pub fn sadd(&mut self, key : &str, values : &[String]) -> usize{
        let entry = self.store.entry(key.to_string())
        .or_insert(RedisValue::Set(HashSet::new()));
         if let RedisValue::Set(set) = entry{
            for value in values.iter(){
                set.insert(value.clone());
            }
            set.len()
         }else{
            0
         }
    }

    pub fn srem(&mut self, key : &str, values : &[String]) -> usize{
        match self.store.get_mut(key){
            Some(RedisValue::Set(ref mut set))=>{
                let mut removed = 0;
                for value in values.iter(){
                    if set.remove(value){
                        removed += 1;
                    }
                }
                removed
            }
            _ => 0,
        }
    }
    
    pub fn smembers(&self, key : &str) -> Vec<String>{
        match self.store.get(key){
            Some(RedisValue::Set(set))=>{
                set.iter().cloned().collect()
            }
            _ => vec![],
        }
    }

    pub fn sismember(&self, key : &str, value : &str) -> bool{
        match self.store.get(key){
            Some(RedisValue::Set(set))=>{
                set.contains(value)
            }
            _ => false,
        }
    }

    pub fn scard(&self, key : &str) -> usize{
        match self.store.get(key){
            Some(RedisValue::Set(set))=>{
                set.len()
            }
            _ => 0,
        }
    }

    pub fn hset(&mut self, key : &str, field : &str, value : &str) -> usize{
        let entry = self.store.entry(key.to_string())
        .or_insert(RedisValue::Hash(HashMap::new()));
         if let RedisValue::Hash(hash) = entry{
            hash.insert(field.to_string(), value.to_string());
            hash.len()
         }else{
            0
         }
    }

    pub fn hget(&self, key : &str, field : &str) -> Option<String>{
        match self.store.get(key){
            Some(RedisValue::Hash(hash))=>{
                hash.get(field).cloned()
            }
            _ => None,
        }
    }
    pub fn hdel(&mut self, key : &str, fields : &[String]) -> usize{
        match self.store.get_mut(key){
            Some(RedisValue::Hash(ref mut hash))=>{
                let mut removed = 0;
                for field in fields.iter(){
                    if hash.remove(field).is_some(){
                        removed += 1;
                    }
                }
                removed
            }
            _ => 0,
        }
    }

    pub fn hkeys(&self, key : &str) -> Vec<String>{
        match self.store.get(key){
            Some(RedisValue::Hash(hash))=>{
                hash.keys().cloned().collect()
            }
            _ => vec![],
        }
    }

    pub fn hvals(&self, key : &str) -> Vec<String>{
        match self.store.get(key){
            Some(RedisValue::Hash(hash))=>{
                hash.values().cloned().collect()
            }
            _ => vec![],
        }
    }

    pub fn hlen(&self, key : &str) -> usize{
        match self.store.get(key){
            Some(RedisValue::Hash(hash))=>{
                hash.len()
            }
            _ => 0,
        }
    }

    pub fn hgetall(&self, key : &str) -> HashMap<String, String>{
        match self.store.get(key){
            Some(RedisValue::Hash(hash))=>{
                hash.clone()
            }
            _ => HashMap::new(),
        }
    }

    pub fn hexists(&self, key : &str, field : &str) -> bool{
        match self.store.get(key){
            Some(RedisValue::Hash(hash))=>{
                hash.contains_key(field)
            }
            _ => false,
        }
    }


}


