//! Reading GameBox files.

pub(crate) mod readable;
pub(crate) mod reader;

mod error;

pub use error::{Error, ErrorKind, TraceEntry};

pub(crate) use readable::{read_body_chunks, BodyChunk, BodyChunks, ReadBody};

use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek},
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};

use readable::HeaderChunks;
use reader::{IdState, IdStateMut, NodeState, Reader};

use crate::{ExternalNodeRef, FILE_SIGNATURE, HEAVY_CHUNK_MARKER_BIT};

/// A readable class.
pub trait Readable: readable::Sealed {}

/// Read settings.
pub struct Settings {
    skip_heavy_header_chunks: bool,
    skip_body: bool,
}

impl Settings {
    /// New.
    pub const fn new() -> Self {
        Self {
            skip_heavy_header_chunks: false,
            skip_body: false,
        }
    }

    /// Read body.
    pub const fn skip_body(mut self, skip_body: bool) -> Self {
        self.skip_body = skip_body;

        self
    }

    /// Read a node from the given `reader`.
    pub fn read<T: Readable>(&self, reader: impl Read + Seek) -> Result<T, Error> {
        let mut r = Reader::new(reader, (), ());

        let signature = r.byte_array()?;

        if signature != FILE_SIGNATURE {
            return Err(Error::new(ErrorKind::Format(
                "invalid file signature".into(),
            )));
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
            return Err(Error::new(ErrorKind::Format(format!(
                "expected class id {:08X}, got {:08X}",
                T::CLASS_ID,
                class_id
            ))));
        }

        let mut node = T::default();

        let header_data_size = r.u32()?;

        if header_data_size != 0 {
            let header_data = r.bytes(header_data_size as usize)?;

            let mut r = Reader::new(Cursor::new(header_data), IdState::new(), ());

            let header_chunks = r.list(|r| {
                let chunk_id = r.u32()?;
                let chunk_size = r.u32()?;

                Ok((chunk_id, chunk_size))
            })?;

            let rem = self.read_header_chunks(&mut node, &mut r, &header_chunks)?;

            if let Some((chunk_id, _)) = rem.first() {
                return Err(Error::new(ErrorKind::Unsupported(format!(
                    "header chunk: {chunk_id:08X?}"
                ))));
            }
        }

        if !self.skip_body {
            let num_nodes = r.u32()?;
            let num_node_refs = num_nodes
                .checked_sub(1)
                .ok_or(Error::new(ErrorKind::Format("index".into())))?;
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
                        .ok_or(Error::new(ErrorKind::Format("index".into())))?;

                    let use_file = r.bool()?;
                    let folder_index = r.u32()?;
                    let mut path = folders
                        .get(folder_index as usize)
                        .ok_or(Error::new(ErrorKind::Format("index".into())))?
                        .clone();

                    path.push(file_name);

                    // println!("{node_index}, {path:?}");

                    node_state.set_external_node_ref(
                        node_ref_index as usize,
                        ExternalNodeRef {
                            ancestor_level,
                            use_file,
                            path: path.into(),
                            phantom: PhantomData,
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
                        .map_err(|_| Error::new(ErrorKind::Format("decompress".into())))?;

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
        }

        Ok(node)
    }

    /// Read a node from a file at the given `path`.
    pub fn read_file<T: Readable>(&self, path: impl AsRef<Path>) -> Result<T, Error> {
        let file = File::open(path).map_err(Error::io)?;
        let reader = BufReader::new(file);

        self.read(reader)
    }

    fn read_header_chunks<'a, T: HeaderChunks, N>(
        &self,
        node: &mut T,
        r: &mut Reader<impl Read + Seek, impl IdStateMut, N>,
        header_chunks: &'a [(u32, u32)],
    ) -> Result<&'a [(u32, u32)], Error> {
        let mut header_chunks = match node.parent() {
            Some(parent) => self.read_header_chunks(parent, r, header_chunks)?,
            None => header_chunks,
        };

        let mut h = T::header_chunks();

        while let Some((chunk_id, chunk_size)) = header_chunks.first() {
            let is_heavy = chunk_size & HEAVY_CHUNK_MARKER_BIT != 0;
            let chunk_size = chunk_size & 0x7fffffff;

            let class_id = chunk_id & 0xfffff000;

            if class_id != T::CLASS_ID {
                break;
            }

            let chunk_num = (chunk_id & 0x00000fff) as u16;

            // println!("{:08X?}, {}", class_id, chunk_num);

            let chunk = h
                .find(|header_chunk| header_chunk.num == chunk_num)
                .ok_or_else(|| {
                    Error::new(ErrorKind::Unsupported(format!(
                        "header chunk: {chunk_id:08X?}"
                    )))
                })?;

            if self.skip_heavy_header_chunks && is_heavy {
                r.skip(chunk_size as u64)?;
            } else {
                (chunk.read_fn)(node, r)?;
            }

            header_chunks = &header_chunks[1..];
        }

        Ok(header_chunks)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
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

/// Read a node from the given `reader`.
pub fn read<T: Readable>(reader: impl Read + Seek) -> Result<T, Error> {
    Settings::default().read(reader)
}

/// Read a node from a file at the given `path`.
pub fn read_file<T: Readable>(path: impl AsRef<Path>) -> Result<T, Error> {
    Settings::default().read_file(path)
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
