//! Reading GameBox nodes.

mod file;
pub(crate) mod readable;
mod reader;

pub use file::GbxFile;
pub use reader::*;

use std::{io::Read, path::Path};

/// Error that occured while reading a GameBox node.
pub type Error = Box<dyn std::error::Error>;

/// Result type used when reading GameBox nodes.
pub type Result<T> = std::result::Result<T, Error>;

/// Implemented by node types that can be read.
///
/// Note that this trait is sealed and can not be implemented for
/// types outside of `gamebox`.
pub trait Readable: readable::Sealed {}

/// Read a node of type `T` from the given `reader`.
///
/// # Examples
///
/// ``` no_run
/// # use gamebox::read;
/// # use gamebox::Item;
/// # |reader: std::io::Cursor<&[u8]>| {
/// let item: Item = read(reader)?;
/// # Ok::<(), gamebox::read::Error>(()) };
/// ```
pub fn read<T: Readable>(reader: impl Read) -> Result<T> {
    Reader::new().read(reader)
}

/// Read a node of type `T` from a file at the given `path`.
///
/// # Examples
///
/// ``` no_run
/// # use gamebox::read_file;
/// # use gamebox::Item;
/// # || {
/// let item: Item = read_file("MyItem.Item.Gbx")?;
/// # Ok::<(), gamebox::read::Error>(()) };
/// ```
pub fn read_file<T: Readable>(path: impl AsRef<Path>) -> Result<T> {
    Reader::new().read_file(path)
}
