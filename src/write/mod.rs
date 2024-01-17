//! Writing GameBox nodes.

pub(crate) mod writable;
mod writer;

pub use lzo1x::CompressLevel;
pub use writer::*;

use std::{io::Write, path::Path};

/// Error while writing a GameBox node.
pub type Error = Box<dyn std::error::Error>;

/// Result type used when writing GameBox nodes.
pub type Result = std::result::Result<(), Error>;

/// Trait which indicates that a certain class is writable.
///
/// Note that this trait is sealed and can not be implemented for
/// types outside of `gamebox`.
pub trait Writable: writable::Sealed {}

/// Write the given `node` to the given `writer`.
///
/// # Examples
///
/// ``` no_run
/// # use gamebox::write;
/// # |map: gamebox::Map, writer: std::io::Cursor<&mut [u8]>| {
/// write(&map, writer)?;
/// # Ok::<(), gamebox::write::Error>(()) };
/// ```
pub fn write(node: &impl Writable, writer: impl Write) -> Result {
    Writer::new().write(node, writer)
}

/// Write the given `node` to a file at the given `path`.
///
/// This function will create a file if it does not exist, and will truncate it if it does.
///
/// # Examples
///
/// ``` no_run
/// # use gamebox::write_file;
/// # |map: gamebox::Map| {
/// write_file(&map, "MyMap.Item.Gbx")?;
/// # Ok::<(), gamebox::write::Error>(()) };
/// ```
pub fn write_file(node: &impl Writable, path: impl AsRef<Path>) -> Result {
    Writer::new().write_file(node, path)
}
