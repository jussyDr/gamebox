//! Types for reading GameBox nodes.

use std::{
    fs::File,
    io::{self, BufReader, Cursor, Read, Seek},
    path::Path,
};

use crate::{
    deserialize::{Deserializer, IdState, IdStateMut, NodeState, NodeStateMut},
    MAGIC, NODE_END, SKIP,
};

use self::readable::{BodyChunkReadFn, ReadBody};

/// Error while reading a GameBox node.
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

/// Result type used when reading GameBox nodes.
pub type Result<T> = std::result::Result<T, Error>;

/// Types that implement this trait are readable .
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
/// # use gamebox::classes::Item;
/// # |reader: std::io::Cursor<&[u8]>| {
/// let item: Item = read(reader)?;
/// # Ok::<(), gamebox::read::Error>(()) };
/// ```
pub fn read<T: Readable>(reader: impl Read + Seek) -> Result<T> {
    Reader::new().read(reader)
}

/// Read a node of type `T` from a file at the given `path`.
///
/// # Examples
///
/// ``` no_run
/// # use gamebox::read_file;
/// # use gamebox::classes::Item;
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
    skip_header: bool,
    skip_body: bool,
}

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

    /// Set whether or not to skip reading the header.
    ///
    /// Set to `false` by default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gamebox::read::Reader;
    /// let reader = Reader::new().skip_header(true);
    /// ```
    pub fn skip_header(mut self, skip_header: bool) -> Self {
        self.skip_header = skip_header;
        self
    }

    /// Set whether or not to skip reading the body.
    ///
    /// Set to `false` by default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gamebox::read::Reader;
    /// let reader = Reader::new().skip_body(true);
    /// ```
    pub fn skip_body(mut self, skip_body: bool) -> Self {
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
    /// # use gamebox::classes::Item;
    /// # |reader: std::io::Cursor<&[u8]>| {
    /// let item: Item = Reader::new().read(reader)?;
    /// # Ok::<(), gamebox::read::Error>(()) };
    /// ```
    pub fn read<T: Readable>(&self, reader: impl Read + Seek) -> Result<T> {
        read_node(reader, self.skip_header, self.skip_body)
    }

    /// Read a node of type `T` from a file at the given `path`.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// # use gamebox::read::Reader;
    /// # use gamebox::classes::Item;
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

fn read_node<T: Readable>(
    reader: impl Read + Seek,
    skip_header: bool,
    skip_body: bool,
) -> Result<T> {
    let mut node = T::default();

    let mut d = Deserializer::new(reader, (), ());

    if d.byte_array()? != MAGIC {
        todo!()
    }

    if d.u16()? != 6 {
        todo!()
    }

    if d.u8()? != b'B' {
        todo!()
    }

    if d.u8()? != b'U' {
        todo!()
    }

    if d.u8()? != b'C' {
        todo!()
    }

    if d.u8()? != b'R' {
        todo!()
    }

    let class_id = d.u32()?;

    if class_id != T::CLASS_ID {
        todo!()
    }

    let user_data_size = d.u32()?;

    if skip_header {
        d.skip(user_data_size)?;
    } else {
        read_header(&mut node, d.take(user_data_size as u64, (), ()))?;
    }

    let num_nodes = d.u32()?;

    let num_node_refs = d.u32()?;

    if num_node_refs > 0 {
        todo!()
    }

    let body_size = d.u32()?;
    let compressed_body_size = d.u32()?;

    if skip_body {
        d.skip(compressed_body_size)?;
    } else {
        let compressed_body = d.bytes(compressed_body_size as usize)?;
        let mut buf = vec![0; body_size as usize];

        let body = lzo1x_1::decompress_to_slice(&compressed_body, &mut buf).unwrap();
        let reader = Cursor::new(body);

        let mut d = Deserializer::new(
            reader,
            IdState::default(),
            NodeState::new(num_nodes as usize),
        );

        read_body(&mut node, &mut d)?;

        d.end()?;
    }

    d.end()?;

    Ok(node)
}

fn read_header<T: Readable, R: Read + Seek, I, N>(
    node: &mut T,
    mut d: Deserializer<R, I, N>,
) -> Result<()> {
    let header_chunks = d.list(|d| {
        let chunk_id = d.u32()?;
        let chunk_size = d.u32()?;

        Ok((chunk_id, chunk_size))
    })?;

    let mut id_state = IdState::default();

    let mut header_chunk_entries = T::header_chunks();

    for (chunk_id, chunk_size) in header_chunks {
        let is_heavy_chunk = chunk_size & 0x80000000 != 0;
        let chunk_size = chunk_size & 0x7FFFFFFF;
        let skip_heavy_chunks = false;

        if is_heavy_chunk && skip_heavy_chunks {
            d.skip(chunk_size)?;
        } else {
            let mut d = d.take(chunk_size as u64, &mut id_state, ());

            let header_chunk_entry = header_chunk_entries
                .find(|header_chunk_entry| header_chunk_entry.id == chunk_id)
                .unwrap();

            (header_chunk_entry.read_fn)(node, &mut d)?;

            d.end()?;
        }
    }

    d.end()?;

    Ok(())
}

pub(crate) fn read_body<T: ReadBody, R: Read, I: IdStateMut, N: NodeStateMut>(
    node: &mut T,
    d: &mut Deserializer<R, I, N>,
) -> Result<()> {
    let mut body_chunk_entries = T::body_chunks();

    loop {
        let chunk_id = d.u32()?;

        if chunk_id == NODE_END {
            break;
        }

        let body_chunk_entry = body_chunk_entries
            .find(|body_chunk_entry| body_chunk_entry.id == chunk_id)
            .unwrap();

        match body_chunk_entry.read_fn {
            BodyChunkReadFn::Normal(read_fn) => {
                read_fn(node, d)?;
            }
            BodyChunkReadFn::Skippable(read_fn) => {
                if d.u32()? != SKIP {
                    todo!()
                }

                let chunk_size = d.u32()?;

                let mut d = d.take2(chunk_size as u64, ());

                read_fn(node, &mut d)?;

                d.end()?;
            }
        }
    }

    Ok(())
}

pub(crate) mod readable {
    use std::io::Read;

    use crate::{
        class::Class,
        deserialize::{Deserializer, IdState, IdStateMut, NodeStateMut, Take},
    };

    use super::Result;

    pub struct HeaderChunkEntry<T, R> {
        pub id: u32,
        pub read_fn: HeaderChunkReadFn<T, R>,
    }

    type HeaderChunkReadFn<T, R> =
        fn(n: &mut T, d: &mut Deserializer<Take<&mut R>, &mut IdState, ()>) -> Result<()>;

    pub struct BodyChunkEntry<T, R, I, N> {
        pub id: u32,
        pub read_fn: BodyChunkReadFn<T, R, I, N>,
    }

    pub enum BodyChunkReadFn<T, R, I, N> {
        Normal(NormalBodyChunkReadFn<T, R, I, N>),
        Skippable(SkippableBodyChunkReadFn<T, R, N>),
    }

    pub type NormalBodyChunkReadFn<T, R, I, N> =
        fn(n: &mut T, d: &mut Deserializer<R, I, N>) -> Result<()>;

    pub type SkippableBodyChunkReadFn<T, R, N> =
        fn(n: &mut T, d: &mut Deserializer<Take<&mut R>, (), &mut N>) -> Result<()>;

    pub trait Sealed: Class + Default + ReadHeader + ReadBody {}

    pub trait ReadHeader {
        fn header_chunks<R: Read>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>>
        where
            Self: Sized;
    }

    pub trait ReadBody {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
        where
            Self: Sized;
    }
}
