//! Reading GameBox files.

pub mod byte_order;
pub mod reader;

mod body;
mod header;

pub use body::{BodyChunk, BodyChunks, ReadBody, read_body_chunks, read_node_body};
pub use header::{HeaderChunk, HeaderChunks};

use std::{
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    ClassId, Extensions, ExternalNodeRef, FILE_SIGNATURE, FILE_VERSION, full_extension,
    read::{
        header::read_header_data,
        reader::{IdTable, NodeTable, Reader},
    },
};

/// An error that occured while reading.
#[derive(Debug)]
pub struct Error(pub String);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

/// Trait implemented by classes that are readable from a GameBox file.
pub trait Readable: Default + Send + Sync + Extensions + ClassId + HeaderChunks + ReadBody {}

/// Read a node of type `T` from the given `reader`.
pub fn read<T: Readable>(reader: impl Read) -> Result<T, Error> {
    let mut r = Reader::new(reader, (), ());

    // Read the header.
    if r.byte_array()? != FILE_SIGNATURE {
        return Err(Error("unknown file signature".into()));
    }

    if r.u16()? != FILE_VERSION {
        return Err(Error("unknown file version".into()));
    }

    if r.u8()? != b'B' {
        return Err(Error("unknown file format".into()));
    }

    if r.u8()? != b'U' {
        return Err(Error(
            "unknown external reference table compression format".into(),
        ));
    }

    let body_compressed = match r.u8()? {
        b'C' => true,
        b'U' => false,
        _ => return Err(Error("unknown body compression format".into())),
    };

    if r.u8()? != b'R' {
        return Err(Error("unknown file format".into()));
    }

    let class_id = r.u32()?;

    if class_id != T::CLASS_ID {
        return Err(Error(format!(
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
        .ok_or(Error("number of nodes is zero".into()))?;

    // Read the reference table.
    let num_external_nodes = r.u32()?;

    let mut node_table = NodeTable::new(num_nodes as usize);

    if num_external_nodes != 0 {
        let ancestor_level = r.u32()?;
        let folders = read_folders(&mut r)?;

        for _ in 0..num_external_nodes {
            let flags = r.u32()?;
            let file_name = r.string()?;
            let node_index = r
                .u32()?
                .checked_sub(1)
                .ok_or(Error("node index is zero".into()))?;
            let use_file = r.bool32()?;
            let folder_index = r.u32()?;

            let mut path = folders
                .get(folder_index as usize)
                .ok_or(Error("folder index exceeds number of folders".into()))?
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
    let mut r = Reader::new(r.into_inner(), IdTable::new(), node_table);

    if body_compressed {
        todo!()
    } else {
        node.read_body(&mut r)?;

        r.expect_eof()?;
    }

    Ok(node)
}

/// Read a node of type `T` from a file at the given `path`.
pub fn read_file<T: Readable + Extensions>(path: impl AsRef<Path>) -> Result<T, Error> {
    let path = path.as_ref();
    let file_extension = full_extension(path).unwrap();

    if !T::EXTENSIONS.contains(&file_extension) {
        todo!("{}", file_extension)
    }

    let file = File::open(path).map_err(|_| Error("".into()))?;
    let reader = BufReader::new(file);

    read(reader)
}

fn read_folders<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Vec<PathBuf>, Error> {
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

pub fn error_unknown_version(name: &str, value: u32) -> Error {
    Error(format!("unknown {name} version: {value}"))
}

pub fn error_unknown_chunk_version(value: u32) -> Error {
    error_unknown_version("chunk", value)
}

pub fn error_unknown_enum_variant(name: &str, value: u32) -> Error {
    Error(format!("unknown variant of enum `{name}`: {value}"))
}
