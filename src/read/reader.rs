//! Reader

use zerocopy::{FromBytes, FromZeros, IntoBytes};

use std::{io::Read, iter};

use crate::{
    Box3D, Iso4, Quat, UVec3, Vec2, Vec3,
    read::{Error, byte_order::LeToNe, error_unknown_version, map_io_error},
};

/// Reader.
pub trait Reader: Read {
    /// Read `n` bytes.
    fn bytes(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; n];
        self.read_exact(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    /// Read `N` bytes into an array.
    fn byte_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    /// Read a byte buffer.
    fn byte_buf(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.u32()?;
        Reader::bytes(self, size as usize)
    }

    /// Read a value using zerocopy.
    fn zerocopy<T: FromBytes + LeToNe>(&mut self) -> Result<T, Error> {
        let mut value = T::read_from_io(self).map_err(map_io_error)?;

        // GameBox files are serialized as little endian,
        // so it is necessary to convert to the target's endianness.
        value.le_to_ne();

        Ok(value)
    }

    /// Read an unsigned 8-bit integer.
    fn u8(&mut self) -> Result<u8, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 16-bit integer.
    fn u16(&mut self) -> Result<u16, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 32-bit integer.
    fn u32(&mut self) -> Result<u32, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 64-bit integer.
    fn u64(&mut self) -> Result<u64, Error> {
        self.zerocopy()
    }

    /// Read an unsigned 128-bit integer.
    fn u128(&mut self) -> Result<u128, Error> {
        self.zerocopy()
    }

    /// Read a 32-bit floating point number.
    fn f32(&mut self) -> Result<f32, Error> {
        self.zerocopy()
    }

    /// Read an 8-bit boolean.
    fn bool8(&mut self) -> Result<bool, Error> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new("expected an 8-bit boolean")),
        }
    }

    /// Read a 32-bit boolean.
    fn bool32(&mut self) -> Result<bool, Error> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new("expected a 32-bit boolean")),
        }
    }

    /// Read a `Vec2`.
    fn vec2(&mut self) -> Result<Vec2, Error> {
        self.zerocopy()
    }

    /// Read a `Vec3`.
    fn vec3(&mut self) -> Result<Vec3, Error> {
        self.zerocopy()
    }

    /// Read a `UVec3`.
    fn uvec3(&mut self) -> Result<UVec3, Error> {
        self.zerocopy()
    }

    /// Read a `Quat`.
    fn quat(&mut self) -> Result<Quat, Error> {
        self.zerocopy()
    }

    /// Read a box.
    fn box3d(&mut self) -> Result<Box3D, Error> {
        self.zerocopy()
    }

    /// Read an `Iso4`.
    fn iso4(&mut self) -> Result<Iso4, Error> {
        self.zerocopy()
    }

    /// Read a string.
    fn string(&mut self) -> Result<String, Error> {
        let bytes = self.byte_buf()?;

        String::from_utf8(bytes).map_err(|_| Error::new("expected an UTF-8 encoded string"))
    }

    /// Read a list of elements.
    fn list<T>(
        &mut self,
        read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        self.repeat(len as usize, read_fn)
    }

    /// Read a list of elements using zerocopy.
    fn list_zerocopy<T: FromZeros + FromBytes + IntoBytes + LeToNe>(
        &mut self,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;
        self.repeat_zerocopy(len as usize)
    }

    /// Read a list of elements.
    fn list_with_version<T>(
        &mut self,
        read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let version = self.u32()?;

        if version != 10 {
            return Err(error_unknown_version("list", version));
        }

        self.list(read_fn)
    }

    /// Repeat the given `read_fn` a total of `n` times.
    fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        repeat_n_with(n, || read_fn(self))
    }

    /// Repeat the given `read_fn` a total of `n` times using zerocopy.
    fn repeat_zerocopy<T: FromZeros + FromBytes + IntoBytes + LeToNe>(
        &mut self,
        n: usize,
    ) -> Result<Vec<T>, Error> {
        let mut list = T::new_vec_zeroed(n).expect("memory allocation failed");
        let bytes = list.as_mut_slice().as_mut_bytes();
        self.read_exact(bytes).map_err(map_io_error)?;

        // GameBox files are serialized as little endian,
        // so it is necessary to convert to the target's endianness.
        list.le_to_ne();

        Ok(list)
    }

    /// Return an error if the reader is not at EOF.
    fn expect_eof(&mut self) -> Result<(), Error> {
        let mut buf = [0];
        let n = self.read(&mut buf).map_err(map_io_error)?;

        if n != 0 {
            return Err(Error::new("expected EOF"));
        }

        Ok(())
    }
}

impl<T: Read> Reader for T {}

fn repeat_n_with<T, U: FromIterator<T>>(n: usize, repeater: impl FnMut() -> T) -> U {
    iter::repeat_with(repeater).take(n).collect()
}

#[cfg(test)]
mod tests {
    use crate::{Vec3, read::reader::Reader};

    #[cfg(target_endian = "little")]
    #[test]
    fn read_vec3() {
        let mut r: &[u8] = &[0, 0, 0, 0, 0, 0, 0x80, 0x3f, 0, 0, 0x80, 0xbf];
        let vec = r.vec3().expect("failed to read Vec3");
        r.expect_eof().expect("expected EOF");
        assert_eq!(vec, Vec3::new(0.0, 1.0, -1.0))
    }
}
