use std::{
    fmt::{self, Display, Formatter},
    io,
};

/// Write error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) const fn io(err: io::Error) -> Self {
        Self {
            kind: ErrorKind::Io(err),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
enum ErrorKind {
    Io(io::Error),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io(ref err) => Display::fmt(err, f),
        }
    }
}
