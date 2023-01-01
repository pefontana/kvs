use serde_json::Deserializer;

use crate::commands::Command;
use crate::errors::KvsError;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
pub type Result<T> = std::result::Result<T, KvsError>;

pub struct KvStore {
    index: HashMap<String, CommandIndex>,
    _log: PathBuf,
    reader: BufReader<File>,
    buf_writer_with_pos: BufWriterWithPos<File>,
}

#[derive(Debug)]
pub struct CommandIndex {
    start: usize,
    len: usize,
}

impl CommandIndex {
    fn new(start: usize, len: usize) -> Self {
        CommandIndex { start, len }
    }
}

pub struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: usize,
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))? as usize;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let log = path.join(PathBuf::from("log"));

        if !log.exists() {
            OpenOptions::new().write(true).create(true).open(&log)?;
        }

        let file = OpenOptions::new().read(true).open(&log)?;

        let buf_writer_with_pos = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&log)?,
        )?;
        let mut kvs = KvStore {
            index: HashMap::new(),
            _log: log,
            reader: BufReader::new(file),
            buf_writer_with_pos,
        };

        kvs.load()?;

        Ok(kvs)
    }
    pub fn load(&mut self) -> Result<()> {
        let mut pos = self.reader.seek(SeekFrom::Start(0))? as usize;
        let mut stream = Deserializer::from_reader(self.reader.get_ref()).into_iter::<Command>();

        while let Some(command) = stream.next() {
            let new_pos = stream.byte_offset();
            let len = new_pos - pos;
            match command? {
                Command::Set { key: k, value: _v } => {
                    self.index.insert(
                        k,
                        CommandIndex {
                            start: pos,
                            len: len,
                        },
                    );
                }
                Command::Rm { key: k } => {
                    self.index.remove(&k);
                }
                Command::Get { key: _key } => return Err(KvsError::Error),
            };

            self.buf_writer_with_pos.pos = pos + len;
            pos = new_pos;
        }
        Ok(())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value,
        };

        let command_json = serde_json::to_string(&command)?;

        let start = self.buf_writer_with_pos.pos;
        let len = self.buf_writer_with_pos.write(command_json.as_bytes())?;
        self.index.insert(key, CommandIndex::new(start, len));
        self.buf_writer_with_pos.flush()?;
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // let contents =
        //     fs::read_to_string(&self._log).expect("Should have been able to read the file");
        // println!("log: {:?}", contents);
        let cmdpos = if let Some(a) = self.index.get(&key) {
            a
        } else {
            return Ok(None);
        };
        self.reader.seek(SeekFrom::Start(cmdpos.start as u64))?;

        let command: Command =
            serde_json::from_reader(self.reader.get_ref().take(cmdpos.len as u64))?;

        if let Command::Set { key: _k, value: v } = command {
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.remove(&key).is_none() {
            return Err(KvsError::KeyNotFound(key));
        }

        let command = Command::Rm { key: key.clone() };

        let command_json = serde_json::to_string(&command)?;
        let _len = self.buf_writer_with_pos.write(command_json.as_bytes())?;
        self.buf_writer_with_pos.flush()?;

        Ok(())
    }
}
