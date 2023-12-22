//! Reading GameBox nodes.

pub(crate) mod deserialize;

use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek},
    path::{Path, PathBuf},
};

use serde_jsonrc::Value;

use crate::{class::Class, FILE_SIGNATURE, NODE_END, SKIP};

use self::{
    deserialize::{Deserializer, IdState, IdStateRef, NodeState, NodeStateMut},
    readable::{BodyChunkReadFn, BodyChunks, HeaderChunks},
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

pub(crate) fn read_gbx<T: Default + Class + HeaderChunks + ReadBody>(
    reader: impl Read + Seek,
    header_options: HeaderOptions,
    body_options: BodyOptions,
) -> Result<T> {
    let mut node = T::default();

    let mut d = Deserializer::new(reader, (), ());

    if d.byte_array()? != FILE_SIGNATURE {
        return Err("invalid file signature".into());
    }

    if d.u16()? != 6 {
        return Err("unsupported gbx version".into());
    }

    if d.u8()? != b'B' {
        return Err("unsupported gbx format".into());
    }

    if d.u8()? != b'U' {
        return Err("unsupported reference table compression".into());
    }

    let is_body_compressed = match d.u8()? {
        b'C' => true,
        b'U' => false,
        _ => return Err("invalid body compression".into()),
    };

    if d.u8()? != b'R' {
        return Err("invalid unknown byte".into());
    }

    let class_id = d.u32()?;

    if class_id != T::class_id() {
        return Err("class id does not match".into());
    }

    let user_data_size = d.u32()?;

    match header_options {
        HeaderOptions::Read { read_heavy_chunks } => {
            read_header(
                &mut node,
                d.take(user_data_size as u64, (), ()),
                read_heavy_chunks,
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

                let id_state = IdState::new();
                let mut d = Deserializer::new(reader, &id_state, node_state);

                T::read_body(&mut node, &mut d)?;

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

                let id_state = IdState::new();
                let mut d = Deserializer::new(reader, &id_state, node_state);

                T::read_body(&mut node, &mut d)?;

                d.end()?;
            }
            BodyOptions::Skip => {}
        }
    }

    Ok(node)
}

fn read_header<T: HeaderChunks, R: Read + Seek, I, N>(
    node: &mut T,
    mut d: Deserializer<R, I, N>,
    read_heavy_chunks: bool,
) -> Result<()> {
    let header_chunks = d.list(|d| {
        let chunk_id = d.u32()?;
        let chunk_size = d.u32()?;

        Ok((chunk_id, chunk_size))
    })?;

    let mut id_state = IdState::new();

    let mut header_chunk_entries = T::header_chunks();

    for (chunk_id, chunk_size) in header_chunks {
        let is_heavy_chunk = chunk_size & 0x80000000 != 0;
        let chunk_size = chunk_size & 0x7FFFFFFF;

        if is_heavy_chunk && !read_heavy_chunks {
            d.skip(chunk_size)?;
        } else {
            let mut d = d.take(chunk_size as u64, &id_state, ());

            let header_chunk_entry = header_chunk_entries
                .find(|header_chunk_entry| header_chunk_entry.id == chunk_id)
                .ok_or("unknown header chunk")?;

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

pub(crate) fn read_body_chunks<'a, T: BodyChunks, R: Read, I: IdStateRef<'a>, N: NodeStateMut>(
    node: &mut T,
    d: &mut Deserializer<R, I, N>,
) -> Result<()> {
    let mut body_chunk_entries = T::body_chunks();

    loop {
        let chunk_id = d.u32()?;

        if chunk_id == NODE_END {
            break;
        }

        // println!("{chunk_id:02X}");

        let body_chunk_entry = body_chunk_entries
            .find(|body_chunk_entry| body_chunk_entry.id == chunk_id)
            .ok_or("unknown body chunk")?;

        match body_chunk_entry.read_fn {
            BodyChunkReadFn::Normal(read_fn) => {
                read_fn(node, d)?;
            }
            BodyChunkReadFn::Skippable(read_fn) => {
                if d.u32()? != SKIP {
                    return Err("expected skippable chunk".into());
                }

                let chunk_size = d.u32()?;

                let mut d = d.take2(chunk_size as u64);

                read_fn(node, &mut d)?;

                d.end()?;
            }
        }
    }

    Ok(())
}

pub(crate) mod readable {
    use std::io::{Read, Seek};

    use super::{
        deserialize::{Deserializer, IdState, IdStateRef, NodeStateMut, Take},
        BodyOptions, HeaderOptions, Result,
    };

    pub struct HeaderChunkEntry<T, R> {
        pub id: u32,
        pub read_fn: HeaderChunkReadFn<T, R>,
    }

    type HeaderChunkReadFn<T, R> =
        fn(n: &mut T, d: &mut Deserializer<Take<&mut R>, &IdState, ()>) -> Result<()>;

    pub struct BodyChunkEntry<T, R, I, N> {
        pub id: u32,
        pub read_fn: BodyChunkReadFn<T, R, I, N>,
    }

    pub enum BodyChunkReadFn<T, R, I, N> {
        Normal(NormalBodyChunkReadFn<T, R, I, N>),
        Skippable(SkippableBodyChunkReadFn<T, R, I, N>),
    }

    pub type NormalBodyChunkReadFn<T, R, I, N> =
        fn(n: &mut T, d: &mut Deserializer<R, I, N>) -> Result<()>;

    pub type SkippableBodyChunkReadFn<T, R, I, N> =
        fn(n: &mut T, d: &mut Deserializer<Take<&mut R>, &I, &mut N>) -> Result<()>;

    pub trait Sealed {
        fn read(
            reader: impl Read + Seek,
            header_options: HeaderOptions,
            body_options: BodyOptions,
        ) -> Result<Self>
        where
            Self: Sized;
    }

    pub trait HeaderChunks {
        fn header_chunks<R: Read>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>>
        where
            Self: Sized;
    }

    pub trait BodyChunks {
        fn body_chunks<'a, R: Read, I: IdStateRef<'a>, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
        where
            Self: Sized;
    }
}
pub(crate) trait ReadBody {
    fn read_body<'a, R: Read, I: IdStateRef<'a>, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()>;
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
