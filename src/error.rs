use failure::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    #[fail(display = "key not found")]
    KeyNotFound,

    #[fail(display = "Unexpcetd command type")]
    UnexpectedCommandType,

    #[fail(display = "InvalidRequest")]
    InvalidRequest,

    #[fail(display = "InvalidReply")]
    InvalidReply
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> Self {
        KvsError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;
