use crate::{resp::utils::{bulk_string, wrong_args}, store::Database};

pub fn handle_list(cmd: &str, parts: &[String], db: &mut Database) -> String {
    match cmd {
       
        "LPOP" => {
            if let Some(key) = parts.get(1) {
                if let Some(value) = db.lpop(key) {
                    bulk_string(&value)
                } else {
                    "$-1\r\n".to_string()
                }
            } else {
                wrong_args("LPOP")
            }
        }

        "RPOP" => {
            if let Some(key) = parts.get(1) {
                if let Some(value) = db.rpop(key) {
                    bulk_string(&value)
                } else {
                    "$-1\r\n".to_string()
                }
            } else {
                wrong_args("RPOP")
            }
        }

        "LPUSH" => {
            if let Some(key) = parts.get(1) {
                let len = db.lpush(key, &parts[2..]);
                format!(":{}\r\n", len)
            } else {
                wrong_args("LPUSH")
            }
        }

        "RPUSH" => {
            if let Some(key) = parts.get(1) {
                let len = db.rpush(key, &parts[2..]);
                format!(":{}\r\n", len)
            } else {
                wrong_args("RPUSH")
            }
        }

        "LLEN" => {
            if let Some(key) = parts.get(1) {
                let len = db.llen(key);
                format!(":{}\r\n", len)
            } else {
                wrong_args("LLEN")
            }
        }

        "LINDEX" => {
            if let (Some(key), Some(index_str)) = (parts.get(1), parts.get(2)) {
                if let Ok(index) = index_str.parse::<usize>() {
                    if let Some(value) = db.lindex(key, index) {
                        bulk_string(&value)
                    } else {
                        "$-1\r\n".to_string()
                    }
                } else {
                    "-ERR invalid index\r\n".to_string()
                }
            } else {
                wrong_args("LINDEX")
            }
        }

        "LSET" => {
            if let (Some(key), Some(index_str), Some(value)) =
                (parts.get(1), parts.get(2), parts.get(3))
            {
                if let Ok(index) = index_str.parse::<usize>() {
                    match db.lset(key, index, value.to_string()) {
                        Ok(()) => "+OK\r\n".to_string(),
                        Err(e) => format!("-ERR {}\r\n", e),
                    }
                } else {
                    "-ERR invalid index\r\n".to_string()
                }
            } else {
                wrong_args("LSET")
            }
        }

        "LRANGE" => {
            if let (Some(key), Some(start_str), Some(end_str)) =
                (parts.get(1), parts.get(2), parts.get(3))
            {
                if let (Ok(start), Ok(end)) = (start_str.parse::<isize>(), end_str.parse::<isize>())
                {
                    let values = db.lrange(key, start, end);
                    let mut response = format!("*{}\r\n", values.len());
                    for value in values {
                        response.push_str(&bulk_string(&value));
                    }
                    response
                } else {
                    "-ERR invalid range\r\n".to_string()
                }
            } else {
                wrong_args("LRANGE")
            }
        }

        _ => {
            format!("-ERR unknown command '{}'\r\n", cmd)
        }
    }
}