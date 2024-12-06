//! Low-level GameBox reader.

mod id;
mod node;

pub use id::{IdState, IdStateMut};
pub use node::{ExternalNodeRef, NodeRef, NodeState, NodeStateMut};

use std::{
    io::{Read, Seek},
    iter,
    path::PathBuf,
};

use node::NullNodeState;

use crate::{PackDesc, Quat, Vec2, Vec3};

use super::{Error, ErrorKind};

pub trait ReadNum: Sized {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error>;
}

impl ReadNum for u8 {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        r.u8()
    }
}

impl ReadNum for u32 {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        r.u32()
    }
}

impl ReadNum for i32 {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        r.i32()
    }
}

impl ReadNum for f32 {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        r.f32()
    }
}

/// Low-level GameBox reader.
pub struct Reader<R, I, N> {
    inner: R,
    id_state: I,
    node_state: N,
}

impl<R, I, N> Reader<R, I, N> {
    /// Create a new `Reader`.
    pub const fn new(inner: R, id_state: I, node_state: N) -> Self {
        Self {
            inner,
            id_state,
            node_state,
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }

    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }
}

impl<R: Read, I, N> Reader<R, I, N> {
    /// Read `n` bytes.
    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; n];

        self.inner.read_exact(&mut buf).map_err(Error::io)?;

        Ok(buf)
    }

    /// Read a byte array of length `S`.
    pub fn byte_array<const S: usize>(&mut self) -> Result<[u8; S], Error> {
        let mut buf = [0; S];

        self.inner.read_exact(&mut buf).map_err(Error::io)?;

        Ok(buf)
    }

    pub fn byte_buf(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.u32()?;

        self.bytes(size as usize)
    }

    /// Read a signed 16-bit integer.
    pub fn i16(&mut self) -> Result<i16, Error> {
        let bytes = self.byte_array()?;

        Ok(i16::from_le_bytes(bytes))
    }

    /// Read a signed 32-bit integer.
    pub fn i32(&mut self) -> Result<i32, Error> {
        let bytes = self.byte_array()?;

        Ok(i32::from_le_bytes(bytes))
    }

    /// Read an unsigned 8-bit integer.
    pub fn u8(&mut self) -> Result<u8, Error> {
        let [byte] = self.byte_array()?;

        Ok(byte)
    }

    /// Read an unsigned 16-bit integer.
    pub fn u16(&mut self) -> Result<u16, Error> {
        let bytes = self.byte_array()?;

        Ok(u16::from_le_bytes(bytes))
    }

    /// Read an unsigned 32-bit integer.
    pub fn u32(&mut self) -> Result<u32, Error> {
        let bytes = self.byte_array()?;

        Ok(u32::from_le_bytes(bytes))
    }

    /// Read an unsigned 64-bit integer.
    pub fn u64(&mut self) -> Result<u64, Error> {
        let bytes = self.byte_array()?;

        Ok(u64::from_le_bytes(bytes))
    }

    /// Read a 32-bit floating point number.
    pub fn f32(&mut self) -> Result<f32, Error> {
        let bytes = self.byte_array()?;

        Ok(f32::from_le_bytes(bytes))
    }

    pub fn bool(&mut self) -> Result<bool, Error> {
        bool_from_u32(self.u32()?)
    }

    pub fn bool8(&mut self) -> Result<bool, Error> {
        bool_from_u32(self.u8()? as u32)
    }

    pub fn enum_u32<T: TryFrom<u32>>(&mut self) -> Result<T, Error> {
        self.u32()?
            .try_into()
            .map_err(|_| Error::new(ErrorKind::Format("enum")))
    }

    pub fn enum_u8<T: TryFrom<u8>>(&mut self) -> Result<T, Error> {
        self.u8()?
            .try_into()
            .map_err(|_| Error::new(ErrorKind::Format("enum")))
    }

    /// Read a 2-dimensional vector of type `T`.
    pub fn vec2<T: ReadNum>(&mut self) -> Result<Vec2<T>, Error> {
        let x = T::read(self)?;
        let y = T::read(self)?;

        Ok(Vec2 { x, y })
    }

    /// Read a 3-dimensional vector of type `T`.
    pub fn vec3<T: ReadNum>(&mut self) -> Result<Vec3<T>, Error> {
        let x = T::read(self)?;
        let y = T::read(self)?;
        let z = T::read(self)?;

        Ok(Vec3 { x, y, z })
    }

    /// Read a quaternion.
    pub fn quat(&mut self) -> Result<Quat, Error> {
        let x = self.f32()?;
        let y = self.f32()?;
        let z = self.f32()?;
        let w = self.f32()?;

        Ok(Quat { x, y, z, w })
    }

    pub fn box3d(&mut self) -> Result<(), Error> {
        self.f32()?;
        self.f32()?;
        self.f32()?;
        self.f32()?;
        self.f32()?;
        self.f32()?;

        Ok(())
    }

    pub fn string(&mut self) -> Result<String, Error> {
        let len = self.u32()? as usize;

        self.string_of_len(len)
    }

    pub fn string_of_len(&mut self, len: usize) -> Result<String, Error> {
        let bytes = self.bytes(len)?;

        String::from_utf8(bytes).map_err(|_| Error::new(ErrorKind::Format("not utf8")))
    }

    pub fn encapsulation(
        &mut self,
        mut read_fn: impl FnMut(&mut Reader<&mut R, IdState, NullNodeState>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let size = self.u32()?;

        let mut reader = Reader::new(&mut self.inner, IdState::new(), NullNodeState);

        read_fn(&mut reader)?;

        // reader.expect_eof()?;

        Ok(())
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        iter::repeat_with(|| read_fn(self)).take(n).collect()
    }

    pub fn expect_eof(&mut self) -> Result<(), Error> {
        let mut buf = [0];

        let n = self.inner.read(&mut buf).map_err(Error::io)?;

        if n != 0 {
            return Err(Error::new(ErrorKind::Format("expected EOF")));
        }

        Ok(())
    }

    pub fn pack_desc(&mut self) -> Result<PackDesc, Error> {
        let version = self.u8()?;

        if version != 3 {
            return Err(Error::version("pack desc", version as u32));
        }

        let checksum = self.byte_array::<32>()?;
        let path = PathBuf::from(self.string()?);
        let locator_url = self.string()?;

        Ok(PackDesc::External {
            path,
            locator_url,
            checksum,
        })
    }

    pub fn list<T>(
        &mut self,
        mut read_elem_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        iter::repeat_with(|| read_elem_fn(self))
            .take(len as usize)
            .collect()
    }

    pub fn list_with_version<T>(
        &mut self,
        read_elem_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let version = self.u32()?;

        if version != 10 {
            return Err(Error::version("list", version));
        }

        self.list(read_elem_fn)
    }
}

fn bool_from_u32(value: u32) -> Result<bool, Error> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::new(ErrorKind::Format("expected a boolean"))),
    }
}

impl<R: Read + Seek, I, N> Reader<R, I, N> {
    pub fn peek_u32(&mut self) -> Result<u32, Error> {
        let value = self.u32()?;
        self.inner.seek_relative(-4).unwrap();

        Ok(value)
    }
}
