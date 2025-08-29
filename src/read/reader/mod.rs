mod body;
pub use body::{BodyReader, BodyReaderImpl, ReadNode, ReadNodeRef};

mod body_chunks;
pub use body_chunks::read_body_chunks;

mod header;
pub use header::HeaderReader;

mod header_chunks;
pub use header_chunks::read_header_chunks;

use std::{io, iter};

use crate::{Rgb, Vec3, YawPitchRoll, read::Result};

use super::Error;

pub trait Reader: io::Read {
    fn array_u8<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(Error::io)?;

        Ok(buf)
    }

    fn repeat_u8(&mut self, n: usize) -> Result<Box<[u8]>> {
        let mut buf = vec![0; n].into_boxed_slice();
        self.read_exact(&mut buf).map_err(Error::io)?;

        Ok(buf)
    }

    fn list_u8(&mut self) -> Result<Box<[u8]>> {
        let len = self.u32()?;

        self.repeat_u8(len as usize)
    }

    fn u8(&mut self) -> Result<u8> {
        let bytes = self.array_u8()?;

        Ok(u8::from_le_bytes(bytes))
    }

    fn u16(&mut self) -> Result<u16> {
        let bytes = self.array_u8()?;

        Ok(u16::from_le_bytes(bytes))
    }

    fn u32(&mut self) -> Result<u32> {
        let bytes = self.array_u8()?;

        Ok(u32::from_le_bytes(bytes))
    }

    fn u64(&mut self) -> Result<u64> {
        let bytes = self.array_u8()?;

        Ok(u64::from_le_bytes(bytes))
    }

    fn u128(&mut self) -> Result<u128> {
        let bytes = self.array_u8()?;

        Ok(u128::from_le_bytes(bytes))
    }

    fn i32(&mut self) -> Result<i32> {
        let bytes = self.array_u8()?;

        Ok(i32::from_le_bytes(bytes))
    }

    fn f32(&mut self) -> Result<f32> {
        let bytes = self.array_u8()?;

        Ok(f32::from_le_bytes(bytes))
    }

    fn bool8(&mut self) -> Result<bool> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::Internal("expected a boolean".into())),
        }
    }

    fn bool32(&mut self) -> Result<bool> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::Internal("expected a boolean".into())),
        }
    }

    fn enum8<T: ReadEnum>(&mut self) -> Result<T> {
        let index = self.u8()?;

        T::from_u32(index as u32)
    }

    fn enum32<T: ReadEnum>(&mut self) -> Result<T> {
        let index = self.u32()?;

        T::from_u32(index)
    }

    fn vec2_f32(&mut self) -> Result<[f32; 2]> {
        let x = self.f32()?;
        let y = self.f32()?;

        Ok([x, y])
    }

    fn vec3_u8(&mut self) -> Result<Vec3<u8>> {
        let x = self.u8()?;
        let y = self.u8()?;
        let z = self.u8()?;

        Ok(Vec3 { x, y, z })
    }

    fn vec3_u32(&mut self) -> Result<Vec3<u32>> {
        let x = self.u32()?;
        let y = self.u32()?;
        let z = self.u32()?;

        Ok(Vec3 { x, y, z })
    }

    fn vec3_f32(&mut self) -> Result<Vec3<f32>> {
        let x = self.f32()?;
        let y = self.f32()?;
        let z = self.f32()?;

        Ok(Vec3 { x, y, z })
    }

    fn rgb(&mut self) -> Result<Rgb> {
        let red = self.f32()?;
        let green = self.f32()?;
        let blue = self.f32()?;

        Ok(Rgb { red, green, blue })
    }

    fn yaw_pitch_roll(&mut self) -> Result<YawPitchRoll> {
        let yaw = self.f32()?;
        let pitch = self.f32()?;
        let roll = self.f32()?;

        Ok(YawPitchRoll { yaw, pitch, roll })
    }

    fn box3_u32(&mut self) -> Result<[u32; 6]> {
        Ok([
            self.u32()?,
            self.u32()?,
            self.u32()?,
            self.u32()?,
            self.u32()?,
            self.u32()?,
        ])
    }

    fn iso4(&mut self) -> Result<[f32; 12]> {
        Ok([
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
        ])
    }

    fn string(&mut self) -> Result<String> {
        let len = self.u32()?;

        self.string_of_len(len as usize)
    }

    fn string_of_len(&mut self, len: usize) -> Result<String> {
        let bytes = self.repeat_u8(len)?;

        String::from_utf8(bytes.into_vec()).map_err(|err| Error::Internal(err.into()))
    }

    fn repeat<T>(
        &mut self,
        n: usize,
        read_fn: impl Fn(&mut Self) -> Result<T>,
    ) -> Result<Box<[T]>> {
        iter::repeat_with(|| read_fn(self)).take(n).collect()
    }

    fn list<T>(&mut self, read_fn: impl Fn(&mut Self) -> Result<T>) -> Result<Box<[T]>> {
        let len = self.u32()?;

        self.repeat(len as usize, read_fn)
    }

    fn list_versioned<T>(&mut self, read_fn: impl Fn(&mut Self) -> Result<T>) -> Result<Box<[T]>> {
        if self.u32()? != 10 {
            return Err(Error::Internal("unknown list version".into()));
        }

        self.list(read_fn)
    }

    fn skip(&mut self, n: usize) -> Result<()> {
        let mut buf = vec![0; n];
        self.read_exact(&mut buf).map_err(Error::io)?;

        Ok(())
    }
}

impl<T: io::Read> Reader for T {}

pub trait ReadEnum: Sized {
    fn from_u32(index: u32) -> Result<Self>;
}
