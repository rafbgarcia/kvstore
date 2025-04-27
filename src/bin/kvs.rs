use std::path::Path;

use clap::{Parser, Subcommand};
use kvs::{KvStore, Result};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut kvs = KvStore::open(Path::new("./kvs_wal"))?;

    match &cli.command {
        Some(Commands::Set { key, value }) => {
            let result = kvs.set(key.to_string(), value.to_string())?;
            Ok(result)
        }

        Some(Commands::Get { key }) => {
            // kvs.get(key.to_string());
            panic!("unimplemented");
        }

        Some(Commands::Rm { key }) => {
            // kvs.remove(key.to_string());
            panic!("unimplemented");
        }

        None => {
            panic!("unimplemented");
        }
    }
}
