use crate::{
    persistence::Persister, resp::handler::handle_command, store::db::Database,
    utils::is_mutating_command,
};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub fn handle_connection(
    mut stream: TcpStream,
    db: Arc<Mutex<Database>>,
    persister: Arc<dyn Persister + Send + Sync>,
) {
    let mut buf = [0; 512];

    loop {
        let bytes_read = match stream.read(&mut buf) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Read error: {e}");
                break;
            }
        };

        let input = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("Received: {input}");

        let mut db = match db.lock() {
            Ok(guard) => guard,
            Err(e) => {
                eprintln!("Failed to lock DB: {e}");
                break;
            }
        };

        let response = handle_command(&input, &mut db);

        let command_name = input
            .lines()
            .find(|line| line.starts_with('$'))
            .map(|_| input.lines().nth(2).unwrap_or("")) // crude but works for RESP
            .unwrap_or("");

        //SAVE to disk if mutating
        if is_mutating_command(command_name) {
            if let Err(e) = persister.save(&db) {
                eprintln!("Failed to save database: {e}");
            } else {
                println!("💾 Database saved to disk");
            }
        }

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Write error: {e}");
            break;
        }
    }
}
