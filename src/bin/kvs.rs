use clap::Parser;
use kvs::{Cli, Command, KvStore, Result};

fn main() -> Result<()> {
    let mut kvs = KvStore::new().unwrap();
    let cli = Cli::parse();

    match &cli.command {
        Command::Get { key: _ } => {
            eprintln!("get unimplemented");
            std::process::exit(1)
        }
        Command::Set { key, value } => {
            println!("Command::Set");
            println!("key: {}", key);
            println!("value: {}", value);
            kvs.set(key.to_string(), value.to_string()).unwrap();
            Ok(())
        }
        Command::Rm { key: _ } => {
            eprintln!("Rm unimplemented");
            std::process::exit(1)
        }
    }
}
