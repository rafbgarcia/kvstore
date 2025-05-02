use clap::Parser;
use kvs::Result;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

const DEFAULT_ADDR: &str = "127.0.0.1:4000";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long)]
    addr: Option<String>,
    #[arg(short, long)]
    engine: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let addr = cli.addr.as_deref().unwrap_or(DEFAULT_ADDR);
    let listener = TcpListener::bind(addr)?;
    println!("Server running on {}", addr);

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut req = [0; 1024];
    stream.read(&mut req)?;

    let val = String::from_utf8_lossy(&req);
    println!("Request: {}", val);

    Ok(())
}
