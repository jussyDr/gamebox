//! Reading GameBox files.

pub mod byte_order;
pub mod reader;

mod body;
mod header_data;

pub use body::{BodyChunk, BodyChunks, ReadBody, read_body_chunks, read_node_from_body};
pub use header_data::{HeaderChunk, HeaderChunks};

use std::{
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    ClassId, ExternalNodeRef, FILE_SIGNATURE, FILE_VERSION, SubExtensions,
    read::{
        header_data::read_header_data,
        reader::{BR, IdTable, NodeTable, Reader},
    },
    sub_extension,
};

/// An error that occured while reading.
#[derive(Debug)]
pub struct Error {
    message: String,
    context: Option<Box<Error>>,
}

impl Error {
    pub(crate) fn new(message: impl Into<String>) -> Error {
        Error {
            message: message.into(),
            context: None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)?;

        if let Some(ref context) = self.context {
            Display::fmt(context, f)?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {}

fn map_io_error(io_error: io::Error) -> Error {
    Error::new(format!("IO error: {io_error}"))
}

/// Trait implemented by types that are readable from a GameBox file.
pub trait Readable:
    Default + Send + Sync + SubExtensions + ClassId + HeaderChunks + ReadBody
{
}

/// Read an instance of type `T` from the given `reader`.
pub fn read<T: Readable>(reader: impl Read) -> Result<T, Error> {
    let mut r = reader;

    // Read the header.
    if r.byte_array()? != FILE_SIGNATURE {
        return Err(Error::new("unknown file signature"));
    }

    if r.u16()? != FILE_VERSION {
        return Err(Error::new("unknown file version"));
    }

    if r.u8()? != b'B' {
        return Err(Error::new("unknown file format"));
    }

    if r.u8()? != b'U' {
        return Err(Error::new(
            "unknown external reference table compression format",
        ));
    }

    let body_compressed = match r.u8()? {
        b'C' => true,
        b'U' => false,
        _ => return Err(Error::new("unknown body compression format")),
    };

    if r.u8()? != b'R' {
        return Err(Error::new("unknown file format"));
    }

    let class_id = r.u32()?;

    if class_id != T::CLASS_ID {
        return Err(Error::new(format!(
            "class id does not match: expected 0x{:08x} but was 0x{:08x}",
            T::CLASS_ID,
            class_id
        )));
    }

    let header_data_size = r.u32()?;

    let mut node = T::default();

    if header_data_size > 0 {
        read_header_data(&mut node, &mut r)?;
    }

    let num_nodes = r
        .u32()?
        .checked_sub(1)
        .ok_or_else(|| Error::new("number of nodes is zero"))?;

    // Read the reference table.
    let num_external_nodes = r.u32()?;

    let mut node_table = NodeTable::new(num_nodes as usize);

    if num_external_nodes != 0 {
        let ancestor_level = r.u32()?;
        let folders = read_folders(&mut r)?;

        for _ in 0..num_external_nodes {
            let _flags = r.u32()?;
            let file_name = r.string()?;
            let node_index = r
                .u32()?
                .checked_sub(1)
                .ok_or_else(|| Error::new("node index is zero"))?;
            let _use_file = r.bool32()?;
            let folder_index = r.u32()?;

            let mut path = folders
                .get(folder_index as usize)
                .ok_or_else(|| Error::new("folder index exceeds number of folders"))?
                .clone();

            path.push(&file_name);

            node_table.set_external(
                node_index,
                ExternalNodeRef {
                    path: Arc::from(path),
                    ancestor_level,
                },
            )?;
        }
    }

    // Read the body.

    if body_compressed {
        let size = r.u32()?;
        let compressed_body = r.byte_buf()?;

        let mut body = vec![0; size as usize];
        lzo1x::decompress(&compressed_body, &mut body)
            .map_err(|_| Error::new("failed to decompress body"))?;

        let mut r = BR {
            reader: body.as_slice(),
            id_table: IdTable::new(),
            node_table,
        };

        node.read_body(&mut r)?;

        r.expect_eof()?;
    } else {
        let mut r = BR {
            reader: r,
            id_table: IdTable::new(),
            node_table,
        };

        node.read_body(&mut r)?;

        r.expect_eof()?;
    }

    Ok(node)
}

/// Read an instance of type `T` from a file at the given `path`.
pub fn read_file<T: Readable + SubExtensions>(path: impl AsRef<Path>) -> Result<T, Error> {
    let path = path.as_ref();
    let sub_extension = sub_extension(path).unwrap();

    if !T::has_sub_extension(sub_extension) {
        todo!("{}", sub_extension)
    }

    let file = File::open(path).map_err(map_io_error)?;
    let reader = BufReader::new(file);

    read(reader)
}

fn read_folders(r: &mut impl Reader) -> Result<Vec<PathBuf>, Error> {
    let mut folders = vec![];
    folders.push(PathBuf::new());

    let num_folders = r.u32()?;

    for _ in 0..num_folders {
        let name = r.string()?;
        let sub_folders = read_folders(r)?;

        for sub_folder in sub_folders {
            let mut folder = PathBuf::from(name.clone());
            folder.push(sub_folder);
            folders.push(folder);
        }
    }

    Ok(folders)
}

pub(crate) fn error_unknown_version(name: &str, version: u32) -> Error {
    Error::new(format!("unknown {name} version: {version}"))
}

pub(crate) fn error_unknown_chunk_version(version: u32) -> Error {
    error_unknown_version("chunk", version)
}

pub(crate) fn error_unknown_enum_variant(name: &str, value: u32) -> Error {
    Error::new(format!("unknown variant of enum `{name}`: {value}"))
}
