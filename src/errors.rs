use std::io;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "Undefined  KvsError")]
    Error,
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    #[fail(display = "{}, Key not found", 0)]
    KeyNotFound(String),
    #[fail(display = "insufficient Log Size. Try to increase the LOG_MAX_SIZE")]
    InsufficientLogSize(),
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}
impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}
