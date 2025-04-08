use crate::db::Database;

pub fn parse_resp(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|line| !line.starts_with('*') && !line.starts_with('$'))
        .map(|line| line.to_string())
        .collect()
}

pub fn handle_command(input: &str, db: &mut Database) -> String {
    let parts = parse_resp(input);
    if parts.is_empty() {
        return "-ERR empty command\r\n".to_string();
    }
    match parts[0].to_uppercase().as_str() {
        "PING" => "+PONG\r\n".to_string(),

        "ECHO" => {
            if let Some(arg) = parts.get(1) {
                format!("${}\r\n{}\r\n", arg.len(), arg)
            } else {
                "-ERR wrong number of arguments for 'ECHO'\r\n".to_string()
            }
        }

        "SET" => {
            if let (Some(key), Some(value)) = (parts.get(1), parts.get(2)) {
                db.set(key.clone(), value.clone());
                "+OK\r\n".to_string()
            } else {
                "-ERR wrong number of arguments for 'SET'\r\n".to_string()
            }
        }

        "GET" => {
            if let Some(key) = parts.get(1) {
                if let Some(value) = db.get(key) {
                    format!("${}\r\n{}\r\n", value.len(), value)
                } else {
                    "$-1\r\n".to_string()
                }
            } else {
                "-ERR wrong number of arguments for 'GET'\r\n".to_string()
            }
        }

        "DEL" => {
            if parts.len() < 2 {
                "-ERR wrong number of arguments for 'DEL'\r\n".to_string()
            } else {
                let deleted = db.delete(&parts[1..]);
                format!(":{}\r\n", deleted)
            }
        }

        "EXISTS" => {
            if parts.len() < 2 {
                "-ERR wrong number of arguments for 'EXISTS'\r\n".to_string()
            } else {
                let exists = db.exists(&parts[1..]);
                format!(":{}\r\n", exists)
            }
        }

        "INCR" => {
            if let Some(key) = parts.get(1) {
                match db.incr(key) {
                    Ok(val) => format!(":{}\r\n", val),
                    Err(e) => format!("-ERR {}\r\n", e),
                }
            } else {
                "-ERR wrong number of arguments for 'INCR'\r\n".to_string()
            }
        }

        "INCRBY" => {
            if let (Some(key), Some(arg)) = (parts.get(1), parts.get(2)) {
                let by: i64 = arg.parse().unwrap_or(0);
                match db.incr_by(key, by) {
                    Ok(val) => format!(":{}\r\n", val),
                    Err(e) => format!("-ERR {}\r\n", e),
                }
            } else {
                "-ERR wrong number of arguments for 'INCRBY'\r\n".to_string()
            }
        }

        "DECRBY" => {
            if let (Some(key), Some(arg)) = (parts.get(1), parts.get(2)) {
                let by: i64 = arg.parse().unwrap_or(0);
                match db.incr_by(key, -by) {
                    Ok(val) => format!(":{}\r\n", val),
                    Err(e) => format!("-ERR {}\r\n", e),
                }
            } else {
                "-ERR wrong number of arguments for 'DECRBY'\r\n".to_string()
            }
        }

        "DECR" => {
            if let Some(key) = parts.get(1) {
                match db.incr_by(key, -1) {
                    Ok(val) => format!(":{}\r\n", val),
                    Err(e) => format!("-ERR {}\r\n", e),
                }
            } else {
                "-ERR wrong number of arguments for 'DECR'\r\n".to_string()
            }
        }

        "KEYS" => {
            let keys = db.keys();
            let mut response = format!("*{}\r\n", keys.len());
            for key in keys {
                response.push_str(&format!("${}\r\n{}\r\n", key.len(), key));
            }
            response
        }

        "FLUSHDB" => {
            db.flushdb();
            "+OK\r\n".to_string()
        }

        "LPOP" => {
            if let Some(key) = parts.get(1) {
                if let Some(value) = db.lpop(key) {
                    format!("${}\r\n{}\r\n", value.len(), value)
                } else {
                    "$-1\r\n".to_string()
                }
            } else {
                "-ERR wrong number of arguments for 'LPOP'\r\n".to_string()
            }
        }

        "RPOP" => {
            if let Some(key) = parts.get(1) {
                if let Some(value) = db.rpop(key) {
                    format!("${}\r\n{}\r\n", value.len(), value)
                } else {
                    "$-1\r\n".to_string()
                }
            } else {
                "-ERR wrong number of arguments for 'RPOP'\r\n".to_string()
            }
        }

        "LPUSH" => {
            if let Some(key) = parts.get(1) {
                let len = db.lpush(key, &parts[2..]);
                format!(":{}\r\n", len)
            } else {
                "-ERR wrong number of arguments for 'LPUSH'\r\n".to_string()
            }
        }

        "RPUSH" => {
            if let Some(key) = parts.get(1) {
                let len = db.rpush(key, &parts[2..]);
                format!(":{}\r\n", len)
            } else {
                "-ERR wrong number of arguments for 'RPUSH'\r\n".to_string()
            }
        }

        "LLEN" => {
            if let Some(key) = parts.get(1) {
                let len = db.llen(key);
                format!(":{}\r\n", len)
            } else {
                "-ERR wrong number of arguments for 'LLEN'\r\n".to_string()
            }
        }

        "LINDEX" => {
            if let (Some(key), Some(index_str)) = (parts.get(1), parts.get(2)) {
                if let Ok(index) = index_str.parse::<usize>() {
                    if let Some(value) = db.lindex(key, index) {
                        format!("${}\r\n{}\r\n", value.len(), value)
                    } else {
                        "$-1\r\n".to_string()
                    }
                } else {
                    "-ERR invalid index\r\n".to_string()
                }
            } else {
                "-ERR wrong number of arguments for 'LINDEX'\r\n".to_string()
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
                "-ERR wrong number of arguments for 'LSET'\r\n".to_string()
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
                        response.push_str(&format!("${}\r\n{}\r\n", value.len(), value));
                    }
                    response
                } else {
                    "-ERR invalid range\r\n".to_string()
                }
            } else {
                "-ERR wrong number of arguments for 'LRANGE'\r\n".to_string()
            }
        }

        "SADD" => {
            if let Some(key) = parts.get(1) {
                let len = db.sadd(key, &parts[2..]);
                format!(":{}\r\n", len)
            } else {
                "-ERR wrong number of arguments for 'SADD'\r\n".to_string()
            }
        }
        "SREM" => {
            if let Some(key) = parts.get(1) {
                let len = db.srem(key, &parts[2..]);
                format!(":{}\r\n", len)
            } else {
                "-ERR wrong number of arguments for 'SREM'\r\n".to_string()
            }
        }
        "SMEMBERS" => {
            if let Some(key) = parts.get(1) {
                let members = db.smembers(key);
                let mut response = format!("*{}\r\n", members.len());
                for member in members {
                    response.push_str(&format!("${}\r\n{}\r\n", member.len(), member));
                }
                response
            } else {
                "-ERR wrong number of arguments for 'SMEMBERS'\r\n".to_string()
            }
        }
        "SISMEMBER" => {
            if let (Some(key), Some(member)) = (parts.get(1), parts.get(2)) {
                let is_member = db.sismember(key, member);
                let resp_val = if is_member { 1 } else { 0 };
                format!(":{}\r\n", resp_val)
            } else {
                "-ERR wrong number of arguments for 'SISMEMBER'\r\n".to_string()
            }
        }

        "SCARD" => {
            if let Some(key) = parts.get(1) {
                let score = db.scard(key);
                format!(":{}\r\n", score)
            } else {
                "-ERR wrong number of arguments for 'SCARD'\r\n".to_string()
            }
        }


        _ => "-ERR unknown command\r\n".to_string(),
    }
}
