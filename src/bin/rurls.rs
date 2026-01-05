use std::env;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::process::Command;

fn handle_client(stream: TcpStream, id: usize) {
    println!("Connection open {}", id);

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(url) => {
                let url = url.trim();
                if !url.is_empty() {
                    println!("Opening on {}: {}", id, url);
                    let _ = Command::new("open").arg(url).spawn();
                }
            }
            Err(e) => {
                eprintln!("Error reading line on {}: {}", id, e);
                break;
            }
        }
    }

    println!("Connection closed: {}", id)
}

const DEFAULT_PORT: u16 = 7878;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() > 1 {
        args[1].parse().expect("Could not parse port")
    } else {
        DEFAULT_PORT
    };

    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind to port");

    println!("Server listening on port {}", port);

    let mut next_id = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                next_id += 1;
                let id = next_id;
                std::thread::spawn(move || handle_client(stream, id));
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}
