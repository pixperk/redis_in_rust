use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::store::Database;

use super::persister::Persister;


#[derive(Serialize, Deserialize, Debug)]
pub struct JsonPersister {
    path: String,
}  

impl JsonPersister{
    pub fn new(path : &str) -> Self{
        Self { path: path.to_string() }
    }
}

impl Persister for JsonPersister{
    fn load(&self) -> Option<Database>{
        if !Path::new(&self.path).exists(){
            return None;
        }

        let data = fs::read_to_string(&self.path).ok()?;
        match serde_json::from_str(&data) {
            Ok(db) => Some(db),
            Err(e) => {
                eprintln!("Failed to parse db.json: {e}");
                None
            }
        }
    }

    fn save(&self, db:&Database) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(db)?;
        fs::write(&self.path, data)?;

        Ok(())
        
    }


}