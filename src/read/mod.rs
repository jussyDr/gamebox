//! Reading GameBox files.

pub mod reader;

use std::{
    collections::VecDeque,
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use reader::{ExternalNodeRef, IdState, IdStateMut, NodeState, NodeStateMut, Reader};

use crate::Class;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    trace: VecDeque<TraceEntry>,
}

impl Error {
    pub const fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            trace: VecDeque::new(),
        }
    }

    pub fn io(io_error: io::Error) -> Self {
        let kind = match io_error.kind() {
            io::ErrorKind::UnexpectedEof => ErrorKind::Format("unexpected EOF"),
            _ => ErrorKind::Io(io_error),
        };

        Self {
            kind,
            trace: VecDeque::new(),
        }
    }

    pub fn version(name: &str, version: u32) -> Self {
        Self {
            kind: ErrorKind::Unsupported(format!("{name} version: {version}")),
            trace: VecDeque::new(),
        }
    }

    pub fn chunk_version(version: u32) -> Self {
        Self::version("chunk", version)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    Unsupported(String),
    Format(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Error")
    }
}

impl std::error::Error for Error {}

pub struct TraceEntry {
    class_id: u32,
    chunk_num: Option<u16>,
}

impl Debug for TraceEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TraceEntry {{ class_id: 0x{:08x} }}", self.class_id)
    }
}

/// Read a node of type `T` from the given `reader`.
pub fn read<T: Readable>(reader: impl Read) -> Result<T, Error> {
    let mut r = Reader::new(reader, (), ());

    let signature = r.byte_array()?;

    if signature != [b'G', b'B', b'X'] {
        return Err(Error::new(ErrorKind::Format("invalid file signature")));
    }

    let version = r.u16()?;

    if version != 6 {
        return Err(Error::version("file", version as u32));
    }

    let format = r.u8()?;

    if format != b'B' {
        return Err(Error::new(ErrorKind::Unsupported(
            "file format".to_string(),
        )));
    }

    let ref_table_compression = r.u8()?;

    if ref_table_compression != b'U' {
        return Err(Error::new(ErrorKind::Unsupported(
            "reference table compression".to_string(),
        )));
    }

    let body_compression = Compression::read(&mut r)?;

    if r.u8()? != b'R' {
        return Err(Error::new(ErrorKind::Unsupported("".to_string())));
    }

    let class_id = r.u32()?;

    if class_id != T::CLASS_ID {
        return Err(Error::new(ErrorKind::Format("class id does not match")));
    }

    let mut node = T::default();

    let header_data = r.byte_buf()?;

    if !header_data.is_empty() {
        // TODO
    }

    let num_nodes = r.u32()?;
    let num_node_refs = num_nodes
        .checked_sub(1)
        .ok_or(Error::new(ErrorKind::Format("index")))?;
    let mut node_state = NodeState::new(num_node_refs as usize);

    let num_external_node_refs = r.u32()?;

    if num_external_node_refs > 0 {
        let ancestor_level = r.u32()? as u8;
        let folders = read_folders(&mut r)?;

        for _ in 0..num_external_node_refs {
            let flags = r.u32()?;

            if flags & 0x00000004 != 0 {
                return Err(Error::new(ErrorKind::Unsupported(
                    "reference table".to_string(),
                )));
            }

            let file_name = r.string()?;

            let node_index = r.u32()?;
            let node_ref_index = node_index
                .checked_sub(1)
                .ok_or(Error::new(ErrorKind::Format("index")))?;

            let use_file = r.bool()?;
            let folder_index = r.u32()?;

            let mut path = folders
                .get(folder_index as usize)
                .ok_or(Error::new(ErrorKind::Format("index")))?
                .clone();

            path.push(file_name);

            node_state.set_external_node_ref(
                node_ref_index as usize,
                ExternalNodeRef {
                    path: path.into(),
                    ancestor_level,
                },
            )?;
        }
    }

    let id_state = IdState::new();

    match body_compression {
        Compression::Compressed => {
            let body_size = r.u32()?;
            let compressed_body = r.byte_buf()?;
            r.expect_eof()?;

            let mut body = vec![0; body_size as usize];
            lzo1x::decompress(&compressed_body, &mut body)
                .map_err(|_| Error::new(ErrorKind::Format("decompress")))?;

            let mut r = Reader::new(body.as_slice(), id_state, node_state);

            match node.read_body(&mut r) {
                Ok(()) => {}
                Err(mut error) => {
                    error.trace.push_front(TraceEntry {
                        class_id: T::CLASS_ID,
                        chunk_num: None,
                    });

                    return Err(error);
                }
            }

            r.expect_eof()?;
        }
        Compression::Uncompressed => {
            let mut r = Reader::new(r.into_inner(), id_state, node_state);

            match node.read_body(&mut r) {
                Ok(()) => {}
                Err(mut error) => {
                    error.trace.push_front(TraceEntry {
                        class_id: T::CLASS_ID,
                        chunk_num: None,
                    });

                    return Err(error);
                }
            }

            r.expect_eof()?;
        }
    }

    Ok(node)
}

/// Read a node of type `T` from a file at the given `path`.
pub fn read_file<T: Readable>(path: impl AsRef<Path>) -> Result<T, Error> {
    let file = File::open(path).map_err(Error::io)?;
    let reader = BufReader::new(file);

    read(reader)
}

fn read_folders<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<Vec<PathBuf>, Error> {
    let path = PathBuf::new();

    let mut folders = vec![];
    folders.push(path.clone());

    read_folders_inner(r, path, &mut folders)?;

    Ok(folders)
}

fn read_folders_inner<R: Read, I, N>(
    r: &mut Reader<R, I, N>,
    path: PathBuf,
    folders: &mut Vec<PathBuf>,
) -> Result<(), Error> {
    let num_sub_folders = r.u32()?;

    for _ in 0..num_sub_folders {
        let name = Arc::from(PathBuf::from(r.string()?));

        let mut path = path.clone();
        path.push(name);

        folders.push(path.clone());

        read_folders_inner(r, path, folders)?;
    }

    Ok(())
}

pub trait Readable: Sealed {}

pub trait Sealed: Class + ReadBody {}

pub trait ReadBody: Send + Sync + Default {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<(), Error>;

    fn read_from_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        r: &mut Reader<R, I, N>,
    ) -> Result<Self, Error> {
        let mut node = Self::default();
        node.read_body(r)?;

        Ok(node)
    }
}

pub fn read_body_chunks<T: Class + BodyChunks>(
    node: &mut T,
    r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
) -> Result<(), Error> {
    let chunk_id = read_body_chunks_inner(node, r)?;

    if chunk_id != 0xfacade01 {
        return Err(Error::new(ErrorKind::Unsupported(format!(
            "chunk: {chunk_id:08X?}"
        ))));
    }

    Ok(())
}

fn read_body_chunks_inner<T: Class + BodyChunks>(
    node: &mut T,
    r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
) -> Result<u32, Error> {
    let mut chunk_id = match node.parent() {
        Some(parent) => read_body_chunks_inner(parent, r)?,
        None => r.u32()?,
    };

    let mut chunks = T::body_chunks();

    loop {
        if chunk_id == 0xfacade01 {
            break;
        }

        let class_id = chunk_id & 0xfffff000;

        if class_id != T::CLASS_ID {
            break;
        }

        let chunk_num = (chunk_id & 0x00000fff) as u16;

        let chunk = chunks
            .find(|chunk| chunk.num == chunk_num)
            .ok_or(Error::new(ErrorKind::Unsupported(format!(
                "chunk: {chunk_num}"
            ))))?;

        match chunk.read_fn {
            BodyChunkReadFn::Normal(read_fn) => {
                read_fn(node, r)?;
            }
            BodyChunkReadFn::Skippable(read_fn) => {
                if r.u32()? != 0x534B4950 {
                    return Err(Error::new(ErrorKind::Format("skip")));
                }

                let size = r.u32()?;

                read_fn(node, r)?;
            }
        }

        chunk_id = r.u32()?;
    }

    Ok(chunk_id)
}

pub trait BodyChunks: Sized + Class {
    fn parent(&mut self) -> Option<&mut impl BodyChunks> {
        None::<&mut Self>
    }

    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>>;
}

pub struct BodyChunk<T, R, I, N> {
    num: u16,
    read_fn: BodyChunkReadFn<T, R, I, N>,
}

pub enum BodyChunkReadFn<T, R, I, N> {
    Normal(BodyChunkReadFnNormal<T, R, I, N>),
    Skippable(BodyChunkReadFnSkippable<T, R, I, N>),
}

pub type BodyChunkReadFnNormal<T, R, I, N> = fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>;

pub type BodyChunkReadFnSkippable<T, R, I, N> =
    fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>;

impl<T, R, I, N> BodyChunk<T, R, I, N> {
    pub const fn new(num: u16, read_fn: BodyChunkReadFnNormal<T, R, I, N>) -> Self {
        Self {
            num,
            read_fn: BodyChunkReadFn::Normal(read_fn),
        }
    }

    pub const fn skippable(num: u16, read_fn: BodyChunkReadFnSkippable<T, R, I, N>) -> Self {
        Self {
            num,
            read_fn: BodyChunkReadFn::Skippable(read_fn),
        }
    }
}

pub enum Compression {
    Compressed,
    Uncompressed,
}

impl Compression {
    pub fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        match r.u8()? {
            b'C' => Ok(Self::Compressed),
            b'U' => Ok(Self::Uncompressed),
            _ => Err(Error::new(ErrorKind::Unsupported(
                "compression".to_string(),
            ))),
        }
    }
}
