use std::error::Error;

use crate::store::Database;

pub trait Persister {
    fn load(&self) -> Option<Database>;
    fn save(&self, db: &Database) -> Result<(), Box<dyn Error>>;
}
