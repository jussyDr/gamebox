//! Reader

mod id;
mod node;

pub use id::{IdTable, IdTableRef};
pub use node::{NodeTable, NodeTableRef};

use std::{
    io::{self, Read},
    iter,
};

use crate::{Iso4, Quat, Vec2, Vec3, read::Error};

/// Reader
pub struct Reader<R, I, N> {
    inner: R,
    id_table: I,
    node_state: N,
}

impl<R, I, N> Reader<R, I, N> {
    /// New
    pub const fn new(inner: R, id_table: I, node_state: N) -> Self {
        Self {
            inner,
            id_table,
            node_state,
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

fn map_io_error(_io_error: io::Error) -> Error {
    Error("IO error".into())
}

impl<R: Read, I, N> Reader<R, I, N> {
    /// Read `n` bytes.
    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; n];
        self.inner.read_exact(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    /// Read `L` bytes as an array.
    pub fn byte_array<const L: usize>(&mut self) -> Result<[u8; L], Error> {
        let mut buf = [0; L];
        self.inner.read_exact(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    /// Read an unsigned 8-bit integer.
    pub fn u8(&mut self) -> Result<u8, Error> {
        let bytes = self.byte_array()?;

        Ok(u8::from_le_bytes(bytes))
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

    /// Read a signed 16-bit integer.
    pub fn i16(&mut self) -> Result<i16, Error> {
        let bytes = self.byte_array()?;

        Ok(i16::from_le_bytes(bytes))
    }

    /// Read a 32-bit floating point number
    pub fn f32(&mut self) -> Result<f32, Error> {
        let bytes = self.byte_array()?;

        Ok(f32::from_le_bytes(bytes))
    }

    /// Read an 8-bit boolean value.
    pub fn bool8(&mut self) -> Result<bool, Error> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error("expected an 8-bit boolean".into())),
        }
    }

    /// Read a 32-bit boolean value.
    pub fn bool32(&mut self) -> Result<bool, Error> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error("expected a 32-bit boolean".into())),
        }
    }

    /// Read a 2-dimensional vector.
    pub fn vec2(&mut self) -> Result<Vec2, Error> {
        let x = self.f32()?;
        let y = self.f32()?;

        Ok(Vec2 { x, y })
    }

    /// Read a 3-dimensional vector.
    pub fn vec3(&mut self) -> Result<Vec3, Error> {
        let x = self.f32()?;
        let y = self.f32()?;
        let z = self.f32()?;

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

    pub fn iso4(&mut self) -> Result<Iso4, Error> {
        let elements = [
            self.f32()?,
            self.f32()?,
            self.f32()?,
            self.f32()?,
            self.f32()?,
            self.f32()?,
            self.f32()?,
            self.f32()?,
            self.f32()?,
            self.f32()?,
            self.f32()?,
            self.f32()?,
        ];

        Ok(Iso4(elements))
    }

    /// Read an UTF-8 encoded string.
    pub fn string(&mut self) -> Result<String, Error> {
        let len = self.u32()?;
        let bytes = self.bytes(len as usize)?;

        String::from_utf8(bytes).map_err(|_| Error("expected an UTF-8 encoded string".into()))
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_elem: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        iter::repeat_with(|| read_elem(self)).take(n).collect()
    }

    pub fn list<T>(
        &mut self,
        read_elem: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        self.repeat(len as usize, read_elem)
    }

    pub fn list_with_version<T>(
        &mut self,
        read_elem: impl Fn(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let version = self.u32()?;

        if version != 10 {
            return Err(Error("unknown list version".into()));
        }

        self.list(read_elem)
    }

    pub fn expect_eof(&mut self) -> Result<(), Error> {
        let mut buf = [0];
        let n = self.inner.read(&mut buf).map_err(map_io_error)?;

        if n != 0 {
            return Err(Error("expected EOF".into()));
        }

        Ok(())
    }

    /// Read all bytes until EOF.
    pub fn read_to_end(&mut self) -> Result<Vec<u8>, Error> {
        let mut buf = vec![];
        self.inner.read_to_end(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    pub fn node_or_null_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<Option<T>, Error> {
        let class_id = self.u32()?;

        if class_id == 0xffffffff {
            return Ok(None);
        }

        let node = read_fn(self, class_id)?;

        Ok(Some(node))
    }

    pub fn node_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let node = self.node_or_null_generic(read_fn)?;

        match node {
            None => Err(Error("node is null".into())),
            Some(node) => Ok(node),
        }
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
}
