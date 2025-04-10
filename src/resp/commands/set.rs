use crate::{resp::utils::{format_array, wrong_args}, store::Database};

pub fn handle_set(cmd: &str, parts: &[String], db: &mut Database) -> String {
    match cmd {
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

        _ => {
            format!("-ERR unknown command '{}'\r\n", cmd)
        }
    }
}