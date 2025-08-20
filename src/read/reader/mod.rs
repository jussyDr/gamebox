mod header;
pub use header::HeaderReader;

mod body;
pub use body::{BodyReader, BodyReaderImpl, ReadNode};

mod body_chunks;
pub use body_chunks::read_body_chunks;

use std::{io, iter};

use crate::read::Result;

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

    fn vec3_u8(&mut self) -> Result<[u8; 3]> {
        let x = self.u8()?;
        let y = self.u8()?;
        let z = self.u8()?;

        Ok([x, y, z])
    }

    fn vec3_u32(&mut self) -> Result<[u32; 3]> {
        let x = self.u32()?;
        let y = self.u32()?;
        let z = self.u32()?;

        Ok([x, y, z])
    }

    fn string(&mut self) -> Result<String> {
        let bytes = self.list_u8()?;

        String::from_utf8(bytes.into_vec()).map_err(|err| Error::Internal(err.into()))
    }

    fn list<T>(&mut self, read_fn: impl Fn(&mut Self) -> Result<T>) -> Result<Box<[T]>> {
        let len = self.u32()?;

        iter::repeat_with(|| read_fn(self))
            .take(len as usize)
            .collect()
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
