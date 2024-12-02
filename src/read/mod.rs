//! Reading GameBox files.

pub mod reader;

pub(crate) mod readable;

pub(crate) use readable::{read_body_chunks, BodyChunk, BodyChunks, ReadBody};

use std::{
    collections::VecDeque,
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, BufReader, Cursor, Read, Seek},
    path::{Path, PathBuf},
    sync::Arc,
};

use reader::{ExternalNodeRef, IdState, NodeState, Reader};

use crate::FILE_SIGNATURE;

/// A readable class.
pub trait Readable: readable::Sealed {}

/// Error while reading.
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

    pub fn enum_variant(name: &str, value: u32) -> Self {
        Self {
            kind: ErrorKind::Unsupported(format!("{name} variant: {value}")),
            trace: VecDeque::new(),
        }
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
pub fn read<T: Readable>(reader: impl Read + Seek) -> Result<T, Error> {
    let mut r = Reader::new(reader, (), ());

    let signature = r.byte_array()?;

    if signature != FILE_SIGNATURE {
        return Err(Error::new(ErrorKind::Format("invalid file signature")));
    }

    let version = r.u16()?;

    if version != 6 {
        return Err(Error::version("gamebox file", version as u32));
    }

    let format = r.enum_u8::<Format>()?;

    if !matches!(format, Format::Binary) {
        return Err(Error::new(ErrorKind::Unsupported(
            "file format".to_string(),
        )));
    }

    let ref_table_compression = r.enum_u8::<Compression>()?;

    if !matches!(ref_table_compression, Compression::Uncompressed) {
        return Err(Error::new(ErrorKind::Unsupported(
            "reference table compression".to_string(),
        )));
    }

    let body_compression = r.enum_u8::<Compression>()?;

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

            let mut r = Reader::new(Cursor::new(body), id_state, node_state);

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

enum Format {
    Binary,
    Text,
}

impl TryFrom<u8> for Format {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'B' => Ok(Self::Binary),
            b'T' => Ok(Self::Text),
            _ => Err(()),
        }
    }
}

enum Compression {
    Compressed,
    Uncompressed,
}

impl TryFrom<u8> for Compression {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'C' => Ok(Self::Compressed),
            b'U' => Ok(Self::Uncompressed),
            _ => Err(()),
        }
    }
}
