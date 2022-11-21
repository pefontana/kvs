use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}
fn main() {
    let _kvs = KvStore::new();
    let cli = Cli::parse();

    match &cli.command {
        Command::Get { key: _ } => {
            eprintln!("get unimplemented");
            std::process::exit(1)
        }
        Command::Set { key: _, value: _ } => {
            eprintln!("set unimplemented");
            std::process::exit(1)
        }
        Command::Rm { key: _ } => {
            eprintln!("set unimplemented");
            std::process::exit(1)
        }
    }
}
