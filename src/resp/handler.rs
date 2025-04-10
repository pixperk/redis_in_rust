use crate::store::db::Database;

use super::{commands::{hash_set::handle_hash_set, key::handle_key, list::handle_list, number::handle_number, set::handle_set, string::handle_string}, utils::{bulk_string, format_array, parse_resp, wrong_args}};



pub fn handle_command(input: &str, db: &mut Database) -> String {
    let parts = parse_resp(input);
    if parts.is_empty() {
        return "-ERR empty command\r\n".to_string();
    }

    let cmd = parts[0].to_uppercase();
    match cmd.as_str() {
        "PING" | "ECHO" | "SET" | "GET" | "DEL" =>{
         handle_string(&cmd, &parts, db)
        }

        "INCR"|"INCRBY"|"DECR"|"DECRBY" => {
            handle_number(&cmd, &parts, db)
        }

        "EXISTS"|"KEYS"|"EXPIRE"|"TTL"|"PERSIST" => {
            handle_key(&cmd, &parts, db)
        }
       
       "LPOP"|"RPOP"|"LPUSH"|"RPUSH"|"LLEN"|"LINDEX"|"LRANGE"|"LSET" => {
            handle_list(&cmd, &parts, db)
        }

        "SADD"|"SREM"|"SMEMBERS"|"SISMEMBER"|"SCARD" => {
            handle_set(&cmd, &parts, db)
        }

        "HSET"|"HGET"|"HDEL"|"HKEYS"|"HVALS"|"HLEN"|"HGETALL"|"HEXISTS" => {
            handle_hash_set(&cmd, &parts, db)
        }
       

        "FLUSHDB" => {
            db.flushdb();
            "+OK\r\n".to_string()
        }
       

        _ => "-ERR unknown command\r\n".to_string(),
    }
}
