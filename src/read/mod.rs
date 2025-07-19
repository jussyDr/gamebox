mod error;

pub use error::Error;

use std::{
    any::Any,
    cell::OnceCell,
    fs::File,
    io::{BufReader, Read},
    iter,
    path::Path,
    sync::Arc,
};

pub fn read_file<T: Readable>(path: impl AsRef<Path>) -> Result<T, Error> {
    let file = File::open(path).map_err(Error::new)?;
    let reader = BufReader::new(file);

    read(reader)
}

pub fn read<T: Readable>(reader: impl Read) -> Result<T, Error> {
    let mut r = Reader(reader);

    if r.u8_array()? != [b'G', b'B', b'X'] {
        return Err(Error::new("unknown file signature"));
    }

    if r.u16()? != 6 {
        return Err(Error::new("unknown file version"));
    }

    if r.u8()? != b'B' {
        return Err(Error::new("unknown file format"));
    }

    if r.u8()? != b'U' {
        return Err(Error::new("unknown node reference table compression"));
    }

    let body_compressed = match r.u8()? {
        b'C' => true,
        b'U' => false,
        _ => return Err(Error::new("unknown body compression")),
    };

    if r.u8()? != b'R' {
        return Err(Error::new("unknown file format"));
    }

    let _class_id = r.u32()?;
    let header_data_size = r.u32()?;
    let header_data = r.repeat_u8(header_data_size as usize)?;
    let num_node_refs = r.u32()?;

    let node_refs = iter::repeat_with(OnceCell::new)
        .take(num_node_refs as usize)
        .collect();

    let num_external_node_refs = r.u32()?;

    if num_external_node_refs > 0 {
        todo!()
    }

    let body_data = if body_compressed {
        let body_data_size = r.u32()?;
        let compressed_body_data_size = r.u32()?;
        let compressed_body_data = r.repeat_u8(compressed_body_data_size as usize)?;

        let mut body_data = vec![0; body_data_size as usize].into_boxed_slice();
        lzo1x::decompress(&compressed_body_data, &mut body_data).map_err(Error::new)?;

        Arc::from(body_data)
    } else {
        todo!()
    };

    T::read_from_header_and_body(header_data, body_data, node_refs)
}

struct Reader<R>(R);

impl<R: Read> Reader<R> {
    fn repeat_u8(&mut self, n: usize) -> Result<Box<[u8]>, Error> {
        let mut bytes = vec![0; n].into_boxed_slice();
        self.0.read_exact(&mut bytes).map_err(Error::new)?;

        Ok(bytes)
    }

    fn u8_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut bytes = [0; N];
        self.0.read_exact(&mut bytes).map_err(Error::new)?;

        Ok(bytes)
    }

    fn u8(&mut self) -> Result<u8, Error> {
        let bytes = self.u8_array()?;

        Ok(u8::from_le_bytes(bytes))
    }

    fn u16(&mut self) -> Result<u16, Error> {
        let bytes = self.u8_array()?;

        Ok(u16::from_le_bytes(bytes))
    }

    fn u32(&mut self) -> Result<u32, Error> {
        let bytes = self.u8_array()?;

        Ok(u32::from_le_bytes(bytes))
    }
}

pub trait Readable {
    fn read_from_header_and_body(
        header_data: Box<[u8]>,
        body_data: Arc<[u8]>,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

pub struct BodyReader<'a, 'b> {
    data: Arc<[u8]>,
    node_refs: Arc<[OnceCell<Box<dyn Any>>]>,

    data_r: &'a [u8],
    node_refs_r: &'a [OnceCell<Box<dyn Any>>],

    pub data_offset: &'b mut usize,

    seen_id: bool,
    ids: Vec<&'a str>,
}

impl<'a, 'b> BodyReader<'a, 'b> {
    pub fn new(
        data: Arc<[u8]>,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
        data_r: &'a [u8],
        node_refs_r: &'a [OnceCell<Box<dyn Any>>],
        data_offset: &'b mut usize,
    ) -> Self {
        Self {
            data,
            node_refs,
            data_r,
            node_refs_r,
            data_offset,
            seen_id: false,
            ids: vec![],
        }
    }

    pub fn repeat_u8(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = *self.data_offset + n;

        let bytes = self
            .data_r
            .get(*self.data_offset..end)
            .ok_or_else(|| Error::new("unexpected EOF"))?;

        *self.data_offset = end;

        Ok(bytes)
    }

    pub fn u8_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let bytes = self.repeat_u8(N)?;

        Ok(bytes.try_into().expect("bytes has length `N`"))
    }

    pub fn u32(&mut self) -> Result<u32, Error> {
        let bytes = self.u8_array()?;

        Ok(u32::from_le_bytes(bytes))
    }

    pub fn bool32(&mut self) -> Result<bool, Error> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new("expected a 32-bit boolean")),
        }
    }

    pub fn string(&mut self) -> Result<&'a str, Error> {
        let len = self.u32()?;
        let bytes = self.repeat_u8(len as usize)?;

        str::from_utf8(bytes).map_err(Error::new)
    }

    pub fn list<T>(
        &mut self,
        mut read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Box<[T]>, Error> {
        let len = self.u32()?;

        iter::repeat_with(|| read_fn(self))
            .take(len as usize)
            .collect()
    }

    pub fn id(&mut self) -> Result<&'a str, Error> {
        match self.id_or_null()? {
            None => todo!(),
            Some(id) => Ok(id),
        }
    }

    pub fn id_or_null(&mut self) -> Result<Option<&'a str>, Error> {
        if !self.seen_id {
            if self.u32()? != 3 {
                todo!()
            }

            self.seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index & 0xc0000000 != 0x40000000 {
            todo!()
        }

        let index = index & 0x3fffffff;

        match index.checked_sub(1) {
            None => {
                let id = self.string()?;
                self.ids.push(id);

                Ok(Some(id))
            }
            Some(index) => todo!(),
        }
    }

    pub fn node_ref<T: 'static + ReadNodeRef>(&mut self) -> Result<&'a T, Error> {
        match self.node_ref_or_null()? {
            None => todo!(),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn node_ref_or_null<T: 'static + ReadNodeRef>(&mut self) -> Result<Option<&'a T>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or_else(|| Error::new("node reference index is zero"))?;

        let slot = self
            .node_refs_r
            .get(index as usize)
            .ok_or_else(|| Error::new("node reference index is out of bounds"))?;

        match slot.get() {
            None => {
                let class_id = self.u32()?;
                let node = T::read_from_body(
                    Arc::clone(&self.data),
                    Arc::clone(&self.node_refs),
                    self.data_offset,
                )?;
                slot.set(Box::new(node)).unwrap();

                Ok(Some(
                    slot.get().expect("init").downcast_ref().expect("downcast"),
                ))
            }
            Some(node_ref) => Ok(Some(node_ref.downcast_ref().unwrap())),
        }
    }
}

pub trait ReadNodeRef {
    fn read_from_body(
        body_data: Arc<[u8]>,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
        body_data_offset: &mut usize,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

pub struct BodyChunksReader<'a, 'b>(pub BodyReader<'a, 'b>);

impl<'a, 'b> BodyChunksReader<'a, 'b> {
    pub fn chunk<T>(
        &mut self,
        id: u32,
        mut read_fn: impl FnMut(&mut BodyReader<'a, 'b>) -> Result<T, Error>,
    ) -> Result<T, Error> {
        if self.0.u32()? != id {
            return Err(Error::new("chunk id mismatch"));
        }

        read_fn(&mut self.0)
    }

    pub fn skippable_chunk<T>(
        &mut self,
        id: u32,
        mut read_fn: impl FnMut(&mut BodyReader<'a, 'b>) -> Result<T, Error>,
    ) -> Result<T, Error> {
        if self.0.u32()? != id {
            return Err(Error::new("chunk id mismatch"));
        }

        if self.0.u32()? != 0x534b4950 {
            return Err(Error::new("expected a skippable chunk"));
        }

        let _size = self.0.u32()?;

        read_fn(&mut self.0)
    }

    pub fn end(&mut self) -> Result<(), Error> {
        if self.0.u32()? != 0xfacade01 {
            return Err(Error::new("expected end of node"));
        }

        Ok(())
    }
}
