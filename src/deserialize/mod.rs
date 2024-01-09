//! Low-level GameBox deserialization.

mod id;
mod node;

pub use id::*;
pub use node::*;

use std::{
    cmp,
    io::{self, BufRead, Read, Seek, SeekFrom},
    iter,
    mem::size_of,
};

use crate::read::Result;

/// Adapter which limits the amount of bytes that can be read from a given reader.
pub struct Take<R> {
    reader: R,
    limit: u64,
}

impl<R: Read> Read for Take<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.limit == 0 {
            return Ok(0);
        }

        let max = cmp::min(buf.len() as u64, self.limit) as usize;
        let n = self.reader.read(&mut buf[..max])?;
        assert!(n as u64 <= self.limit, "number of read bytes exceeds limit");
        self.limit -= n as u64;

        Ok(n)
    }
}

impl<T: BufRead> BufRead for Take<T> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.limit == 0 {
            return Ok(&[]);
        }

        let buf = self.reader.fill_buf()?;
        let cap = cmp::min(buf.len() as u64, self.limit) as usize;
        Ok(&buf[..cap])
    }

    fn consume(&mut self, amt: usize) {
        let amt = cmp::min(amt as u64, self.limit) as usize;
        self.limit -= amt as u64;
        self.reader.consume(amt);
    }
}

impl<R: Seek> Seek for Take<R> {
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
        unimplemented!()
    }
}

/// Low-level GameBox deserializer.
pub struct Deserializer<R, I, N> {
    reader: R,
    id_state: I,
    node_state: N,
}

impl<R, I, N> Deserializer<R, I, N> {
    /// Create a new `Deserializer` with the given `id_state` and `node_state`.
    pub fn new(reader: R, id_state: I, node_state: N) -> Self {
        Self {
            reader,
            id_state,
            node_state,
        }
    }

    /// Unwraps this `Deserializer`, returning the underlying reader.
    pub fn into_reader(self) -> R {
        self.reader
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_reader_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Creates an adapter which will read at most `limit` bytes from it.
    pub fn take(&mut self, limit: u64) -> Deserializer<Take<&mut R>, &mut I, &mut N> {
        let inner = Take {
            reader: &mut self.reader,
            limit,
        };

        Deserializer::new(inner, &mut self.id_state, &mut self.node_state)
    }

    /// Creates an adapter with a new `id_state` and `node_state` which will read at most `limit` bytes from it.
    pub fn take_with<IS, NS>(
        &mut self,
        limit: u64,
        id_state: IS,
        node_state: NS,
    ) -> Deserializer<Take<&mut R>, IS, NS> {
        let inner = Take {
            reader: &mut self.reader,
            limit,
        };

        Deserializer::new(inner, id_state, node_state)
    }
}

impl<R: Read, I, N> Deserializer<R, I, N> {
    /// Read an unsigned 8-bit integer.
    #[inline]
    pub fn u8(&mut self) -> Result<u8> {
        let bytes = self.byte_array()?;

        Ok(u8::from_le_bytes(bytes))
    }

    /// Read an unsigned 16-bit integer.
    #[inline]
    pub fn u16(&mut self) -> Result<u16> {
        let bytes = self.byte_array()?;

        Ok(u16::from_le_bytes(bytes))
    }

    /// Read an unsigned 32-bit integer.
    #[inline]
    pub fn u32(&mut self) -> Result<u32> {
        let bytes = self.byte_array()?;

        Ok(u32::from_le_bytes(bytes))
    }

    /// Read an unsigned 64-bit integer.
    #[inline]
    pub fn u64(&mut self) -> Result<u64> {
        let bytes = self.byte_array()?;

        Ok(u64::from_le_bytes(bytes))
    }

    /// Read a signed 16-bit integer.
    #[inline]
    pub fn i16(&mut self) -> Result<i16> {
        let bytes = self.byte_array()?;

        Ok(i16::from_le_bytes(bytes))
    }

    /// Read a signed 32-bit integer.
    #[inline]
    pub fn i32(&mut self) -> Result<i32> {
        let bytes = self.byte_array()?;

        Ok(i32::from_le_bytes(bytes))
    }

    /// Read a 32-bit floating point number.
    #[inline]
    pub fn f32(&mut self) -> Result<f32> {
        let bytes = self.byte_array()?;

        Ok(f32::from_le_bytes(bytes))
    }

    /// Read a 8-bit boolean.
    pub fn bool8(&mut self) -> Result<bool> {
        let x = self.u8()?;
        bool_from_u8(x)
    }

    /// Read a 32-bit boolean.
    pub fn bool32(&mut self) -> Result<bool> {
        let x = self.u32()?;
        bool_from_u8(x as u8)
    }

    /// Read `n` bytes.
    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; n];
        self.reader.read_exact(&mut buf)?;

        Ok(buf)
    }

    /// Read `L` bytes into an array.
    #[inline]
    pub fn byte_array<const L: usize>(&mut self) -> Result<[u8; L]> {
        let mut array = [0; L];
        self.reader.read_exact(&mut array)?;

        Ok(array)
    }

    /// Read a string.
    pub fn string(&mut self) -> Result<String> {
        let len = self.u32()?;
        let bytes = self.bytes(len as usize)?;
        let string = String::from_utf8(bytes)?;

        Ok(string)
    }

    /// Repeat the given `read_fn` a total of `n` times.
    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<T>> {
        repeat_n_with(n, || read_fn(self))
    }

    /// Read a list where the given `read_fn` is used to read each element.
    pub fn list<T>(&mut self, read_fn: impl FnMut(&mut Self) -> Result<T>) -> Result<Vec<T>> {
        let len = self.u32()?;
        self.repeat(len as usize, read_fn)
    }

    /// Read a list zipped with the given `vec` where the given `read_fn` is used to read each element.
    pub fn list_zipped_with<T, U>(
        &mut self,
        vec: Vec<U>,
        mut read_fn: impl FnMut(&mut Self, U) -> Result<T>,
    ) -> Result<Vec<T>> {
        let len = self.u32()?;

        if len as usize != vec.len() {
            return Err("list sizes do not match".into());
        }

        vec.into_iter().map(|x| read_fn(self, x)).collect()
    }

    /// Returns `Ok(())` if the underlying reader has no data left.
    pub fn eof(&mut self) -> Result<()> {
        let mut buf = [0];

        match self.reader.read(&mut buf) {
            Ok(0) => Ok(()),
            _ => Err("expected end of reader".into()),
        }
    }

    /// Read all bytes until EOF in this source.
    pub fn read_to_end(&mut self) -> Result<Vec<u8>> {
        let mut buf = vec![];
        self.reader.read_to_end(&mut buf)?;

        Ok(buf)
    }
}

impl<R: Seek, I, N> Deserializer<R, I, N> {
    /// Skip `n` bytes.
    pub fn skip(&mut self, n: u32) -> Result<()> {
        self.reader.seek(io::SeekFrom::Current(n as i64))?;

        Ok(())
    }
}

impl<R: Read + Seek, I, N> Deserializer<R, I, N> {
    /// Peek an unsigned 32-bit integer without advancing the underlying reader.
    pub fn peek_u32(&mut self) -> Result<u32> {
        let value = self.u32()?;
        self.reader
            .seek(io::SeekFrom::Current(-(size_of::<u32>() as i64)))?;

        Ok(value)
    }
}

fn bool_from_u8(x: u8) -> Result<bool> {
    match x {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err("expected a boolean".into()),
    }
}

fn repeat_n_with<T, V: FromIterator<T>>(n: usize, repeater: impl FnMut() -> T) -> V {
    iter::repeat_with(repeater).take(n).collect()
}
