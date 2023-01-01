use std::path::Path;

use clap::Parser;
use kvs::{Cli, Command, KvStore, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut kvs = KvStore::open(Path::new(""))?;
    match &cli.command {
        Command::Get { key } => {
            let result = kvs.get(key.to_string())?;
            if result.is_some() {
                println!("{}", result.unwrap());
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        Command::Set { key, value } => {
            kvs.set(key.to_string(), value.to_string()).unwrap();
            Ok(())
        }
        Command::Rm { key } => kvs.remove(key.to_string()),
    }
}
