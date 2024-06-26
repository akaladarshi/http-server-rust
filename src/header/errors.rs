use std::{fmt, io};
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum HeaderError {
    NoHeaderFound,
    NoStatusLine,
    InvalidMethod,
    InvalidProtocolVersion,
    IOError(io::Error),  // To wrap I/O errors
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HeaderError::NoHeaderFound => write!(f, "header not found"),
            HeaderError::NoStatusLine => write!(f, "no status line found"),
            HeaderError::InvalidMethod => write!(f, "invalid method"),
            HeaderError::InvalidProtocolVersion => write!(f, "invalid protocol version: version should be HTTP/1.1"),
            HeaderError::IOError(ref err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl Error for HeaderError{}

impl From<io::Error> for HeaderError{
    fn from(error: io::Error) -> Self {
        HeaderError::IOError(error)
    }
}

impl From<HeaderError> for io::Error {
    fn from(err: HeaderError) -> Self {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}

