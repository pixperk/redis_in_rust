use std::{io::{Read, Write}, net::TcpStream, sync::{Arc, Mutex}};
use crate::{store::db::Database, resp::handler::handle_command};

pub fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<Database>>) {
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

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Write error: {e}");
            break;
        }
    }
}
