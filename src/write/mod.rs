//! Writing GameBox files.

pub mod writer;

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

/// Error while writing.
pub struct Error;

/// A writable class.
pub trait Writable {}

pub fn write(node: &impl Writable, writer: impl Write) -> Result<(), Error> {
    todo!()
}

pub fn write_file(node: &impl Writable, path: impl AsRef<Path>) -> Result<(), Error> {
    let file = File::create(path).map_err(|_| Error)?;
    let writer = BufWriter::new(file);

    write(node, writer)
}
