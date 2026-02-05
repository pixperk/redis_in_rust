use crate::pubsub::{cmd::handle_subscribe, PubSub};
use crate::store::db::Database;
use std::sync::Arc;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

use super::commands::{
    hash_set::handle_hash_set,
    key::handle_key,
    list::handle_list,
    number::handle_number,
    set::handle_set,
    string::handle_string,
};

pub async fn handle_command(
    parts: &[String],
    db: &mut Database,
    pubsub: Arc<PubSub>,
    writer: Arc<Mutex<OwnedWriteHalf>>,
) -> String {
    if parts.is_empty() {
        return "-ERR empty command\r\n".to_string();
    }

    let cmd = parts[0].to_uppercase();

    match cmd.as_str() {
        // Regular commands
        "PING" | "ECHO" | "SET" | "GET" | "DEL" => handle_string(&cmd, parts, db),
        "INCR" | "INCRBY" | "DECR" | "DECRBY" => handle_number(&cmd, parts, db),
        "EXISTS" | "KEYS" | "EXPIRE" | "TTL" | "PERSIST" => handle_key(&cmd, parts, db),
        "LPOP" | "RPOP" | "LPUSH" | "RPUSH" | "LLEN" | "LINDEX" | "LRANGE" | "LSET" => {
            handle_list(&cmd, parts, db)
        }
        "SADD" | "SREM" | "SMEMBERS" | "SISMEMBER" | "SCARD" => handle_set(&cmd, parts, db),
        "HSET" | "HGET" | "HDEL" | "HKEYS" | "HVALS" | "HLEN" | "HGETALL" | "HEXISTS" => {
            handle_hash_set(&cmd, parts, db)
        }
        "FLUSHDB" => {
            db.flushdb();
            "+OK\r\n".to_string()
        }

        "SUBSCRIBE" => {
            handle_subscribe(parts.iter().map(|s| s.as_str()).collect(), writer, pubsub).await;
            "".to_string()
        }

        "PUBLISH" => {
            if parts.len() < 3 {
                return "-ERR usage: PUBLISH <channel> <message>\r\n".to_string();
            }
            let channel = parts[1].clone();
            let message = parts[2..].join(" ");
            let delivered = pubsub.publish(&channel, message).await;
            format!(":{}\r\n", delivered)
        }

        _ => "-ERR unknown command\r\n".to_string(),
    }
}
