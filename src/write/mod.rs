//! Writing GameBox nodes.

pub(crate) mod writable;

pub use lzo1x::CompressLevel;

use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

use self::writable::write_gbx;

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

/// A GameBox node writer.
pub struct Writer {
    body_compression: Option<CompressLevel>,
}

impl Writer {
    /// Create a new writer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gamebox::write::Writer;
    /// let writer = Writer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            body_compression: Some(CompressLevel::default()),
        }
    }

    /// Set how to compress the body.
    ///
    /// Set to `None` to leave the body uncompressed.
    ///
    /// The default is to compress the body with level 3.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gamebox::write::{Writer, CompressLevel};
    /// let writer = Writer::new().body_compression(Some(CompressLevel::default()));
    /// ```
    pub fn body_compression(mut self, body_compression: Option<CompressLevel>) -> Self {
        self.body_compression = body_compression;
        self
    }

    /// Write the given `node` to the given `writer`.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// # use gamebox::write::Writer;
    /// # |map: gamebox::Map, writer: std::io::Cursor<&mut [u8]>| {
    /// Writer::new().write(&map, writer)?;
    /// # Ok::<(), gamebox::write::Error>(()) };
    /// ```
    pub fn write(&self, node: &impl Writable, writer: impl Write) -> Result {
        write_gbx(node, writer, self.body_compression)
    }

    /// Write the given `node` to a file at the given `path`.
    ///
    /// This function will create a file if it does not exist, and will truncate it if it does.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// # use gamebox::write::Writer;
    /// # |map: gamebox::Map| {
    /// Writer::new().write_file(&map, "MyMap.Item.Gbx")?;
    /// # Ok::<(), gamebox::write::Error>(()) };
    /// ```
    pub fn write_file(&self, node: &impl Writable, path: impl AsRef<Path>) -> Result {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        self.write(node, writer)
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}
