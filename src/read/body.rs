use std::{any::Any, cell::Cell, iter};

use elsa::FrozenVec;

use crate::{U8Vec3, U32Vec3, read::Error};

#[derive(Clone)]
pub struct BodyReader<'a> {
    data: &'a [u8],
    id_refs: &'a IdRefs<'a>,
    node_refs: &'a [Option<Box<dyn Any>>],
}

pub struct IdRefs<'a> {
    seen_id: Cell<bool>,
    ids: FrozenVec<&'a str>,
}

impl<'a> IdRefs<'a> {
    pub fn new() -> Self {
        Self {
            seen_id: Cell::new(false),
            ids: FrozenVec::new(),
        }
    }
}

impl<'a> BodyReader<'a> {
    pub fn new(
        data: &'a [u8],
        id_refs: &'a mut IdRefs<'a>,
        node_refs: &'a [Option<Box<dyn Any>>],
    ) -> Self {
        Self {
            data,
            id_refs,
            node_refs,
        }
    }

    pub fn repeat_u8(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let (bytes, remaining_data) = self
            .data
            .split_at_checked(n)
            .ok_or_else(|| Error::new("unexpected EOF"))?;

        self.data = remaining_data;

        Ok(bytes)
    }

    fn u8_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let bytes = self.repeat_u8(N)?;

        Ok(bytes.try_into().expect("len is N"))
    }

    pub fn u8(&mut self) -> Result<u8, Error> {
        let bytes = self.u8_array()?;

        Ok(u8::from_le_bytes(bytes))
    }

    pub fn u32(&mut self) -> Result<u32, Error> {
        let bytes = self.u8_array()?;

        Ok(u32::from_le_bytes(bytes))
    }

    pub fn vec3_u8(&mut self) -> Result<U8Vec3, Error> {
        todo!()
    }

    pub fn vec3_u32(&mut self) -> Result<U32Vec3, Error> {
        todo!()
    }

    pub fn bool32(&mut self) -> Result<bool, Error> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::expected("a 32-bit boolean")),
        }
    }

    pub fn string(&mut self) -> Result<&'a str, Error> {
        let len = self.u32()?;
        let bytes = self.repeat_u8(len as usize)?;

        str::from_utf8(bytes).map_err(Error::new)
    }

    pub fn list<T>(
        &mut self,
        read_fn: impl FnMut(Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        self.repeat(len as usize, read_fn)
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        iter::repeat_with(|| read_fn(self.clone()))
            .take(n)
            .collect()
    }

    pub fn id(&mut self) -> Result<&'a str, Error> {
        match self.id_or_null()? {
            None => Err(Error::expected_non_null("identifier")),
            Some(id) => Ok(id),
        }
    }

    pub fn id_or_null(&mut self) -> Result<Option<&'a str>, Error> {
        if !self.id_refs.seen_id.get() {
            let version = self.u32()?;

            if version != 3 {
                return Err(Error::unknown_version("identifier", version));
            }

            self.id_refs.seen_id.set(true);
        }

        let index = self.u32()?;

        if index == 0xfffffff {
            return Ok(None);
        }

        if index & 0xc0000000 != 0x40000000 {
            return Err(Error::expected("an identifier"));
        }

        let index = index & 0x3fffffff;

        match index.checked_sub(1) {
            None => {
                let id = self.string()?;
                self.id_refs.ids.push(id);

                Ok(Some(id))
            }
            Some(index) => {
                let id = self
                    .id_refs
                    .ids
                    .get(index as usize)
                    .ok_or_else(|| Error::index_out_of_bounds("identifier"))?;

                Ok(Some(id))
            }
        }
    }

    pub fn node_ref<T: 'static>(&mut self) -> Result<&'a T, Error> {
        match self.node_ref_or_null()? {
            None => Err(Error::expected_non_null("node reference")),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn node_ref_or_null<T: 'static>(&mut self) -> Result<Option<&'a T>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or_else(|| Error::zero("node reference index"))?;

        let slot = self
            .node_refs
            .get(index as usize)
            .ok_or_else(|| Error::index_out_of_bounds("node reference"))?;

        match slot {
            None => {
                todo!()
            }
            Some(node_ref) => {
                let node_ref = node_ref
                    .downcast_ref()
                    .ok_or_else(|| Error::new("node reference type mismatch"))?;

                Ok(Some(node_ref))
            }
        }
    }
}

pub struct BodyChunksReader<'a> {
    inner: BodyReader<'a>,
    chunk_id: Option<u32>,
}

impl<'a> BodyChunksReader<'a> {
    pub fn new(inner: BodyReader<'a>) -> Self {
        Self {
            inner,
            chunk_id: None,
        }
    }

    pub fn chunk<T>(
        &mut self,
        id: u32,
        mut read_fn: impl FnMut(&mut BodyReader<'a>) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let chunk_id = match self.chunk_id {
            None => {
                let chunk_id = self.inner.u32()?;
                self.chunk_id = Some(chunk_id);
                chunk_id
            }
            Some(chunk_id) => chunk_id,
        };

        if chunk_id != id {
            return Err(Error::new("chunk id mismatch"));
        }

        read_fn(&mut self.inner)
    }

    pub fn skippable_chunk<T>(
        &mut self,
        id: u32,
        mut read_fn: impl FnMut(&mut BodyReader<'a>) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let chunk_id = match self.chunk_id {
            None => {
                let chunk_id = self.inner.u32()?;
                self.chunk_id = Some(chunk_id);
                chunk_id
            }
            Some(chunk_id) => chunk_id,
        };

        if chunk_id != id {
            return Err(Error::new("chunk id mismatch"));
        }

        if self.inner.u32()? != 0x534b4950 {
            todo!()
        }

        let _size = self.inner.u32()?;

        read_fn(&mut self.inner)
    }

    pub fn expect_end(&mut self) -> Result<(), Error> {
        todo!()
    }
}
