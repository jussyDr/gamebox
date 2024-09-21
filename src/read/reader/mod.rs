mod id;
mod node;

pub use id::{IdState, IdStateMut, IdStateRef};
pub use node::{NodeState, NodeStateMut, NodeStateRef};

use std::{
    cmp,
    io::{self, Read, Seek, SeekFrom},
    iter,
};

use crate::{Box3, PackDesc, Vec2, Vec3, Vec4};

use super::Error;

pub trait Num {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error>
    where
        Self: Sized;
}

impl Num for f32 {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let bytes = r.byte_array()?;

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Num for i32 {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let bytes = r.byte_array()?;

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Num for u8 {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let bytes = r.byte_array()?;

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Num for u32 {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let bytes = r.byte_array()?;

        Ok(Self::from_le_bytes(bytes))
    }
}

/// Reader adapter which limits the bytes read from an underlying reader.
pub struct Take<R> {
    inner: R,
    limit: u64,
}

impl<R: Read> Read for Take<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.limit == 0 {
            return Ok(0);
        }

        let max = cmp::min(buf.len() as u64, self.limit) as usize;
        let n = self.inner.read(&mut buf[..max])?;

        assert!(n as u64 <= self.limit, "number of read bytes exceeds limit");

        self.limit -= n as u64;

        Ok(n)
    }
}

/// Low-level GameBox reader.
pub struct Reader<R, I, N> {
    inner: R,
    id_state: I,
    node_state: N,
}

impl<R, I, N> Reader<R, I, N> {
    /// Create a new reader.
    pub const fn new(inner: R, id_state: I, node_state: N) -> Self {
        Self {
            inner,
            id_state,
            node_state,
        }
    }

    /// Return the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Creates an adapter which will read at most `limit` bytes from it.
    pub fn take(&mut self, limit: u64) -> Reader<Take<&mut R>, &mut I, &mut N> {
        Reader {
            inner: Take {
                inner: &mut self.inner,
                limit,
            },
            id_state: &mut self.id_state,
            node_state: &mut self.node_state,
        }
    }

    /// Creates an adapter which will read at most `limit` bytes from it with the given `id_state` and `node_state`.
    pub fn take_with<IS, NS>(
        &mut self,
        limit: u64,
        id_state: IS,
        node_state: NS,
    ) -> Reader<Take<&mut R>, IS, NS> {
        Reader {
            inner: Take {
                inner: &mut self.inner,
                limit,
            },
            id_state,
            node_state,
        }
    }
}

impl<R: Read, I, N> Reader<R, I, N> {
    /// Read `n` bytes.
    pub fn bytes(&mut self, n: usize) -> Result<Box<[u8]>, Error> {
        let mut bytes = vec![0; n];
        self.inner.read_exact(&mut bytes).map_err(|_| Error)?;

        Ok(bytes.into_boxed_slice())
    }

    /// Read an array of `L` bytes.
    pub fn byte_array<const L: usize>(&mut self) -> Result<[u8; L], Error> {
        let mut byte_array = [0; L];
        self.inner.read_exact(&mut byte_array).map_err(|_| Error)?;

        Ok(byte_array)
    }

    /// Read a 32-bit floating point number.
    pub fn f32(&mut self) -> Result<f32, Error> {
        f32::read(self)
    }

    /// Read a signed 32-bit integer.
    pub fn i32(&mut self) -> Result<i32, Error> {
        let bytes = self.byte_array()?;

        Ok(i32::from_le_bytes(bytes))
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

    /// Read an unsigned 128-bit integer.
    pub fn u128(&mut self) -> Result<u128, Error> {
        let bytes = self.byte_array()?;

        Ok(u128::from_le_bytes(bytes))
    }

    /// Read an 8-bit boolean.
    pub fn bool8(&mut self) -> Result<bool, Error> {
        let b = match self.u8()? {
            0 => false,
            1 => true,
            _ => return Err(Error),
        };

        Ok(b)
    }

    /// Read a 32-bit boolean.
    pub fn bool(&mut self) -> Result<bool, Error> {
        let b = match self.u32()? {
            0 => false,
            1 => true,
            _ => return Err(Error),
        };

        Ok(b)
    }

    /// Read a 2-dimensional vector.
    pub fn vec2<T: Num>(&mut self) -> Result<Vec2<T>, Error> {
        let x = T::read(self)?;
        let y = T::read(self)?;

        Ok(Vec2 { x, y })
    }

    /// Read a 3-dimensional vector.
    pub fn vec3<T: Num>(&mut self) -> Result<Vec3<T>, Error> {
        let x = T::read(self)?;
        let y = T::read(self)?;
        let z = T::read(self)?;

        Ok(Vec3 { x, y, z })
    }

    /// Read a 4-dimensional vector.
    pub fn vec4<T: Num>(&mut self) -> Result<Vec4<T>, Error> {
        let x = T::read(self)?;
        let y = T::read(self)?;
        let z = T::read(self)?;
        let w = T::read(self)?;

        Ok(Vec4 { x, y, z, w })
    }

    /// Read a 3-dimensional box.
    pub fn box3<T: Num>(&mut self) -> Result<Box3<T>, Error> {
        Ok(Box3(self.vec3()?, self.vec3()?))
    }

    /// Read a pack descriptor.
    pub fn pack_desc(&mut self) -> Result<Option<PackDesc>, Error> {
        let version = self.u8()?;

        if version != 3 {
            return Err(Error);
        }

        let _checksum = self.byte_array::<32>()?;
        let _path = self.string()?;
        let _locator_url = self.string()?;

        Ok(Some(PackDesc))
    }

    /// Read buffer of bytes.
    pub fn byte_buf(&mut self) -> Result<Box<[u8]>, Error> {
        let len = self.u32()?;

        self.bytes(len as usize)
    }

    /// Read a UTF-8 string with the given `len`.
    pub fn string_of_len(&mut self, len: usize) -> Result<String, Error> {
        let bytes = self.bytes(len)?;

        String::from_utf8(bytes.into()).map_err(|_| Error)
    }

    /// Read a UTF-8 string.
    pub fn string(&mut self) -> Result<String, Error> {
        let len = self.u32()?;

        self.string_of_len(len as usize)
    }

    /// Repeat the given `read_fn` a total of `n` times.
    pub fn repeat<T>(
        &mut self,
        n: usize,
        read_fn: impl Fn(&mut Self) -> Result<T, Error>,
    ) -> Result<Box<[T]>, Error> {
        iter::repeat_with(|| read_fn(self)).take(n).collect()
    }

    /// Read a list.
    pub fn list<T>(
        &mut self,
        read_elem: impl Fn(&mut Self) -> Result<T, Error>,
    ) -> Result<Box<[T]>, Error> {
        let len = self.u32()?;

        self.repeat(len as usize, read_elem)
    }

    /// Read a list.
    pub fn versioned_list<T>(
        &mut self,
        read_elem: impl Fn(&mut Self) -> Result<T, Error>,
    ) -> Result<Box<[T]>, Error> {
        let version = self.u32()?;

        if version != 10 {
            return Err(Error);
        }

        self.list(read_elem)
    }

    /// Check if this reader is at EOF else return an error.
    pub fn expect_eof(&mut self) -> Result<(), Error> {
        let mut buf = [0];

        if self.inner.read(&mut buf).map_err(|_| Error)? != 0 {
            return Err(Error);
        }

        Ok(())
    }

    /// Read all bytes until EOF.
    pub fn read_to_end(&mut self) -> Result<Box<[u8]>, Error> {
        let mut bytes = vec![];
        self.inner.read_to_end(&mut bytes).map_err(|_| Error)?;

        Ok(bytes.into_boxed_slice())
    }
}

impl<R: Seek, I, N> Reader<R, I, N> {
    /// Return the current position.
    pub fn pos(&mut self) -> Result<u64, Error> {
        self.inner.stream_position().map_err(|_| Error)
    }

    /// Skip `n` bytes and return the new position.
    pub fn skip(&mut self, n: u64) -> Result<u64, Error> {
        self.inner
            .seek(SeekFrom::Current(n as i64))
            .map_err(|_| Error)
    }
}
