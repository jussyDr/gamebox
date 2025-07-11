use std::io::Read;

use crate::{
    ClassId, END_OF_BODY_MARKER, NULL, SKIPPABLE_CHUNK_MARKER,
    read::{
        ChunkId, Error, HeaderReader, IdTable, NodeRefTable, ReadNodeRef,
        id::{TryFromId, read_id},
        node_ref::read_node_ref,
    },
};

/// Read body.
pub trait ReadBody {
    /// Read body.
    fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error>;
}

/// Body chunks.
pub trait BodyChunks: ClassId {
    /// Parent.
    fn parent(&mut self) -> Option<&mut impl BodyChunks>
    where
        Self: Sized,
    {
        None::<&mut Self>
    }

    /// Body chunks.
    fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>>;
}

/// Body chunk.
pub struct BodyChunk<T: ?Sized, R> {
    num: u16,
    read_fn: BodyChunkReadFn<T, R>,
    skippable: bool,
}

impl<T, R> BodyChunk<T, R> {
    /// New.
    pub fn new(num: u16, read_fn: BodyChunkReadFn<T, R>) -> Self {
        Self {
            num,
            read_fn,
            skippable: false,
        }
    }

    /// Skippable.
    pub fn skippable(num: u16, read_fn: BodyChunkReadFn<T, R>) -> Self {
        Self {
            num,
            read_fn,
            skippable: true,
        }
    }
}

type BodyChunkReadFn<T, R> = fn(&mut T, &mut R) -> Result<(), Error>;

/// Read body chunks.
pub fn read_body_chunks<T: BodyChunks>(r: &mut impl BodyReader, node: &mut T) -> Result<(), Error> {
    let chunk_id = read_body_chunks_inner(r, node)?;

    if let Some(chunk_id) = chunk_id {
        return Err(Error::new(format!("unknown chunk: 0x{chunk_id:08x}")));
    }

    Ok(())
}

fn read_body_chunks_inner<T: BodyChunks>(
    r: &mut impl BodyReader,
    node: &mut T,
) -> Result<Option<u32>, Error> {
    // Read parent chunks, if any.
    let mut chunk_id_or = match node.parent() {
        None => r.u32()?,
        Some(parent) => match read_body_chunks_inner(r, parent)? {
            None => return Ok(None),
            Some(chunk_id) => chunk_id,
        },
    };

    // Read chunks until either an end marker is reached, or a chunk ID of a different class encountered.
    let mut chunks = T::body_chunks().into_iter();

    loop {
        if chunk_id_or == END_OF_BODY_MARKER {
            break;
        }

        let chunk_id = ChunkId(chunk_id_or);
        let class_id = chunk_id.class_id();

        if class_id != T::CLASS_ID {
            return Ok(Some(chunk_id.0));
        }

        let chunk_num = chunk_id.num();

        let chunk = match chunks.find(|chunk| chunk.num == chunk_num) {
            None => {
                return Err(Error::new(format!(
                    "unknown chunk number: {chunk_num} of class 0x{class_id:08x}"
                )));
            }
            Some(chunk) => chunk,
        };

        if chunk.skippable {
            if r.u32()? != SKIPPABLE_CHUNK_MARKER {
                return Err(Error::new("expected a skippable chunk"));
            }

            let _size = r.u32()?;

            (chunk.read_fn)(node, r)?;
        } else {
            (chunk.read_fn)(node, r)?;
        }

        chunk_id_or = r.u32()?;
    }

    Ok(None)
}

/// Read node from body.
pub fn read_node_from_body<T: Default + ReadBody>(r: &mut impl BodyReader) -> Result<T, Error> {
    let mut node = T::default();
    node.read_body(r)?;

    Ok(node)
}

/// Body reader.
pub trait BodyReader: HeaderReader {
    /// Read a node reference.
    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<T, Error>
    where
        Self: Sized;

    /// Read a node.
    fn node<T: Default + ClassId + ReadBody>(&mut self) -> Result<T, Error>
    where
        Self: Sized,
    {
        let node = self.node_generic(|r, class_id| {
            if class_id != T::CLASS_ID {
                todo!("{:08X?}", class_id);
            }

            let mut node = T::default();
            node.read_body(r)?;

            Ok(node)
        })?;

        Ok(node)
    }

    /// Read a node.
    fn node_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let node = self.node_or_null_generic(read_fn)?;

        match node {
            None => Err(Error::new("node is null")),
            Some(node) => Ok(node),
        }
    }

    /// Read a node.
    fn node_or_null_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<Option<T>, Error> {
        let class_id = self.u32()?;

        if class_id == NULL {
            return Ok(None);
        }

        let node = read_fn(self, class_id)?;

        Ok(Some(node))
    }
}

/// Implementation of the `BodyReader` trait.
pub struct BodyReaderImpl<'n, R> {
    /// Reader.
    pub reader: R,
    /// Identifier table.
    pub id_table: IdTable,
    /// Node table.
    pub node_table: &'n NodeRefTable, // Needs to be a reference because of borrow checker magic.
}

impl<'n, R: Read> Read for BodyReaderImpl<'n, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<'n, R: Read> HeaderReader for BodyReaderImpl<'n, R> {
    fn id<T: TryFromId>(&mut self) -> Result<T, Error> {
        read_id(&mut self.reader, &mut self.id_table)
    }
}

impl<'n, R: Read> BodyReader for BodyReaderImpl<'n, R> {
    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<T, Error> {
        read_node_ref(self)
    }
}
