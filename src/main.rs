use clap::{builder::Str, Parser, Subcommand};

#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Command,
    // #[command(subcommand)]
    // command: Commands,
}

#[derive(Subcommand)]
enum Command {
    Get { key: String },
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Get { key } => println!("{} No esta gato", key),
    }
}
