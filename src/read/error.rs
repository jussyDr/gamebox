use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    io,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadSignature,
    ClassMismatch,
    Io(io::Error),
    Internal(Box<dyn StdError>),
}

impl Error {
    pub(crate) fn io(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::UnexpectedEof => Self::Internal("unexpected EOF".into()),
            _ => Self::Io(error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Self::BadSignature => writeln!(f, "bad file signature"),
            Self::ClassMismatch => writeln!(f, "class mismatch"),
            Self::Io(ref err) => writeln!(f, "{err}"),
            Self::Internal(ref message) => writeln!(f, "internal error {message}"),
        }
    }
}

impl StdError for Error {}
