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

use crate::{F32Vec2, F32Vec3, U8Vec3, U32Box3, U32Vec3};

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

    if r.u32()? != T::CLASS_ID {
        return Err(Error::new("class id mismatch"));
    }

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
        let mut body_data = vec![];
        r.0.read_to_end(&mut body_data).map_err(Error::new)?;

        Arc::from(body_data)
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

pub trait Readable: ClassId {
    fn read_from_header_and_body(
        header_data: Box<[u8]>,
        body_data: Arc<[u8]>,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait ClassId {
    const CLASS_ID: u32;
}

pub struct BodyReader<'a, 'b> {
    pub data: &'a Arc<[u8]>,
    pub data_offset: &'b mut usize,
    pub node_refs: &'a Arc<[OnceCell<Box<dyn Any>>]>,
    seen_id: &'b mut bool,
    ids: &'b mut Vec<(usize, usize)>,
}

impl<'a, 'b> BodyReader<'a, 'b> {
    pub fn new(
        data: &'a Arc<[u8]>,
        data_offset: &'b mut usize,
        node_refs: &'a Arc<[OnceCell<Box<dyn Any>>]>,
        seen_id: &'b mut bool,
        ids: &'b mut Vec<(usize, usize)>,
    ) -> Self {
        Self {
            data,
            data_offset,
            node_refs,
            seen_id,
            ids,
        }
    }

    pub fn repeat_u8(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = *self.data_offset + n;

        let bytes = self
            .data
            .get(*self.data_offset..end)
            .ok_or_else(|| Error::new("unexpected EOF"))?;

        *self.data_offset = end;

        Ok(bytes)
    }

    pub fn list_u8(&mut self) -> Result<&'a [u8], Error> {
        let len = self.u32()?;

        self.repeat_u8(len as usize)
    }

    pub fn u8_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let bytes = self.repeat_u8(N)?;

        Ok(bytes.try_into().expect("bytes has length `N`"))
    }

    pub fn u8(&mut self) -> Result<u8, Error> {
        let bytes = self.u8_array()?;

        Ok(u8::from_le_bytes(bytes))
    }

    pub fn u16(&mut self) -> Result<u16, Error> {
        let bytes = self.u8_array()?;

        Ok(u16::from_le_bytes(bytes))
    }

    pub fn u32(&mut self) -> Result<u32, Error> {
        let bytes = self.u8_array()?;

        Ok(u32::from_le_bytes(bytes))
    }

    pub fn u128(&mut self) -> Result<u128, Error> {
        let bytes = self.u8_array()?;

        Ok(u128::from_le_bytes(bytes))
    }

    pub fn f32(&mut self) -> Result<f32, Error> {
        let bytes = self.u8_array()?;

        Ok(f32::from_le_bytes(bytes))
    }

    pub fn vec2_f32(&mut self) -> Result<F32Vec2, Error> {
        let x = self.f32()?;
        let y = self.f32()?;

        Ok(F32Vec2 { x, y })
    }

    pub fn vec3_u8(&mut self) -> Result<U8Vec3, Error> {
        let x = self.u8()?;
        let y = self.u8()?;
        let z = self.u8()?;

        Ok(U8Vec3 { x, y, z })
    }

    pub fn vec3_u32(&mut self) -> Result<U32Vec3, Error> {
        let x = self.u32()?;
        let y = self.u32()?;
        let z = self.u32()?;

        Ok(U32Vec3 { x, y, z })
    }

    pub fn vec3_f32(&mut self) -> Result<F32Vec3, Error> {
        let x = self.f32()?;
        let y = self.f32()?;
        let z = self.f32()?;

        Ok(F32Vec3 { x, y, z })
    }

    pub fn box3_u32(&mut self) -> Result<U32Box3, Error> {
        let a = self.vec3_u32()?;
        let b = self.vec3_u32()?;

        Ok(U32Box3 { a, b })
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

    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Box<[T]>, Error> {
        iter::repeat_with(|| read_fn(self)).take(n).collect()
    }

    pub fn list<T>(
        &mut self,
        read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Box<[T]>, Error> {
        let len = self.u32()?;

        self.repeat(len as usize, read_fn)
    }

    pub fn list_with_version<T>(
        &mut self,
        read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Box<[T]>, Error> {
        let version = self.u32()?;

        if version != 10 {
            return Err(Error::new(format!("unknown list version: {version}")));
        }

        self.list(read_fn)
    }

    pub fn id(&mut self) -> Result<&'a str, Error> {
        match self.id_or_null()? {
            None => Err(Error::new("expected a non-null identifier")),
            Some(id) => Ok(id),
        }
    }

    pub fn id_or_null(&mut self) -> Result<Option<&'a str>, Error> {
        if !*self.seen_id {
            let version = self.u32()?;

            if version != 3 {
                return Err(Error::new(format!("unknown identifier version: {version}")));
            }

            *self.seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index == 0x0000001a {
            return Ok(Some("")); // What is this?
        }

        if index == 0x00002713 {
            return Ok(Some("")); // What is this?
        }

        if index & 0xc0000000 != 0x40000000 {
            return Err(Error::new("expected an identifier"));
        }

        let index = index & 0x3fffffff;

        match index.checked_sub(1) {
            None => {
                let start_offset = *self.data_offset + size_of::<u32>();
                let id = self.string()?;
                let end_offset = *self.data_offset;

                self.ids.push((start_offset, end_offset));

                Ok(Some(id))
            }
            Some(index) => {
                let (start_offset, end_offset) = self
                    .ids
                    .get(index as usize)
                    .ok_or_else(|| Error::new("identifier index out of bounds"))?;

                let id = str::from_utf8(&self.data[*start_offset..*end_offset]).unwrap();

                Ok(Some(id))
            }
        }
    }

    pub fn node<T: ReadNode>(&mut self) -> Result<T, Error> {
        let class_id = self.u32()?;

        if class_id != T::CLASS_ID {
            return Err(Error::new("class id mismatch"));
        }

        T::read_from_body(
            Arc::clone(self.data),
            self.data_offset,
            Arc::clone(self.node_refs),
            self.seen_id,
            self.ids,
        )
    }

    pub fn node_ref<T: 'static + ReadNode>(&mut self) -> Result<&'a T, Error> {
        match self.node_ref_or_null()? {
            None => Err(Error::new("expected a non-null node reference")),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn node_ref_or_null<T: 'static + ReadNode>(&mut self) -> Result<Option<&'a T>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or_else(|| Error::new("node reference index is zero"))?;

        let slot = self
            .node_refs
            .get(index as usize)
            .ok_or_else(|| Error::new("node reference index is out of bounds"))?;

        match slot.get() {
            None => {
                let node = self.node::<T>()?;

                slot.set(Box::new(node)).unwrap();

                Ok(Some(
                    slot.get().expect("init").downcast_ref().expect("downcast"),
                ))
            }
            Some(node_ref) => Ok(Some(node_ref.downcast_ref().unwrap())),
        }
    }

    pub fn node_ref_generic<T: ReadNodeRef<'a>>(&mut self) -> Result<T, Error> {
        match self.node_ref_generic_or_null()? {
            None => todo!(),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn node_ref_generic_or_null<T: ReadNodeRef<'a>>(&mut self) -> Result<Option<T>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or_else(|| Error::new("node reference index is zero"))?;

        let slot = self
            .node_refs
            .get(index as usize)
            .ok_or_else(|| Error::new("node reference index is out of bounds"))?;

        let node_ref = match slot.get() {
            None => {
                let class_id = self.u32()?;

                let node = T::read(
                    Arc::clone(self.data),
                    self.data_offset,
                    Arc::clone(self.node_refs),
                    self.seen_id,
                    self.ids,
                    class_id,
                )?;

                slot.set(node)
                    .map_err(|_| Error::new("reentrant node reference init"))?;

                slot.get().expect("slot is initialized above")
            }
            Some(node_ref) => node_ref,
        };

        Ok(Some(T::upcast(node_ref.as_ref())?))
    }
}

pub trait ReadNode: ClassId {
    fn read_from_body(
        body_data: Arc<[u8]>,
        body_data_offset: &mut usize,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
        seen_id: &mut bool,
        ids: &mut Vec<(usize, usize)>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait ReadNodeRef<'a> {
    fn read(
        body_data: Arc<[u8]>,
        body_data_offset: &mut usize,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
        seen_id: &mut bool,
        ids: &mut Vec<(usize, usize)>,
        class_id: u32,
    ) -> Result<Box<dyn Any>, Error>;

    fn upcast(node_ref: &'a dyn Any) -> Result<Self, Error>
    where
        Self: Sized;
}

pub struct BodyChunksReader<'a, 'b, 'c>(pub &'c mut BodyReader<'a, 'b>);

impl<'a, 'b, 'c> BodyChunksReader<'a, 'b, 'c> {
    pub fn chunk<T>(
        &mut self,
        id: u32,
        mut read_fn: impl FnMut(&mut BodyReader<'a, 'b>) -> Result<T, Error>,
    ) -> Result<T, Error> {
        if self.0.u32()? != id {
            return Err(Error::new("chunk id mismatch"));
        }

        read_fn(self.0)
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

        read_fn(self.0)
    }

    pub fn end(&mut self) -> Result<(), Error> {
        if self.0.u32()? != 0xfacade01 {
            return Err(Error::new("expected end of node"));
        }

        Ok(())
    }
}
