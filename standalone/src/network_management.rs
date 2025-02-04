use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const LEVEL: &str = include_str!("level1.json");

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").expect("Could not bind on port 8080");
    println!("Listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { std::thread::spawn(|| handle_client(stream)); }
            Err(e) => { eprintln!("Failed to establish connection: {}", e); }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).expect("Could not read from stream");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
    let response = LEVEL.as_bytes();
    stream.write(response).expect("Could not write to stream");
}
