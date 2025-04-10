use crate::{resp::utils::{bulk_string, format_array, wrong_args}, store::Database};

pub fn handle_hash_set(cmd: &str, parts: &[String], db: &mut Database) -> String {
    match cmd {
       
        "HSET" => {
            if let Some(key) = parts.get(1) {
                if parts.len() < 4 || parts.len() % 2 != 0 {
                    return wrong_args("HSET");
                }

                let mut inserted = 0;
                let mut i = 2;
                while i < parts.len() {
                    if let (Some(field), Some(value)) = (parts.get(i), parts.get(i + 1)) {
                        inserted += db.hset(key, field, value);
                        i += 2;
                    } else {
                        return wrong_args("HSET");
                    }
                }

                format!(":{}\r\n", inserted)
            } else {
                wrong_args("HSET")
            }
        }

        "HGET" => {
            if let (Some(key), Some(field)) = (parts.get(1), parts.get(2)) {
                match db.hget(key, field) {
                    Some(val) => bulk_string(&val),
                    None => "$-1\r\n".to_string(),
                }
            } else {
                wrong_args("HGET")
            }
        }

        "HDEL" => {
            if let Some(key) = parts.get(1) {
                let fields: Vec<String> = parts.iter().skip(2).cloned().collect();
                if fields.is_empty() {
                    wrong_args("HDEL")
                } else {
                    let removed = db.hdel(key, &fields);
                    format!(":{}\r\n", removed)
                }
            } else {
                wrong_args("HDEL")
            }
        }

        "HKEYS" => {
            if let Some(key) = parts.get(1) {
                format_array(db.hkeys(key))
            } else {
                wrong_args("HKEYS")
            }
        }

        "HVALS" => {
            if let Some(key) = parts.get(1) {
                format_array(db.hvals(key))
            } else {
                wrong_args("HVALS")
            }
        }

        "HLEN" => {
            if let Some(key) = parts.get(1) {
                let len = db.hlen(key);
                format!(":{}\r\n", len)
            } else {
                wrong_args("HLEN")
            }
        }

        "HGETALL" => {
            if let Some(key) = parts.get(1) {
                let hash = db.hgetall(key);
                let mut resp = format!("*{}\r\n", hash.len() * 2);
                for (k, v) in hash {
                    resp.push_str(&bulk_string(&k));
                    resp.push_str(&bulk_string(&v));
                }
                resp
            } else {
                wrong_args("HGETALL")
            }
        }

        "HEXISTS" => {
            if let (Some(key), Some(field)) = (parts.get(1), parts.get(2)) {
                let exists = db.hexists(key, field);
                format!(":{}\r\n", if exists { 1 } else { 0 })
            } else {
                wrong_args("HEXISTS")
            }
        }

        _ => {
            format!("-ERR unknown command '{}'\r\n", cmd)
        }

    }
}