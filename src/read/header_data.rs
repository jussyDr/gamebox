use std::io::Read;

use crate::{
    ClassId,
    read::{
        ChunkId, Error, IdTable,
        id::{TryFromId, read_id},
        reader::Reader,
    },
};

/// Header data reader.
pub trait HeaderReader: Reader {
    /// Read an identifier.
    fn id<T: TryFromId>(&mut self) -> Result<T, Error>
    where
        Self: Sized;
}

/// Header chunks.
pub trait HeaderChunks: ClassId {
    /// Header chunks.
    fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>>;
}

/// Header chunk.
pub struct HeaderChunk<T: ?Sized, R> {
    num: u16,
    read_fn: HeaderChunkReadFn<T, R>,
}

impl<T, R> HeaderChunk<T, R> {
    /// Create a new header chunk.
    pub fn new(num: u16, read_fn: HeaderChunkReadFn<T, R>) -> Self {
        Self { num, read_fn }
    }
}

type HeaderChunkReadFn<T, R> = fn(&mut T, r: &mut R) -> Result<(), Error>;

pub fn read_header_data<T: HeaderChunks>(node: &mut T, r: &mut impl Reader) -> Result<(), Error> {
    let header_chunk_entries = r.list(|r| {
        let chunk_id = r.u32()?;
        let chunk_size = r.u32()?;

        Ok((chunk_id, chunk_size))
    })?;

    let mut header_chunks = T::header_chunks().into_iter();

    let mut r = HeaderReaderImpl {
        reader: r,
        id_table: IdTable::new(),
    };

    for (chunk_id, _chunk_size) in header_chunk_entries {
        let chunk_id = ChunkId(chunk_id);

        if chunk_id.class_id() != T::CLASS_ID {
            return Err(Error::new("class id does not match"));
        }

        let chunk_num = chunk_id.num();

        let chunk = header_chunks
            .find(|chunk| chunk.num == chunk_num)
            .ok_or_else(|| Error::new(format!("unknown header chunk number: {chunk_num}")))?;

        (chunk.read_fn)(node, &mut r)?;
    }

    Ok(())
}

struct HeaderReaderImpl<R> {
    reader: R,
    id_table: IdTable,
}

impl<R: Read> Read for HeaderReaderImpl<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read> HeaderReader for HeaderReaderImpl<R> {
    /// Read an identifier.
    fn id<T: TryFromId>(&mut self) -> Result<T, Error> {
        read_id(&mut self.reader, &mut self.id_table)
    }
}
