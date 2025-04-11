use std::{fs, path::Path};

use crate::store::Database;
use serde::{ Deserialize, Serialize};

use super::persister::Persister;


#[derive(Serialize, Deserialize)]
pub struct  JsonPersister{
   path : String,
}

impl JsonPersister{
    pub fn new(path : &str) -> Self{
        JsonPersister{
            path : path.to_string(),
        }
    }
}

impl Persister for JsonPersister{
    fn load(&self) -> Option<Database> {
        if !Path::new(&self.path).exists(){
            return None;
        }

        let data = fs::read_to_string(&self.path).ok()?;
        serde_json::from_str(&data).ok()
    }

    fn save(&self, db: &Database) {
        if let Ok(data) = serde_json::to_string_pretty(db){
            let _ = fs::write(&self.path, data);
        }
    }
}