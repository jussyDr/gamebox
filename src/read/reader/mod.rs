//! Low-level GameBox reader.

mod id;
mod node;

pub use id::{IdState, IdStateMut};
pub use node::{NodeState, NodeStateMut};

use std::{
    cmp::min,
    io::{self, Read, Seek, SeekFrom},
    mem::MaybeUninit,
    path::PathBuf,
    slice,
};

use node::NullNodeState;

use crate::{FileRef, Iso4, PitchYawRoll, Quat, Rgb, Vec2, Vec3};

use super::{Error, ErrorKind};

trait FromLe {
    fn from_le(value: Self) -> Self;
}

impl FromLe for u8 {
    fn from_le(value: Self) -> Self {
        value
    }
}

impl FromLe for u32 {
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

impl<T: FromLe> FromLe for Vec2<T> {
    fn from_le(mut value: Self) -> Self {
        value.x = T::from_le(value.x);
        value.y = T::from_le(value.y);

        value
    }
}

impl<T: FromLe> FromLe for Vec3<T> {
    fn from_le(mut value: Self) -> Self {
        value.x = T::from_le(value.x);
        value.y = T::from_le(value.y);
        value.z = T::from_le(value.z);

        value
    }
}

impl<T: FromLe> FromLe for Rgb<T> {
    fn from_le(mut value: Self) -> Self {
        value.r = T::from_le(value.r);
        value.g = T::from_le(value.g);
        value.b = T::from_le(value.b);

        value
    }
}

impl FromLe for PitchYawRoll {
    fn from_le(mut value: Self) -> Self {
        value.pitch = f32::from_le(value.pitch);
        value.yaw = f32::from_le(value.yaw);
        value.roll = f32::from_le(value.roll);

        value
    }
}

impl FromLe for Quat {
    fn from_le(mut value: Self) -> Self {
        value.x = f32::from_le(value.x);
        value.y = f32::from_le(value.y);
        value.z = f32::from_le(value.z);
        value.w = f32::from_le(value.w);

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
        todo!()
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

    pub fn vec2<T: FromLe>(&mut self) -> Result<Vec2<T>, Error> {
        let mut vec = MaybeUninit::<Vec2<T>>::uninit();

        let buf =
            unsafe { slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, size_of::<Vec2<T>>()) };

        self.inner.read_exact(buf).map_err(Error::io)?;

        let vec = unsafe { vec.assume_init() };

        Ok(Vec2::from_le(vec))
    }

    pub fn vec3<T: FromLe>(&mut self) -> Result<Vec3<T>, Error> {
        let mut vec = MaybeUninit::<Vec3<T>>::uninit();

        let buf =
            unsafe { slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, size_of::<Vec3<T>>()) };

        self.inner.read_exact(buf).map_err(Error::io)?;

        let vec = unsafe { vec.assume_init() };

        Ok(Vec3::from_le(vec))
    }

    pub fn rgb<T: FromLe>(&mut self) -> Result<Rgb<T>, Error> {
        let mut rgb = MaybeUninit::<Rgb<T>>::uninit();

        let buf =
            unsafe { slice::from_raw_parts_mut(rgb.as_mut_ptr() as *mut u8, size_of::<Rgb<T>>()) };

        self.inner.read_exact(buf).map_err(Error::io)?;

        let rgb = unsafe { rgb.assume_init() };

        Ok(Rgb::from_le(rgb))
    }

    pub fn pitch_yaw_roll(&mut self) -> Result<PitchYawRoll, Error> {
        let mut pitch_yaw_roll = MaybeUninit::<PitchYawRoll>::uninit();

        let buf = unsafe {
            slice::from_raw_parts_mut(
                pitch_yaw_roll.as_mut_ptr() as *mut u8,
                size_of::<PitchYawRoll>(),
            )
        };

        self.inner.read_exact(buf).map_err(Error::io)?;

        let pitch_yaw_roll = unsafe { pitch_yaw_roll.assume_init() };

        Ok(PitchYawRoll::from_le(pitch_yaw_roll))
    }

    /// Read a quaternion.
    pub fn quat(&mut self) -> Result<Quat, Error> {
        let mut quat = MaybeUninit::<Quat>::uninit();

        let buf =
            unsafe { slice::from_raw_parts_mut(quat.as_mut_ptr() as *mut u8, size_of::<Quat>()) };

        self.inner.read_exact(buf).map_err(Error::io)?;

        let quat = unsafe { quat.assume_init() };

        Ok(Quat::from_le(quat))
    }

    pub fn iso4(&mut self) -> Result<Iso4, Error> {
        self.vec3::<f32>()?;
        self.vec3::<f32>()?;
        self.vec3::<f32>()?;
        self.vec3::<f32>()?;

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

    pub fn string_or_empty(&mut self) -> Result<Option<String>, Error> {
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

    pub fn list<T>(
        &mut self,
        read_elem_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        self.repeat(len as usize, read_elem_fn)
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

    pub fn repeat_u8x4(&mut self, n: usize) -> Result<Vec<[u8; 4]>, Error> {
        let mut vec = Vec::with_capacity(n);

        let buf = unsafe {
            slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, n * size_of::<[u8; 4]>())
        };

        self.inner.read_exact(buf).map_err(Error::io)?;

        unsafe { vec.set_len(n) };

        #[cfg(target_endian = "big")]
        todo!();

        Ok(vec)
    }

    pub fn repeat_f32x2(&mut self, n: usize) -> Result<Vec<[f32; 2]>, Error> {
        let mut vec = Vec::with_capacity(n);

        let buf = unsafe {
            slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, n * size_of::<[f32; 2]>())
        };

        self.inner.read_exact(buf).map_err(Error::io)?;

        unsafe { vec.set_len(n) };

        #[cfg(target_endian = "big")]
        todo!();

        Ok(vec)
    }

    pub fn repeat_f32x3(&mut self, n: usize) -> Result<Vec<[f32; 3]>, Error> {
        let mut vec = Vec::with_capacity(n);

        let buf = unsafe {
            slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, n * size_of::<[f32; 3]>())
        };

        self.inner.read_exact(buf).map_err(Error::io)?;

        unsafe { vec.set_len(n) };

        #[cfg(target_endian = "big")]
        todo!();

        Ok(vec)
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
