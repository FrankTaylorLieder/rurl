use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use url::Url;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to bind to
    #[arg(short, long, default_value_t = 7878)]
    port: u16,

    /// Allow all valid URL schemes (not just http/https)
    #[arg(long)]
    allow_all_schemes: bool,
}

fn is_safe_url(input: &str, allow_all_schemes: bool) -> Result<String, String> {
    // 1. Prevent flag injection
    if input.trim().starts_with('-') {
        return Err("URL cannot start with '-'".to_string());
    }

    // 2. Parse URL with strict validation
    let parsed = Url::parse(input.trim())
        .map_err(|e| format!("Invalid URL: {}", e))?;

    // 3. Check scheme allowlist
    if !allow_all_schemes {
        match parsed.scheme() {
            "http" | "https" => {},
            other => return Err(format!("Scheme '{}' not allowed. Only http/https permitted (use --allow-all-schemes to enable)", other))
        }
    }

    Ok(parsed.to_string())
}

fn handle_client(stream: TcpStream, id: usize, allow_all_schemes: bool) {
    println!("Connection open {}", id);

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(url) => {
                let url = url.trim();
                if !url.is_empty() {
                    match is_safe_url(url, allow_all_schemes) {
                        Ok(safe_url) => {
                            println!("Opening on {}: {}", id, safe_url);
                            let _ = Command::new("open").arg(safe_url).spawn();
                        }
                        Err(e) => {
                            eprintln!("Rejected unsafe URL on {}: {} ({})", id, url, e);
                        }
                    }
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

fn main() {
    let args = Args::parse();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .expect("Failed to bind to port");

    println!("Server listening on port {}", args.port);
    if args.allow_all_schemes {
        println!("WARNING: Allowing all URL schemes (security reduced)");
    } else {
        println!("Only allowing http/https URLs (use --allow-all-schemes to enable others)");
    }

    let mut next_id = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                next_id += 1;
                let id = next_id;
                let allow_all = args.allow_all_schemes;
                std::thread::spawn(move || handle_client(stream, id, allow_all));
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}
