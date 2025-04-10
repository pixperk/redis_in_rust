use crate::{resp::utils::{bulk_string, wrong_args}, store::Database};

pub fn handle_key (cmd: &str, parts: &[String], db: &mut Database) -> String {
    match cmd{
        "EXISTS" => {
            if parts.len() < 2 {
                wrong_args("EXISTS")
            } else {
                let exists = db.exists(&parts[1..]);
                format!(":{}\r\n", exists)
            }
        }

        "KEYS" => {
            let keys = db.keys();
            let mut response = format!("*{}\r\n", keys.len());
            for key in keys {
                response.push_str(&bulk_string(&key));
            }
            response
        }

        "EXPIRE" => {
            if let (Some(key), Some(seconds_str)) = (parts.get(1), parts.get(2)) {
                if let Ok(seconds) = seconds_str.parse::<u64>() {
                    db.expire(key, seconds);
                    "+OK\r\n".to_string()
                } else {
                    "-ERR invalid seconds\r\n".to_string()
                }
            } else {
                wrong_args("EXPIRE")
            }
        }
        "TTL" => {
            if let Some(key) = parts.get(1) {
                let ttl = db.ttl(key);
                format!(":{}\r\n", ttl)
            } else {
                wrong_args("TTL")
            }
        }
        "PERSIST" => {
            if let Some(key) = parts.get(1) {
                db.persist(key);
                "+OK\r\n".to_string()
            } else {
                wrong_args("PERSIST")
            }
        }

        _ => {
            format!("-ERR unknown command '{}'\r\n", cmd)
        }
    }
}