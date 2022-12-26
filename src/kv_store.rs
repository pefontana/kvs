use crate::commands::Command;
use crate::errors::KvsError;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, LineWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
pub type Result<T> = std::result::Result<T, KvsError>;

pub struct KvStore {
    data: HashMap<String, String>,
    index: HashMap<String, CommandIndex>,
    log: PathBuf,
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
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos: pos as usize,
        })
    }
}
// pub struct Reader<R: Read + Seek> {
//     reader: BufReader<R>,
//     start: i128,
// }

// impl<R: Read + Seek> Read for Reader<R> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.read(buf)
//     }
// }

// impl<R: Read + Seek> Seek for Reader<R> {
//     fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
//         self.reader.seek(pos)
//     }
// }

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let log = path.join(PathBuf::from("log"));

        let file = OpenOptions::new().read(true).open(&log)?;

        let buf_writer_with_pos = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&log)?,
        )?;
        let kvs = KvStore {
            data: HashMap::new(),
            index: HashMap::new(),
            log,
            reader: BufReader::new(file),
            buf_writer_with_pos,
        };

        if !kvs.log.exists() {
            OpenOptions::new().write(true).create(true).open(&kvs.log)?;
        }

        Ok(kvs)
    }

    fn write_to_log(&mut self, command: &Command) -> Result<()> {
        let file = OpenOptions::new().read(true).append(true).open(&self.log)?;
        let mut file = LineWriter::new(file);

        let command_json = serde_json::to_string(&command)?;

        // println!("command_json, {}", command_json);

        file.write(command_json.as_bytes())?;
        file.write_all(b"\n")?;
        file.flush()?;

        Ok(())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };

        let command_json = serde_json::to_string(&command)?;

        let start = self.buf_writer_with_pos.pos;
        let len = self.buf_writer_with_pos.write(command_json.as_bytes())?;
        self.index.insert(key, CommandIndex::new(start, len));
        let _x = self.buf_writer_with_pos.write(b"\n")?;
        self.buf_writer_with_pos.flush()?;

        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let cmdpos = &self
            .index
            .get(&key)
            .ok_or(KvsError::KeyNotFound(key.clone()))?;
        self.reader.seek(SeekFrom::Start(cmdpos.start as u64))?;
        // let x = self.reader.take(cmdpos.len as u64);

        let command: Command =
            serde_json::from_reader(self.reader.get_ref().take(cmdpos.len as u64))?;

        if let Command::Set { key: _k, value: v } = command {
            Ok(Some(v))
        } else {
            Err(KvsError::Error)
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
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
            self.write_to_log(&Command::Rm { key })?;
            return Ok(());
        }
        println!("Key not found");
        Err(KvsError::KeyNotFound(key))
    }
}
