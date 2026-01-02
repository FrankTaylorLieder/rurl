use std::env;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::process::Command;

fn handle_client(stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Connection from {}", peer_addr);

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(url) => {
                let url = url.trim();
                if !url.is_empty() {
                    println!("Opening: {}", url);
                    let _ = Command::new("open").arg(url).spawn();
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
}

const DEFAULT_PORT: u16 = 7878;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() > 1 {
        args[1].parse().unwrap_or(DEFAULT_PORT)
    } else {
        DEFAULT_PORT
    };

    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind to port");

    println!("Server listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}
