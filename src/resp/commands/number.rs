use crate::{resp::utils::wrong_args, store::Database};

pub fn handle_number (cmd: &str, parts: &[String], db: &mut Database) -> String {
    match cmd {
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

        _ => {
            format!("-ERR unknown command '{}'\r\n", cmd)
        }
}

}