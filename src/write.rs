//! Types for writing GameBox nodes.

use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

use crate::{
    serialize::{NodeState, Serializer},
    MAGIC,
};

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
/// # |item: gamebox::classes::Item, writer: std::io::Cursor<&mut [u8]>| {
/// write(&item, writer)?;
/// # Ok::<(), gamebox::write::Error>(()) };
/// ```
pub fn write(node: &impl Writable, writer: impl Write) -> Result<()> {
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
/// # |item: gamebox::classes::Item| {
/// write_file(&item, "MyItem.Item.Gbx")?;
/// # Ok::<(), gamebox::write::Error>(()) };
/// ```
pub fn write_file(node: &impl Writable, path: impl AsRef<Path>) -> Result<()> {
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
    /// # |item: gamebox::classes::Item, writer: std::io::Cursor<&mut [u8]>| {
    /// Writer::new().write(&item, writer)?;
    /// # Ok::<(), gamebox::write::Error>(()) };
    /// ```
    pub fn write(&self, node: &impl Writable, writer: impl Write) -> Result<()> {
        write_node(node, writer, self.compress_body)
    }

    /// Write the given `node` to a file at the given `path`.
    ///
    /// This function will create a file if it does not exist, and will truncate it if it does.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// # use gamebox::write::Writer;
    /// # |item: gamebox::classes::Item| {
    /// Writer::new().write_file(&item, "MyItem.Item.Gbx")?;
    /// # Ok::<(), gamebox::write::Error>(()) };
    /// ```
    pub fn write_file(&self, node: &impl Writable, path: impl AsRef<Path>) -> Result<()> {
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

fn write_node<T: Writable>(node: &T, writer: impl Write, compress_body: bool) -> Result<()> {
    let mut s = Serializer::new(writer, (), ());

    let user_data = write_header(node);
    let (num_nodes, body) = write_body(node);

    s.byte_array(MAGIC)?;
    s.u16(6)?;
    s.u8(b'B')?;
    s.u8(b'U')?;

    if compress_body {
        s.u8(b'C')?;
    } else {
        s.u8(b'U')?;
    }

    s.u8(b'R')?;
    s.u32(T::CLASS_ID)?;
    s.u32(user_data.len() as u32)?;
    s.bytes(&user_data)?;
    s.u32(num_nodes)?;

    s.u32(0)?;

    if compress_body {
        let mut buf = vec![0; lzo1x_1::worst_compress(body.len())];
        let compressed_body = lzo1x_1::compress_to_slice(&body, &mut buf);

        s.u32(body.len() as u32)?;
        s.u32(compressed_body.len() as u32)?;
        s.bytes(compressed_body)?;
    } else {
        s.bytes(&body)?;
    }

    Ok(())
}

fn write_header(node: &impl Writable) -> Vec<u8> {
    let mut header = vec![];
    let mut s = Serializer::new(&mut header, (), ());

    todo!();

    header
}

fn write_body(node: &impl Writable) -> (u32, Vec<u8>) {
    let mut body = vec![];
    let mut s = Serializer::new(&mut body, (), NodeState::new());

    todo!();

    (s.num_nodes(), body)
}

pub(crate) mod writable {
    use crate::class::Class;

    pub trait Sealed: Class {}
}
