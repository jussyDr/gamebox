//! Reading GameBox nodes.

pub(crate) mod readable;

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
};

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
/// It is recommended that the given `reader` is buffered for optimal performance.
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
pub fn read<T: Readable>(reader: impl BufRead + Seek) -> Result<T> {
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

/// A GameBox node reader.
#[derive(Default)]
pub struct Reader {
    read_header: HeaderOptions,
    skip_body: BodyOptions,
}

// read header
// read header, skip heavy,
// dont read header, ignore size
// dont read header

impl Reader {
    /// Create a new GameBox node reader.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gamebox::read::Reader;
    /// let reader = Reader::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set options for reading the header.
    pub fn read_header(mut self, read_header: HeaderOptions) -> Self {
        self.read_header = read_header;
        self
    }

    /// Set options for reading the body.
    pub fn read_body(mut self, skip_body: BodyOptions) -> Self {
        self.skip_body = skip_body;
        self
    }

    /// Read a node of type `T` from the given `reader`.
    ///
    /// It is recommended that the given `reader` is buffered for optimal performance.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// # use gamebox::read::Reader;
    /// # use gamebox::Item;
    /// # |reader: std::io::Cursor<&[u8]>| {
    /// let item: Item = Reader::new().read(reader)?;
    /// # Ok::<(), gamebox::read::Error>(()) };
    /// ```
    pub fn read<T: Readable>(&self, reader: impl BufRead + Seek) -> Result<T> {
        T::read(reader, self.read_header, self.skip_body)
    }

    /// Read a node of type `T` from a file at the given `path`.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// # use gamebox::read::Reader;
    /// # use gamebox::Item;
    /// # || {
    /// let item: Item = Reader::new().read_file("MyItem.Item.Gbx")?;
    /// # Ok::<(), gamebox::read::Error>(()) };
    /// ```
    pub fn read_file<T: Readable>(&self, path: impl AsRef<Path>) -> Result<T> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        self.read(reader)
    }
}

/// Options for reading the header.
#[derive(Clone, Copy)]
pub enum HeaderOptions {
    /// Should read the body.
    Read {
        /// Set whether or not read heavy chunks.
        ///
        /// Set to `true` by default.
        read_heavy_chunks: bool,
    },
    /// Should skip reading the header.
    Skip {
        /// Assume that the header size field is zero.
        ///
        /// This option exists for reading nodes extracted with
        /// the hook extract option using OpenPlanet, which sets
        /// the header size field to an incorrect value.
        assume_size_zero: bool,
    },
}

impl Default for HeaderOptions {
    fn default() -> Self {
        Self::Read {
            read_heavy_chunks: true,
        }
    }
}

/// Options for reading the body.
#[derive(Clone, Copy)]
pub enum BodyOptions {
    /// Should read the body.
    Read {
        /// Set whether or not to read skippable chunks.
        ///
        /// Set to `true` by default.
        read_skippable_chunks: bool,
    },
    /// Should skip reading the body.
    Skip,
}

impl Default for BodyOptions {
    fn default() -> Self {
        Self::Read {
            read_skippable_chunks: true,
        }
    }
}
