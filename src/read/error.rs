use std::{
    collections::VecDeque,
    fmt::{self, Debug, Display, Formatter},
    io,
};

/// Read error.
#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) trace: VecDeque<TraceEntry>,
}

impl Error {
    pub(crate) const fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            trace: VecDeque::new(),
        }
    }

    pub(crate) fn io(io_error: io::Error) -> Self {
        let kind = match io_error.kind() {
            io::ErrorKind::UnexpectedEof => ErrorKind::Format("unexpected EOF".into()),
            _ => ErrorKind::Io(io_error),
        };

        Self {
            kind,
            trace: VecDeque::new(),
        }
    }

    pub(crate) fn version(name: &str, version: u32) -> Self {
        Self {
            kind: ErrorKind::Unsupported(format!("{name} version: {version}")),
            trace: VecDeque::new(),
        }
    }

    pub(crate) fn chunk_version(version: u32) -> Self {
        Self::version("chunk", version)
    }

    pub(crate) fn enum_variant(name: &str, value: u32) -> Self {
        Self {
            kind: ErrorKind::Unsupported(format!("{name} variant: {value}")),
            trace: VecDeque::new(),
        }
    }

    pub(crate) fn index() -> Self {
        todo!()
    }

    pub(crate) fn null(name: &str) -> Self {
        Self {
            kind: ErrorKind::Format(format!("{name} is null")),
            trace: VecDeque::new(),
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
pub(crate) enum ErrorKind {
    Io(io::Error),
    Unsupported(String),
    Format(String),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io(ref err) => Display::fmt(err, f),
            Self::Unsupported(ref message) => f.write_str(message),
            Self::Format(ref message) => f.write_str(message),
        }
    }
}

pub(crate) struct TraceEntry {
    pub class_id: u32,
    pub chunk_num: Option<u16>,
}

impl Debug for TraceEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TraceEntry {{ class_id: 0x{:08x} }}", self.class_id)
    }
}
