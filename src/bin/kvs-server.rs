use clap::Parser;
use kvs::{GetResponse, KvStore, Request, Result};
use serde::Deserialize;
use std::{
    io::{BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    path::Path,
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
        let stream = stream?;
        println!("Connection received from: {}", &stream.peer_addr()?);

        handle_client(stream)?;
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut de = serde_json::Deserializer::from_reader(reader);
    let operation = Request::deserialize(&mut de)?;

    let mut kvs = KvStore::open(Path::new("."))?;

    match operation {
        Request::Get { key } => {
            let value = kvs.get(key)?;
            serde_json::to_writer(&mut writer, &GetResponse::Ok(value))?;
        }
        _ => {}
    }

    writer.flush()?;

    Ok(())
}
