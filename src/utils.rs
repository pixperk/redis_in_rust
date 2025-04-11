pub fn is_mutating_command(input: &str) -> bool {
    let cmd = input.trim().split_whitespace().next();

    match cmd {
        Some(cmd) => {
            matches!(
                cmd.to_uppercase().as_str(),
                "SET" | "DEL" 
                | "INCR" | "INCRBY" | "DECR" | "DECRBY"
                | "EXPIRE" | "PERSIST"
                | "LPOP" | "RPOP" | "LPUSH" | "RPUSH" | "LSET"
                | "SADD" | "SREM"
                | "HSET" | "HDEL"
                | "FLUSHDB"
            )
        }
        None => false,
    }
}
