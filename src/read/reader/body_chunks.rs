use crate::read::{BodyReader, Error, Result};

pub fn read_body_chunks<'a, R: BodyReader, T>(
    r: &'a mut R,
    read_fn: impl FnOnce(&mut BodyChunksReader<'a, R>) -> Result<T>,
) -> Result<T> {
    let mut r = BodyChunksReader::new(r);

    let chunks = read_fn(&mut r)?;

    let chunk_id = r.chunk_id()?;

    if chunk_id != 0xfacade01 {
        return Err(Error::Internal(
            format!("expected end-of-node marker, got {chunk_id:08x}").into(),
        ));
    }

    Ok(chunks)
}

pub struct BodyChunksReader<'a, R> {
    inner: &'a mut R,
    chunk_id: Option<u32>,
}

impl<'a, R> BodyChunksReader<'a, R> {
    pub fn new(inner: &'a mut R) -> Self {
        Self {
            inner,
            chunk_id: None,
        }
    }
}

impl<'a, R: BodyReader> BodyChunksReader<'a, R> {
    fn chunk_id(&mut self) -> Result<u32> {
        match self.chunk_id {
            None => self.inner.u32(),
            Some(chunk_id) => {
                self.chunk_id = None;

                Ok(chunk_id)
            }
        }
    }

    pub fn chunk<T>(&mut self, id: u32, read_fn: impl FnOnce(&mut R) -> Result<T>) -> Result<T> {
        let chunk_id = self.chunk_id()?;

        println!("{:08x?}", chunk_id);

        if chunk_id != id {
            return Err(Error::Internal(
                format!("expected chunk with id {id:08x}, got {chunk_id:08x}").into(),
            ));
        }

        read_fn(self.inner)
    }

    pub fn chunk_skippable<T>(
        &mut self,
        id: u32,
        read_fn: impl FnOnce(&mut R) -> Result<T>,
    ) -> Result<T> {
        let chunk_id = self.chunk_id()?;

        println!("{:08x?}", chunk_id);

        if chunk_id != id {
            return Err(Error::Internal(
                format!("expected chunk with id {id:08x}, got {chunk_id:08x}").into(),
            ));
        }

        if self.inner.u32()? != 0x534b4950 {
            return Err(Error::Internal("expected a skippable chunk".into()));
        }

        let size = self.inner.u32()?;

        read_fn(self.inner)
    }

    pub fn chunk_optional<T>(
        &mut self,
        id: u32,
        read_fn: impl FnOnce(&mut R) -> Result<T>,
    ) -> Result<Option<T>> {
        let chunk_id = self.chunk_id()?;

        println!("{:08x?}", chunk_id);

        if chunk_id != id {
            self.chunk_id = Some(chunk_id);

            return Ok(None);
        }

        let chunk = read_fn(self.inner)?;

        Ok(Some(chunk))
    }

    pub fn chunk_skippable_optional<T>(
        &mut self,
        id: u32,
        read_fn: impl FnOnce(&mut R) -> Result<T>,
    ) -> Result<Option<T>> {
        let chunk_id = self.chunk_id()?;

        println!("{:08x?}", chunk_id);

        if chunk_id != id {
            self.chunk_id = Some(chunk_id);

            return Ok(None);
        }

        if self.inner.u32()? != 0x534b4950 {
            return Err(Error::Internal("expected a skippable chunk".into()));
        }

        let size = self.inner.u32()?;

        let chunk = read_fn(self.inner)?;

        Ok(Some(chunk))
    }
}
