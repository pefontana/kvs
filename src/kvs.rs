use crate::commands::Command;
use crate::errors::KvsError;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};
pub type Result<T> = std::result::Result<T, KvsError>;

#[derive(Debug)]
pub struct KvStore {
    data: HashMap<String, String>,
    log: PathBuf,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        todo!()
    }
    pub fn new() -> Result<KvStore> {
        File::create("log").unwrap();
        Ok(KvStore {
            data: HashMap::new(),
            log: PathBuf::from("log"),
        })
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let mut file = OpenOptions::new().read(true).append(true).open(&self.log)?;

        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };

        let command_json = serde_json::to_string(&command)?;

        println!("command_json, {}", command_json);

        file.write(command_json.as_bytes())?;

        self.data.insert(key, value);
        Ok(())
    }
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let file = OpenOptions::new().read(true).open(&self.log)?;

        let reader = BufReader::new(file);

        let mut result: Option<String> = None;

        for line in reader.lines() {
            let command: Command = serde_json::from_str(&line?)?;

            match command {
                Command::Set { key: k, value: v } => {
                    if key == k {
                        result = Some(v);
                    }
                }
                Command::Rm { key: k } => {
                    if key == k {
                        result = None;
                    }
                }
                Command::Get { key: _key } => return Err(KvsError::Error),
            };
        }

        if result.is_some() {
            return Ok(result);
        }
        Err(KvsError::KeyNotFound(key))
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.data.remove(&key);
        todo!()
    }
}
