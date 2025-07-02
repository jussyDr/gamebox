use std::io::Read;

use crate::{
    ClassId,
    read::{Error, reader::Reader},
};

pub trait HeaderChunks: ClassId {
    fn header_chunks<R: Read, I, N>() -> impl IntoIterator<Item = HeaderChunk<Self, R, I, N>>;
}

pub struct HeaderChunk<T: ?Sized, R, I, N> {
    num: u8,
    read_fn: HeaderChunkReadFn<T, R, I, N>,
}

impl<T, R, I, N> HeaderChunk<T, R, I, N> {
    pub fn new(num: u8, read_fn: HeaderChunkReadFn<T, R, I, N>) -> Self {
        Self { num, read_fn }
    }
}

type HeaderChunkReadFn<T, R, I, N> = fn(&mut T, r: &mut Reader<R, I, N>) -> Result<(), Error>;

pub fn read_header_data<T: HeaderChunks, I, N>(
    node: &mut T,
    r: &mut Reader<impl Read, I, N>,
) -> Result<(), Error> {
    let header_chunk_entries = r.list(|r| {
        let chunk_id = r.u32()?;
        let chunk_size = r.u32()?;

        Ok((chunk_id, chunk_size))
    })?;

    let mut header_chunks = T::header_chunks().into_iter();

    for (chunk_id, chunk_size) in header_chunk_entries {
        let class_id = chunk_id & 0xffffff00;

        if class_id != T::CLASS_ID {
            todo!()
        }

        let chunk_num = (chunk_id & 0x000000ff) as u8;

        let chunk = header_chunks
            .find(|chunk| chunk.num == chunk_num)
            .ok_or(Error::new(format!(
                "unknown header chunk: 0x{chunk_id:08x}"
            )))?;

        (chunk.read_fn)(node, r)?;
    }

    Ok(())
}
