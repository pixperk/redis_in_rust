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
    match parts[0].to_uppercase().as_str() {
        "PING" => "+PONG\r\n".to_string(),
        "ECHO" => format!("${}\r\n{}\r\n", parts[1].len(), parts[1]),
        "SET" => {
            db.set(parts[1].clone(), parts[2].clone());
            "+OK\r\n".to_string()
        }
        "GET" => {
            if let Some(value) = db.get(&parts[1]) {
                format!("${}\r\n{}\r\n", value.len(), value)
            } else {
                "$-1\r\n".to_string()
            }
        }
        "DEL" => {
            let deleted = db.delete(&parts[1..]);
            format!(":{}\r\n", deleted)
        }
        "EXISTS" => {
            let exists = db.exists(&parts[1..]);
            format!(":{}\r\n", exists)
        }
        "INCR" => match db.incr(&parts[1]) {
            Ok(val) => format!(":{}\r\n", val),
            Err(e) => format!("-ERR {}\r\n", e),
        },
        "INCRBY" => {
            let by: i64 = parts[2].parse().unwrap_or(0);
            match db.incr_by(&parts[1], by) {
                Ok(val) => format!(":{}\r\n", val),
                Err(e) => format!("-ERR {}\r\n", e),
            }
        }

        "DECRBY" => {
            let by: i64 = parts[2].parse().unwrap_or(0);
            match db.incr_by(&parts[1], -by) {
                Ok(val) => format!(":{}\r\n", val),
                Err(e) => format!("-ERR {}\r\n", e),
            }
        }
        "DECR" => match db.incr_by(&parts[1], -1) {
            Ok(val) => format!(":{}\r\n", val),
            Err(e) => format!("-ERR {}\r\n", e),
        },
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

        _ => "-ERR unknown command\r\n".to_string(),
    }
}
