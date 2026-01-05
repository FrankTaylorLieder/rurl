use std::env;
use std::io::Write;
use std::net::TcpStream;

const DEFAULT_PORT: u16 = 7878;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <url> [port]", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let port: u16 = if args.len() > 2 {
        args[2].parse().expect("Could not parse port")
    } else {
        DEFAULT_PORT
    };

    match TcpStream::connect(format!("127.0.0.1:{}", port)) {
        Ok(mut stream) => match writeln!(stream, "{}", url) {
            Ok(_) => println!("Sent: {}", url),
            Err(e) => eprintln!("Failed to send: {}", e),
        },
        Err(e) => {
            eprintln!("Failed to connect to localhost:{}: {}", port, e);
            std::process::exit(1);
        }
    }
}
