use std::{
    fmt::{self, Display, Formatter},
    io,
};

/// An error that occured while reading.
#[derive(Debug)]
pub struct Error {
    message: String,
    context: Option<Box<Error>>,
}

impl Error {
    pub(crate) fn new(message: impl Into<String>) -> Error {
        Error {
            message: message.into(),
            context: None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)?;

        if let Some(ref context) = self.context {
            Display::fmt(context, f)?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {}

pub fn map_io_error(io_error: io::Error) -> Error {
    Error::new(format!("IO error: {io_error}"))
}

pub fn error_unknown_version(name: &str, version: u32) -> Error {
    Error::new(format!("unknown {name} version: {version}"))
}

pub fn error_unknown_chunk_version(version: u32) -> Error {
    error_unknown_version("chunk", version)
}

pub fn error_unknown_enum_variant(name: &str, value: u32) -> Error {
    Error::new(format!("unknown variant of enum `{name}`: {value}"))
}
