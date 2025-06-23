//! Reading GameBox files.

pub mod reader;

use std::{
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    Class, END_OF_BODY_MARKER, ExternalNodeRef, FILE_SIGNATURE, FILE_VERSION,
    SKIPPABLE_CHUNK_MARKER,
    read::reader::{IdTable, IdTableRef, NodeTable, NodeTableRef, Reader},
};

/// Read a node of type `T` from the given `reader`.
pub fn read<T: Readable>(reader: impl Read) -> Result<T, Error> {
    let mut r = Reader::new(reader, (), ());

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
        let header_data = r.bytes(header_data_size as usize)?;
    }

    let num_nodes = r
        .u32()?
        .checked_sub(1)
        .ok_or(Error("number of nodes is zero".into()))?;

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

    let mut r = Reader::new(r.into_inner(), IdTable::new(), node_table);

    if body_compressed {
        todo!()
    } else {
        node.read_body(&mut r)?;

        r.expect_eof()?;
    }

    Ok(T::default())
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

/// Read a node of type `T` from a file at the given `path`.
pub fn read_file<T: Readable>(path: impl AsRef<Path>) -> Result<T, Error> {
    let file = File::open(path).map_err(|_| Error("".into()))?;
    let reader = BufReader::new(file);

    read(reader)
}

/// Error that occured while reading.
#[derive(Debug)]
pub struct Error(pub String);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

pub trait Readable: Default + Class + ReadBody {}

pub trait ReadBody {
    fn read_body(
        &mut self,
        r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
    ) -> Result<(), Error>;
}

pub fn read_body_chunks<T: BodyChunks>(
    r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
    node: &mut T,
) -> Result<(), Error> {
    let chunk_id = read_body_chunks_inner(r, node)?;

    if let Some(chunk_id) = chunk_id {
        return Err(Error(format!("unknown chunk: 0x{chunk_id:08x}")));
    }

    Ok(())
}

fn read_body_chunks_inner<T: BodyChunks>(
    r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
    node: &mut T,
) -> Result<Option<u32>, Error> {
    let mut chunk_id = match node.parent() {
        None => r.u32()?,
        Some(parent) => match read_body_chunks_inner(r, parent)? {
            None => return Ok(None),
            Some(chunk_id) => chunk_id,
        },
    };

    let mut chunks = T::body_chunks().into_iter();

    loop {
        if chunk_id == END_OF_BODY_MARKER {
            break;
        }

        let chunk = match chunks.find(|chunk| chunk.id == chunk_id) {
            None => return Ok(Some(chunk_id)),
            Some(chunk) => chunk,
        };

        if chunk.skippable {
            if r.u32()? != SKIPPABLE_CHUNK_MARKER {
                todo!()
            }

            let size = r.u32()?;
        }

        (chunk.read_fn)(node, r)?;

        chunk_id = r.u32()?;
    }

    Ok(None)
}

pub trait BodyChunks {
    type Parent: BodyChunks;

    fn parent(&mut self) -> Option<&mut Self::Parent>;

    fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
    -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>>;
}

pub struct BodyChunk<T: ?Sized, R, I, N> {
    id: u32,
    read_fn: fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>,
    skippable: bool,
}

impl<T, R, I, N> BodyChunk<T, R, I, N> {
    pub fn new(id: u32, read_fn: fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>) -> Self {
        Self {
            id,
            read_fn,
            skippable: false,
        }
    }

    pub fn skippable(
        id: u32,
        read_fn: fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>,
    ) -> Self {
        Self {
            id,
            read_fn,
            skippable: true,
        }
    }
}

pub fn read_node<T: Default + Class + ReadBody>(
    r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
) -> Result<T, Error> {
    let mut node = T::default();
    node.read_body(r)?;
    Ok(node)
}
