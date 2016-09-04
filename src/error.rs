//! Errors

use std::fmt;

/// An enum to represent various possible run-time errors that may occur.
#[derive(Debug)]
pub enum Error {
    /// An error happened with I/O.
    IoError(::std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::IoError(err)
    }
}
