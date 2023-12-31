//! Writing GameBox nodes.

pub(crate) mod serialize;
pub(crate) mod writable;

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
/// # |map: gamebox::classes::Map, writer: std::io::Cursor<&mut [u8]>| {
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
/// # |map: gamebox::classes::Map| {
/// write_file(&map, "MyMap.Item.Gbx")?;
/// # Ok::<(), gamebox::write::Error>(()) };
/// ```
pub fn write_file(node: &impl Writable, path: impl AsRef<Path>) -> Result {
    Writer::new().write_file(node, path)
}

/// A GameBox node writer.
pub struct Writer {
    compress_body: bool,
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
            compress_body: true,
        }
    }

    /// Set whether or not to compress the body while writing.
    ///
    /// Set to `true` by default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gamebox::write::Writer;
    /// let writer = Writer::new().compress_body(false);
    /// ```
    pub fn compress_body(mut self, compress_body: bool) -> Self {
        self.compress_body = compress_body;
        self
    }

    /// Write the given `node` to the given `writer`.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// # use gamebox::write::Writer;
    /// # |map: gamebox::classes::Map, writer: std::io::Cursor<&mut [u8]>| {
    /// Writer::new().write(&map, writer)?;
    /// # Ok::<(), gamebox::write::Error>(()) };
    /// ```
    pub fn write(&self, node: &impl Writable, writer: impl Write) -> Result {
        write_gbx(node, writer, self.compress_body)
    }

    /// Write the given `node` to a file at the given `path`.
    ///
    /// This function will create a file if it does not exist, and will truncate it if it does.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// # use gamebox::write::Writer;
    /// # |map: gamebox::classes::Map| {
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
