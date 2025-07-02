//! Reader

mod id;
mod node;

pub use id::{IdTable, IdTableRef};
pub use node::{NodeTable, NodeTableRef};
use zerocopy::{FromBytes, FromZeros, IntoBytes};

use std::{io::Read, iter};

use crate::{
    Iso4, Quat, Vec2, Vec3,
    read::{Error, byte_order::LeToNe, map_io_error},
};

fn repeat_n_with<T, U: FromIterator<T>>(n: usize, repeater: impl FnMut() -> T) -> U {
    iter::repeat_with(repeater).take(n).collect()
}

/// Reader
pub struct Reader<R, I, N> {
    inner: R,
    id_table: I,
    node_state: N,
}

impl<R, I, N> Reader<R, I, N> {
    /// Create a new reader.
    pub fn new(inner: R, id_table: I, node_state: N) -> Self {
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

    pub fn byte_buf(&mut self) -> Result<Vec<u8>, Error> {
        let len = self.u32()?;
        self.bytes(len as usize)
    }

    fn zerocopy<T: FromBytes + LeToNe>(&mut self) -> Result<T, Error> {
        let mut value = T::read_from_io(&mut self.inner).map_err(map_io_error)?;

        // GameBox files are serialized as little endian.
        // Here we convert to the target's endianness.
        value.le_to_ne();

        Ok(value)
    }

    /// Read an unsigned 8-bit integer.
    pub fn u8(&mut self) -> Result<u8, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 16-bit integer.
    pub fn u16(&mut self) -> Result<u16, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 32-bit integer.
    pub fn u32(&mut self) -> Result<u32, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 64-bit integer.
    pub fn u64(&mut self) -> Result<u64, Error> {
        self.zerocopy()
    }

    /// Read a signed 16-bit integer.
    pub fn i16(&mut self) -> Result<i16, Error> {
        self.zerocopy()
    }

    /// Read a 32-bit floating point number
    pub fn f32(&mut self) -> Result<f32, Error> {
        self.zerocopy()
    }

    /// Read an 8-bit boolean value.
    pub fn bool8(&mut self) -> Result<bool, Error> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new("expected an 8-bit boolean")),
        }
    }

    /// Read a 32-bit boolean value.
    pub fn bool32(&mut self) -> Result<bool, Error> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new("expected a 32-bit boolean")),
        }
    }

    /// Read a 2-dimensional vector.
    pub fn vec2(&mut self) -> Result<Vec2, Error> {
        self.zerocopy()
    }

    /// Read a 3-dimensional vector.
    pub fn vec3(&mut self) -> Result<Vec3, Error> {
        self.zerocopy()
    }

    /// Read a quaternion.
    pub fn quat(&mut self) -> Result<Quat, Error> {
        self.zerocopy()
    }

    pub fn box3d(&mut self) -> Result<(), Error> {
        self.u32()?;
        self.u32()?;
        self.u32()?;
        self.u32()?;
        self.u32()?;
        self.u32()?;

        Ok(())
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

    pub fn string(&mut self) -> Result<String, Error> {
        let bytes = self.byte_buf()?;

        String::from_utf8(bytes).map_err(|_| Error::new("expected an UTF-8 encoded string"))
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_elem: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        repeat_n_with(n, || read_elem(self))
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
            return Err(Error::new("unknown list version"));
        }

        self.list(read_elem)
    }

    pub fn repeat_zerocopy<T: FromZeros + FromBytes + IntoBytes + LeToNe>(
        &mut self,
        n: usize,
    ) -> Result<Vec<T>, Error> {
        let mut list = T::new_vec_zeroed(n).unwrap();
        let bytes = list.as_mut_slice().as_mut_bytes();
        self.inner.read_exact(bytes).map_err(map_io_error)?;

        // GameBox files are serialized as little endian.
        // Here we convert to the target's endianness.
        list.le_to_ne();

        Ok(list)
    }

    pub fn list_zerocopy<T: FromZeros + FromBytes + IntoBytes + LeToNe>(
        &mut self,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;
        self.repeat_zerocopy(len as usize)
    }

    /// Returns an error if the reader is not at EOF.
    pub fn expect_eof(&mut self) -> Result<(), Error> {
        let mut buf = [0];
        let n = self.inner.read(&mut buf).map_err(map_io_error)?;

        if n != 0 {
            return Err(Error::new("expected EOF"));
        }

        Ok(())
    }

    /// Read all bytes until EOF.
    pub fn read_to_end(&mut self) -> Result<Vec<u8>, Error> {
        let mut buf = vec![];
        self.inner.read_to_end(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }
}
