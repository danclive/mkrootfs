use std::{fmt, io, num};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Xshell(xshell::Error),
    ParseInt(num::ParseIntError),
    FileNotExist(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(ref inner) => inner.source(),
            Error::Xshell(ref inner) => inner.source(),
            Error::ParseInt(ref inner) => inner.source(),
            Error::FileNotExist(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<xshell::Error> for Error {
    fn from(value: xshell::Error) -> Self {
        Error::Xshell(value)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref inner) => inner.fmt(f),
            Error::Xshell(ref inner) => inner.fmt(f),
            Error::ParseInt(ref inner) => inner.fmt(f),
            Error::FileNotExist(ref path) => {
                write!(f, "File '{}' not exist", path)
            }
        }
    }
}
