use crate::read::{Reader, Result, reader::header::HeaderReaderImpl};

pub fn read_header_chunks<T>(
    mut r: &[u8],
    read_fn: impl FnOnce(&mut HeaderChunksReader) -> Result<T>,
) -> Result<T> {
    let chunks = if r.is_empty() {
        vec![].into_boxed_slice()
    } else {
        r.list(|r| {
            let id = r.u32()?;
            let size = r.u32()?;

            Ok((id, size))
        })?
    };

    let mut r = HeaderChunksReader {
        inner: HeaderReaderImpl::new(r),
        chunks,
        current_chunk_index: 0,
    };

    let chunks = read_fn(&mut r)?;

    if let Some((id, _)) = r.chunk_id() {
        todo!("{id:08x}")
    }

    Ok(chunks)
}

pub struct HeaderChunksReader<'a> {
    inner: HeaderReaderImpl<&'a [u8]>,
    chunks: Box<[(u32, u32)]>,
    current_chunk_index: usize,
}

impl HeaderChunksReader<'_> {
    fn chunk_id(&mut self) -> Option<(u32, u32)> {
        self.chunks.get(self.current_chunk_index).cloned()
    }

    pub fn chunk<T>(
        &mut self,
        id: u32,
        read_fn: impl FnOnce(&mut HeaderReaderImpl<&[u8]>) -> Result<T>,
    ) -> Result<T> {
        if self.chunk_id().unwrap().0 != id {
            todo!()
        }

        self.current_chunk_index += 1;

        read_fn(&mut self.inner)
    }

    pub fn chunk_optional<T>(
        &mut self,
        id: u32,
        read_fn: impl FnOnce(&mut HeaderReaderImpl<&[u8]>) -> Result<T>,
    ) -> Result<Option<T>> {
        if self.chunk_id().unwrap().0 != id {
            return Ok(None);
        }

        self.current_chunk_index += 1;

        let chunk = read_fn(&mut self.inner)?;

        Ok(Some(chunk))
    }
}
