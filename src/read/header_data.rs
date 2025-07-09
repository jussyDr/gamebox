use crate::{
    ClassId,
    read::{
        Error,
        reader::{HeaderReader, HeaderReaderImpl, IdTable, Reader},
    },
};

/// Header chunks.
pub trait HeaderChunks: ClassId {
    /// Header chunks.
    fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>>;
}

/// Header chunk.
pub struct HeaderChunk<T: ?Sized, R> {
    num: u8,
    read_fn: HeaderChunkReadFn<T, R>,
}

impl<T, R> HeaderChunk<T, R> {
    /// Create a new header chunk.
    pub fn new(num: u8, read_fn: HeaderChunkReadFn<T, R>) -> Self {
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
        let class_id = chunk_id & 0xffffff00;

        if class_id != T::CLASS_ID {
            todo!()
        }

        let chunk_num = (chunk_id & 0x000000ff) as u8;

        let chunk = header_chunks
            .find(|chunk| chunk.num == chunk_num)
            .ok_or_else(|| Error::new(format!("unknown header chunk: 0x{chunk_id:08x}")))?;

        (chunk.read_fn)(node, &mut r)?;
    }

    Ok(())
}
