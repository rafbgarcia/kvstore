use std::path::Path;

use clap::{Parser, Subcommand};
use kvs::{KvStore, KvsError, Result};

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
    let mut kvs = KvStore::open(Path::new("./"))?;

    match &cli.command {
        Some(Commands::Set { key, value }) => {
            kvs.set(key.to_string(), value.to_string())?;
            Ok(())
        }

        Some(Commands::Get { key }) => {
            if let Some(value) = kvs.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
            Ok(())
        }

        Some(Commands::Rm { key }) => {
            match kvs.remove(key.to_string()) {
                Ok(()) => {}

                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    std::process::exit(1);
                }

                Err(e) => {
                    println!("Error: {}", e);
                    std::process::exit(1);
                }
            }

            Ok(())
        }

        None => {
            panic!("unimplemented");
        }
    }
}
