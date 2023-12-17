//! Reading GameBox nodes.

use std::{
    fs::File,
    io::{self, BufReader, Cursor, Read, Seek},
    path::{Path, PathBuf},
};

use serde_jsonrc::Value;

use crate::{
    class::Class,
    deserialize::{Deserializer, IdState, IdStateMut, NodeState, NodeStateMut},
    MAGIC, NODE_END, SKIP,
};

use self::readable::{BodyChunkReadFn, ReadBody, ReadHeader, Sealed};

/// Error that occured while reading a GameBox node.
#[derive(Debug)]
pub enum Error {
    Generic(()),
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

    pub fn read_header(mut self, read_header: HeaderOptions) -> Self {
        self.read_header = read_header;
        self
    }

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
    /// # use gamebox::classes::Item;
    /// # |reader: std::io::Cursor<&[u8]>| {
    /// let item: Item = Reader::new().read(reader)?;
    /// # Ok::<(), gamebox::read::Error>(()) };
    /// ```
    pub fn read<T: Readable>(&self, reader: impl Read + Seek) -> Result<T> {
        T::read(reader, self.read_header, self.skip_body)
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

#[derive(Clone, Copy)]
pub enum HeaderOptions {
    Read { skip_heavy_chunks: bool },
    Skip { assume_size_zero: bool },
}

#[derive(Clone, Copy)]
pub enum BodyOptions {
    Read { skip_skippable_chunks: bool },
    Skip,
}

impl Default for BodyOptions {
    fn default() -> Self {
        Self::Read {
            skip_skippable_chunks: false,
        }
    }
}

impl Default for HeaderOptions {
    fn default() -> Self {
        Self::Read {
            skip_heavy_chunks: false,
        }
    }
}

pub(crate) fn read_gbx<T: Default + Class + ReadHeader + ReadBody>(
    reader: impl Read + Seek,
    header_options: HeaderOptions,
    body_options: BodyOptions,
) -> Result<T> {
    let mut node = T::default();

    let mut d = Deserializer::new(reader, (), ());

    if d.byte_array()? != MAGIC {
        return Err(Error::Generic(()));
    }

    if d.u16()? != 6 {
        return Err(Error::Generic(()));
    }

    if d.u8()? != b'B' {
        return Err(Error::Generic(()));
    }

    if d.u8()? != b'U' {
        return Err(Error::Generic(()));
    }

    let is_body_compressed = match d.u8()? {
        b'C' => true,
        b'U' => false,
        _ => return Err(Error::Generic(())),
    };

    if d.u8()? != b'R' {
        return Err(Error::Generic(()));
    }

    let class_id = d.u32()?;

    if class_id != T::class_id() {
        return Err(Error::Generic(()));
    }

    let user_data_size = d.u32()?;

    match header_options {
        HeaderOptions::Read { skip_heavy_chunks } => {
            read_header(
                &mut node,
                d.take(user_data_size as u64, (), ()),
                skip_heavy_chunks,
            )?;
        }
        HeaderOptions::Skip { assume_size_zero } => {
            if !assume_size_zero {
                d.skip(user_data_size)?;
            }
        }
    }

    let num_nodes = d.u32()?;

    let mut node_state = NodeState::new(num_nodes as usize);

    let num_node_refs = d.u32()?;

    if num_node_refs > 0 {
        d.u32()?;
        let mut folders = vec![];
        read_folders(&mut d, PathBuf::new(), &mut folders)?;
        d.repeat(num_node_refs as usize, |d| {
            d.u32()?;
            let file_name = d.string()?;
            let node_index = d.u32()?;
            d.u32()?;
            let folder_index = d.u32()?;

            let mut file_path = folders[folder_index as usize].clone();
            file_path.push(file_name);

            node_state.set_ref(node_index as usize, file_path);

            Ok(())
        })?;
    }

    if is_body_compressed {
        let body_size = d.u32()?;
        let compressed_body_size = d.u32()?;

        match body_options {
            BodyOptions::Read { .. } => {
                let compressed_body = d.bytes(compressed_body_size as usize)?;
                let mut buf = vec![0; body_size as usize];

                let body = lzo1x_1::decompress_to_slice(&compressed_body, &mut buf).unwrap();
                let reader = Cursor::new(body);

                let mut d = Deserializer::new(reader, IdState::default(), node_state);

                read_body(&mut node, &mut d)?;

                d.end()?;
            }
            BodyOptions::Skip => {
                d.skip(compressed_body_size)?;
            }
        }

        d.end()?;
    } else {
        match body_options {
            BodyOptions::Read { .. } => {
                let reader = d.into_reader();

                let mut d = Deserializer::new(reader, IdState::default(), node_state);

                read_body(&mut node, &mut d)?;

                d.end()?;
            }
            BodyOptions::Skip => {}
        }
    }

    Ok(node)
}

fn read_header<T: ReadHeader, R: Read + Seek, I, N>(
    node: &mut T,
    mut d: Deserializer<R, I, N>,
    skip_heavy_chunks: bool,
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

        if is_heavy_chunk && skip_heavy_chunks {
            d.skip(chunk_size)?;
        } else {
            let mut d = d.take(chunk_size as u64, &mut id_state, ());

            let header_chunk_entry = header_chunk_entries
                .find(|header_chunk_entry| header_chunk_entry.id == chunk_id)
                .ok_or(Error::Generic(()))?;

            (header_chunk_entry.read_fn)(node, &mut d)?;

            d.end()?;
        }
    }

    d.end()?;

    Ok(())
}

fn read_folders<R: Read, I, N>(
    d: &mut Deserializer<R, I, N>,
    path: PathBuf,
    folders: &mut Vec<PathBuf>,
) -> Result<()> {
    folders.push(path.clone());

    d.list(|d| {
        let folder_name = d.string()?;

        let mut path = path.clone();
        path.push(folder_name);

        read_folders(d, path, folders)
    })?;

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
            .ok_or(Error::Generic(()))?;

        match body_chunk_entry.read_fn {
            BodyChunkReadFn::Normal(read_fn) => {
                read_fn(node, d)?;
            }
            BodyChunkReadFn::Skippable(read_fn) => {
                if d.u32()? != SKIP {
                    return Err(Error::Generic(()));
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

impl<T: Sealed> Readable for T {}

pub(crate) mod readable {
    use std::io::{Read, Seek};

    use crate::deserialize::{Deserializer, IdState, IdStateMut, NodeStateMut, Take};

    use super::{BodyOptions, HeaderOptions, Result};

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

    pub trait Sealed {
        fn read(
            reader: impl Read + Seek,
            header_options: HeaderOptions,
            body_options: BodyOptions,
        ) -> Result<Self>
        where
            Self: Sized;
    }

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

pub(crate) trait ReadJson {
    const CLASS_NAME: &'static str;

    fn read(json: Value) -> Result<Self>
    where
        Self: Sized;
}

pub(crate) fn read_json<T: ReadJson>(reader: impl Read) -> Result<T> {
    let mut value: Value = serde_jsonrc::from_reader(reader).unwrap();
    let object = value.as_object_mut().unwrap();
    let class_name = object.get("ClassId").unwrap();

    if class_name != T::CLASS_NAME {
        todo!()
    }

    object.remove("ClassId");

    T::read(value)
}
