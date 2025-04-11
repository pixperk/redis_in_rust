use crate::store::Database;

pub trait Persister{
    fn load(&self) -> Option<Database>;
    fn save(&self, db: &Database);
}