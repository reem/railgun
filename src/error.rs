use std::error::{Error, FromError};
use std::io;

use {hyper};

pub enum RailgunError {
    HyperError(hyper::HttpError),
    IoError(io::IoError)
}

impl Error for RailgunError {
    fn description(&self) -> &str {
        match *self {
            RailgunError::HyperError(ref err) => err.description(),
            RailgunError::IoError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RailgunError::HyperError(ref err) => Some(err as &Error),
            RailgunError::IoError(ref err) => Some(err as &Error)
        }
    }
}

impl FromError<hyper::HttpError> for RailgunError {
    fn from_error(err: hyper::HttpError) -> RailgunError {
        RailgunError::HyperError(err)
    }
}

impl FromError<io::IoError> for RailgunError {
    fn from_error(err: io::IoError) -> RailgunError {
        RailgunError::IoError(err)
    }
}

