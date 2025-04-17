use std::sync::Arc;
use tokio::sync::Mutex;



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
    tokio::spawn(async move {
        loop {
            {
                let mut db = db.lock().await;
                db.remove_expired_keys(&*persister);
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
}
