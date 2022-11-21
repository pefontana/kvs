use std::{collections::HashMap, path::Path};

use tempfile::TempDir;

pub struct KvStore {
    data: HashMap<String, String>,
}
pub type Result<T> = std::result::Result<T, KvsError>;

#[derive(Debug)]
pub enum KvsError {
    Error,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        todo!()
    }
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.data.insert(key, value);
        todo!()
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
