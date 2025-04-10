use crate::{resp::utils::{bulk_string, wrong_args}, store::Database};

pub fn handle_string(cmd: &str, parts: &[String], db: &mut Database) -> String {
    match cmd{
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
        },
        _ => "-ERR unknown command\r\n".to_string()

    }
    
    }

