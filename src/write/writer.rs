use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use lzo1x::CompressLevel;

use super::{writable::write_gbx, Result, Writable};

/// A GameBox node writer.
#[derive(Debug)]
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
