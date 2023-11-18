//! Types for writing GameBox nodes.

use std::io::{self, Write};

/// Error while writing a GameBox node.
#[derive(Debug)]
pub enum Error {
    /// An I/O error.
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(io_err: io::Error) -> Self {
        Self::Io(io_err)
    }
}

/// Result type used when writing GameBox nodes.
pub type Result<T> = std::result::Result<T, Error>;

trait WriteableInner {}

/// Trait which indicates that a certain class is writable.
pub trait Writeable: WriteableInner {}

pub fn write<T: Writeable>(writer: impl Write, node: T) -> Result<()> {
    todo!()
}
