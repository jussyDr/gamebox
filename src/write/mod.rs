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
    body_compression: BodyCompression,
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
            body_compression: BodyCompression::default(),
        }
    }

    /// Sets how to compress the body while writing.
    ///
    /// The default is fast compression with level 3.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gamebox::write::{Writer, BodyCompression, FastBodyCompression};
    /// let writer = Writer::new().body_compression(BodyCompression::Fast(FastBodyCompression::Level3));
    /// ```
    pub fn body_compression(mut self, body_compression: BodyCompression) -> Self {
        self.body_compression = body_compression;
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

/// Configures how the body should be compressed.
#[derive(Clone, Copy)]
pub enum BodyCompression {
    /// Do not compress.
    None,
    /// Compress using a faster compressor but with a lower compression ratio.
    Fast(FastBodyCompression),
    /// Compress using a slower compressor but with a higher compression ratio.
    Slow(SlowBodyCompression),
}

impl Default for BodyCompression {
    fn default() -> Self {
        Self::Fast(FastBodyCompression::default())
    }
}

/// Configures the fast body compression.
#[derive(Clone, Copy, Default)]
pub enum FastBodyCompression {
    /// Slowest but uses the least memory.
    Level1,
    /// Slower but uses less memory.
    Level2,
    /// Faster but uses more memory.
    #[default]
    Level3,
    /// Fastest but uses the most memory.
    Level4,
}

/// Configures the slow body compression.
#[derive(Clone, Copy, Default)]
pub enum SlowBodyCompression {
    /// Level 1.
    Level1,
    /// Level 2.
    Level2,
    /// Level 3.
    Level3,
    /// Level 4.
    Level4,
    /// Level 5.
    Level5,
    /// Level 6.
    Level6,
    /// Level 7.
    Level7,
    /// Level 8.
    #[default]
    Level8,
    /// Level 9.
    Level9,
}
