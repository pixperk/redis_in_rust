use crate::store::db::Database;

fn wrong_args(cmd: &str) -> String {
    format!("-ERR wrong number of arguments for '{}'\r\n", cmd)
}

fn bulk_string(s: &str) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
}

fn format_array(values: Vec<String>) -> String {
    let mut resp = format!("*{}\r\n", values.len());
    for v in values {
        resp.push_str(&bulk_string(&v));
    }
    resp
}

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

    let cmd = parts[0].to_uppercase();
    match cmd.as_str() {
        "PING" => "+PONG\r\n".to_string(),

        "ECHO" => {
            if let Some(arg) = parts.get(1) {
                bulk_string(arg)
            } else {
                wrong_args("ECHO")
            }
        }

        "SET" => {
            if let (Some(key), Some(value)) = (parts.get(1), parts.get(2)) {
                let mut expiry = None;

                if let (Some(option), Some(seconds)) = (parts.get(3), parts.get(4)) {
                    if option.to_uppercase() == "EX" {
                        if let Ok(sec) = seconds.parse::<u64>() {
                            expiry = Some(sec);
                        }
                    }
                }

                db.set(key, value.clone(), expiry);
                "+OK\r\n".to_string()
            } else {
                wrong_args("SET")
            }
        }

        "GET" => {
            if let Some(key) = parts.get(1) {
                if let Some(value) = db.get(key) {
                    bulk_string(&value)
                } else {
                    "$-1\r\n".to_string()
                }
            } else {
                wrong_args("GET")
            }
        }

        "DEL" => {
            if parts.len() < 2 {
                wrong_args("DEL")
            } else {
                let deleted = db.delete(&parts[1..]);
                format!(":{}\r\n", deleted)
            }
        }

        "EXISTS" => {
            if parts.len() < 2 {
                wrong_args("EXISTS")
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
                wrong_args("INCR")
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
                wrong_args("INCRBY")
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
                wrong_args("DECRBY")
            }
        }

        "DECR" => {
            if let Some(key) = parts.get(1) {
                match db.incr_by(key, -1) {
                    Ok(val) => format!(":{}\r\n", val),
                    Err(e) => format!("-ERR {}\r\n", e),
                }
            } else {
                wrong_args("DECR")
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

        "FLUSHDB" => {
            db.flushdb();
            "+OK\r\n".to_string()
        }

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

        "SADD" => {
            if let Some(key) = parts.get(1) {
                let len = db.sadd(key, &parts[2..]);
                format!(":{}\r\n", len)
            } else {
                wrong_args("SADD")
            }
        }

        "SREM" => {
            if let Some(key) = parts.get(1) {
                let len = db.srem(key, &parts[2..]);
                format!(":{}\r\n", len)
            } else {
                wrong_args("SREM")
            }
        }

        "SMEMBERS" => {
            if let Some(key) = parts.get(1) {
                let members = db.smembers(key);
                format_array(members)
            } else {
                wrong_args("SMEMBERS")
            }
        }

        "SISMEMBER" => {
            if let (Some(key), Some(member)) = (parts.get(1), parts.get(2)) {
                let is_member = db.sismember(key, member);
                format!(":{}\r\n", if is_member { 1 } else { 0 })
            } else {
                wrong_args("SISMEMBER")
            }
        }

        "SCARD" => {
            if let Some(key) = parts.get(1) {
                let count = db.scard(key);
                format!(":{}\r\n", count)
            } else {
                wrong_args("SCARD")
            }
        }

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

        _ => "-ERR unknown command\r\n".to_string(),
    }
}
