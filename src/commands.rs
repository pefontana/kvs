use clap::{Parser, Subcommand};
use serde::Serialize;

#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Serialize)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}
