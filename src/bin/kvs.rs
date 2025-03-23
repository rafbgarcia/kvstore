use clap::{Parser, Subcommand};

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

    match &cli.command {
        Some(Commands::Set { key: _, value: _ }) => {
            panic!("unimplemented");
        }

        Some(Commands::Get { key: _ }) => {
            panic!("unimplemented");
        }

        Some(Commands::Rm { key: _ }) => {
            panic!("unimplemented");
        }

        None => {
            panic!("unimplemented");
        }
    }
}
