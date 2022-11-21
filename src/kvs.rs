use crate::commands::Command;
use crate::errors::KvsError;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Write,
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
        // let mut file = File::open(&self.log).unwrap();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.log)
            .unwrap();

        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };

        let append = serde_json::to_string(&command).unwrap();

        println!("append, {}", append);

        file.write(append.as_bytes()).unwrap();

        self.data.insert(key, value);
        Ok(())
    }
    pub fn get(&self, key: String) -> Result<Option<String>> {
        self.data.get(&key).cloned();
        todo!()
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.data.remove(&key);
        todo!()
    }
}
