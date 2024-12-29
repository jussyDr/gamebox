//! Low-level GameBox reader.

mod id;
mod node;

use bytemuck::{bytes_of_mut, cast_slice_mut, Pod};
pub use id::{IdState, IdStateMut};
pub use node::{NodeState, NodeStateMut};

use std::{
    cmp::min,
    io::{self, Read, Seek, SeekFrom},
    path::PathBuf,
};

use node::NullNodeState;

use crate::{
    Byte3, FileRef, Int2, Int3, Iso4, Nat3, PitchYawRoll, Quat, RgbFloat, RgbNat, Rgba, Vec2, Vec3,
    YawPitchRoll,
};

use super::{Error, ErrorKind};

/// Convert from little endian to native endian.
pub trait FromLe {
    /// Convert the `value` from little endian to native endian.
    fn from_le(value: Self) -> Self;
}

impl FromLe for u8 {
    fn from_le(value: Self) -> Self {
        value
    }
}

impl FromLe for u16 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for u32 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for u64 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for i16 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for i32 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for f32 {
    fn from_le(value: Self) -> Self {
        Self::from_bits(u32::from_le(value.to_bits()))
    }
}

impl<T: Copy + FromLe, const N: usize> FromLe for [T; N] {
    fn from_le(mut value: Self) -> Self {
        for value in &mut value {
            *value = T::from_le(*value);
        }

        value
    }
}

pub struct Take<R> {
    inner: R,
    limit: u64,
}

impl<R: Read> Read for Take<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.limit == 0 {
            return Ok(0);
        }

        let max = min(buf.len() as u64, self.limit) as usize;
        let n = self.inner.read(&mut buf[..max])?;

        assert!(n as u64 <= self.limit, "number of read bytes exceeds limit");

        self.limit -= n as u64;

        Ok(n)
    }
}

impl<R> Seek for Take<R> {
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
        unimplemented!()
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
    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        self.inner.read_to_end(buf).map_err(Error::io)
    }

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

    pub fn pod<T: Pod + FromLe>(&mut self) -> Result<T, Error> {
        let mut value = T::zeroed();

        self.inner
            .read_exact(bytes_of_mut(&mut value))
            .map_err(Error::io)?;

        Ok(T::from_le(value))
    }

    /// Read a signed 16-bit integer.
    pub fn i16(&mut self) -> Result<i16, Error> {
        self.pod()
    }

    /// Read a signed 32-bit integer.
    pub fn i32(&mut self) -> Result<i32, Error> {
        self.pod()
    }

    /// Read an unsigned 8-bit integer.
    pub fn u8(&mut self) -> Result<u8, Error> {
        self.pod()
    }

    /// Read an unsigned 16-bit integer.
    pub fn u16(&mut self) -> Result<u16, Error> {
        self.pod()
    }

    /// Read an unsigned 32-bit integer.
    pub fn u32(&mut self) -> Result<u32, Error> {
        self.pod()
    }

    /// Read an unsigned 64-bit integer.
    pub fn u64(&mut self) -> Result<u64, Error> {
        self.pod()
    }

    /// Read a 32-bit floating point number.
    pub fn f32(&mut self) -> Result<f32, Error> {
        self.pod()
    }

    /// Read a 32-bit boolean value.
    pub fn bool(&mut self) -> Result<bool, Error> {
        bool_from_u32(self.u32()?)
    }

    /// Read an 8-bit boolean value.
    pub fn bool8(&mut self) -> Result<bool, Error> {
        bool_from_u32(self.u8()? as u32)
    }

    pub fn enum_u32<T: TryFrom<u32>>(&mut self) -> Result<T, Error> {
        self.u32()?
            .try_into()
            .map_err(|_| Error::new(ErrorKind::Format("enum".into())))
    }

    pub fn enum_u8<T: TryFrom<u8>>(&mut self) -> Result<T, Error> {
        self.u8()?
            .try_into()
            .map_err(|_| Error::new(ErrorKind::Format("enum".into())))
    }

    pub fn u32_or_null(&mut self) -> Result<Option<u32>, Error> {
        let value = self.u32()?;

        if value == 0xffffffff {
            Ok(None)
        } else {
            Ok(Some(value))
        }
    }

    pub fn byte3(&mut self) -> Result<Byte3, Error> {
        self.pod()
    }

    pub fn nat3(&mut self) -> Result<Nat3, Error> {
        self.pod()
    }

    pub fn int2(&mut self) -> Result<Int2, Error> {
        self.pod()
    }

    pub fn int3(&mut self) -> Result<Int3, Error> {
        self.pod()
    }

    pub fn vec2(&mut self) -> Result<Vec2, Error> {
        self.pod()
    }

    pub fn vec3(&mut self) -> Result<Vec3, Error> {
        self.pod()
    }

    pub fn rgba(&mut self) -> Result<Rgba, Error> {
        self.pod()
    }

    pub fn rgb_nat(&mut self) -> Result<RgbNat, Error> {
        self.pod()
    }

    pub fn rgb_float(&mut self) -> Result<RgbFloat, Error> {
        self.pod()
    }

    pub fn yaw_pitch_roll(&mut self) -> Result<YawPitchRoll, Error> {
        self.pod()
    }

    pub fn pitch_yaw_roll(&mut self) -> Result<PitchYawRoll, Error> {
        self.pod()
    }

    /// Read a quaternion.
    pub fn quat(&mut self) -> Result<Quat, Error> {
        self.pod()
    }

    pub fn iso4(&mut self) -> Result<Iso4, Error> {
        self.vec3()?;
        self.vec3()?;
        self.vec3()?;
        self.vec3()?;

        Ok(Iso4::default())
    }

    pub fn box3d(&mut self) -> Result<u8, Error> {
        self.f32()?;
        self.f32()?;
        self.f32()?;
        self.f32()?;
        self.f32()?;
        self.f32()?;

        Ok(0)
    }

    pub fn string_of_len(&mut self, len: usize) -> Result<String, Error> {
        let bytes = self.bytes(len)?;

        String::from_utf8(bytes).map_err(|_| Error::new(ErrorKind::Format("not utf8".into())))
    }

    pub fn string(&mut self) -> Result<String, Error> {
        let len = self.u32()?;

        self.string_of_len(len as usize)
    }

    pub fn string_non_empty(&mut self) -> Result<Option<String>, Error> {
        let len = self.u32()?;

        if len == 0 {
            Ok(None)
        } else {
            let string = self.string_of_len(len as usize)?;

            Ok(Some(string))
        }
    }

    pub fn pack_desc_or_null(&mut self) -> Result<Option<FileRef>, Error> {
        let version = self.u8()?;

        if version != 3 {
            return Err(Error::version("pack desc", version as u32));
        }

        let checksum = self.byte_array::<32>()?;
        let path = self.string()?;
        let locator_url = self.string()?;

        if path.is_empty() {
            return Ok(None);
        }

        if locator_url.is_empty() {
            Ok(Some(FileRef::Internal {
                path: PathBuf::from(path),
            }))
        } else {
            Ok(Some(FileRef::External {
                path: PathBuf::from(path),
                locator_url,
                checksum,
            }))
        }
    }

    pub fn pack_desc(&mut self) -> Result<FileRef, Error> {
        match self.pack_desc_or_null()? {
            Some(pack_desc) => Ok(pack_desc),
            None => Err(Error::new(ErrorKind::Format("pack desc null".into()))),
        }
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let mut vec = Vec::with_capacity(n);

        for _ in 0..n {
            vec.push(read_fn(self)?);
        }

        Ok(vec)
    }

    pub fn repeat_pod<T: Pod + FromLe>(&mut self, len: usize) -> Result<Vec<T>, Error> {
        let mut vec = vec![T::zeroed(); len];

        self.inner
            .read_exact(cast_slice_mut(&mut vec))
            .map_err(Error::io)?;

        for value in &mut vec {
            *value = T::from_le(*value);
        }

        Ok(vec)
    }

    pub fn list<T>(
        &mut self,
        read_elem_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        self.repeat(len as usize, read_elem_fn)
    }

    pub fn list_pod<T: Pod + FromLe>(&mut self) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        self.repeat_pod(len as usize)
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

    pub fn encapsulation(
        &mut self,
        mut read_fn: impl FnMut(&mut Reader<Take<&mut R>, IdState, NullNodeState>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let size = self.u32()?;

        let mut reader = Reader::new(
            Take {
                inner: &mut self.inner,
                limit: size as u64,
            },
            IdState::new(),
            NullNodeState,
        );

        read_fn(&mut reader)?;

        reader.expect_eof()?;

        Ok(())
    }

    pub fn expect_eof(&mut self) -> Result<(), Error> {
        let mut buf = [0];

        let n = self.inner.read(&mut buf).map_err(Error::io)?;

        if n != 0 {
            return Err(Error::new(ErrorKind::Format("expected EOF".into())));
        }

        Ok(())
    }
}

fn bool_from_u32(value: u32) -> Result<bool, Error> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::new(ErrorKind::Format("expected a boolean".into()))),
    }
}

impl<R: Seek, I, N> Reader<R, I, N> {
    pub fn skip(&mut self, n: u64) -> Result<(), Error> {
        self.inner.seek_relative(n as i64).map_err(Error::io)?;

        Ok(())
    }

    pub fn seek_to_end(&mut self) -> Result<(), Error> {
        self.inner.seek(SeekFrom::End(0)).map_err(Error::io)?;

        Ok(())
    }
}

impl<R: Read + Seek, I, N> Reader<R, I, N> {
    pub fn peek_u32(&mut self) -> Result<u32, Error> {
        let value = self.u32()?;
        self.inner.seek_relative(-4).map_err(Error::io)?;

        Ok(value)
    }
}
