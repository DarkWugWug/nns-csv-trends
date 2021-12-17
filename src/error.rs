use std::error::Error as StdError;
use std::io::Error as IoError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  FileError(IoError)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::FileError(e) => write!(f, "{}", e)
    }
  }
}

impl StdError for Error {}

impl From<IoError> for Error {
  fn from(err: IoError) -> Self {
    return Error::FileError(err)
  }
}