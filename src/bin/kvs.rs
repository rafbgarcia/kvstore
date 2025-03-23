use clap::{Parser, Subcommand};
use kvs::KvStore;

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

fn main() {
    let cli = Cli::parse();
    let mut kvs = KvStore::new();

    match &cli.command {
        Some(Commands::Set { key, value }) => {
            kvs.set(key.to_owned(), value.to_owned());
            panic!("unimplemented");
        }

        Some(Commands::Get { key }) => {
            kvs.get(key.to_owned());
            panic!("unimplemented");
        }

        Some(Commands::Rm { key }) => {
            kvs.remove(key.to_owned());
            panic!("unimplemented");
        }

        None => {
            panic!("unimplemented");
        }
    }
}
