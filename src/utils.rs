use std::{
    sync::{Arc, Mutex},
    thread
};

use crate::{persistence::Persister, store::Database};

pub fn is_mutating_command(input: &str) -> bool {
    let cmd = input.trim().split_whitespace().next();

    match cmd {
        Some(cmd) => {
            matches!(
                cmd.to_uppercase().as_str(),
                "SET"
                    | "DEL"
                    | "INCR"
                    | "INCRBY"
                    | "DECR"
                    | "DECRBY"
                    | "EXPIRE"
                    | "PERSIST"
                    | "LPOP"
                    | "RPOP"
                    | "LPUSH"
                    | "RPUSH"
                    | "LSET"
                    | "SADD"
                    | "SREM"
                    | "HSET"
                    | "HDEL"
                    | "FLUSHDB"
            )
        }
        None => false,
    }
}

pub fn current_unix_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn start_expiry_worker(db: Arc<Mutex<Database>>, persister: Arc<dyn Persister + Send + Sync>) {
    thread::spawn(move || loop {
        let mut db = db.lock().unwrap();
        db.remove_expired_keys(&*persister);
    });
}
