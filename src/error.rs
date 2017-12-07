use std::fmt;
use std::error;
use std::io;
use serde_json;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    SerdeJsonError(serde_json::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::Io(ref err) => write!(f, "IO error: {}", err),
            AppError::SerdeJsonError(ref err) => write!(f, "Serde json error: {}", err),
        }
    }
}

impl error::Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::Io(ref err) => err.description(),
            AppError::SerdeJsonError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AppError::Io(ref err) => Some(err),
            AppError::SerdeJsonError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> AppError {
        AppError::SerdeJsonError(err)
    }
}