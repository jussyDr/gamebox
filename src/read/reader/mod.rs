//! Low-level GameBox reader.

mod id;
mod node;
mod take;

pub use id::*;
pub use node::*;
pub use take::Take;

use std::{
    io::{self, Read, Seek},
    iter,
    mem::size_of,
};

use crate::{
    common::{Class, ClassId},
    read::{readable::ReadBody, Result},
};

use self::take::take;

/// Low-level GameBox reader.
pub struct Reader<R, I, N> {
    inner: R,
    id_state: I,
    node_state: N,
}

impl<R, I, N> Reader<R, I, N> {
    /// Create a new `Reader` with the given `id_state` and `node_state`.
    pub fn new(inner: R, id_state: I, node_state: N) -> Self {
        Self {
            inner,
            id_state,
            node_state,
        }
    }

    /// Unwraps this `Reader`, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_inner_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Creates an adapter which will read at most `limit` bytes from it.
    pub fn take(&mut self, limit: u64) -> Reader<Take<&mut R>, &mut I, &mut N> {
        let take = take(&mut self.inner, limit);

        Reader::new(take, &mut self.id_state, &mut self.node_state)
    }

    /// Creates an adapter with a new `id_state` and `node_state` which will read at most `limit` bytes from it.
    pub fn take_with<IS, NS>(
        &mut self,
        limit: u64,
        id_state: IS,
        node_state: NS,
    ) -> Reader<Take<&mut R>, IS, NS> {
        let take = take(&mut self.inner, limit);

        Reader::new(take, id_state, node_state)
    }
}

impl<R: Read, I, N> Reader<R, I, N> {
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
    #[inline(always)]
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
        self.inner.read_exact(&mut buf)?;

        Ok(buf)
    }

    /// Read `L` bytes into an array.
    #[inline]
    pub fn byte_array<const L: usize>(&mut self) -> Result<[u8; L]> {
        let mut array = [0; L];
        self.inner.read_exact(&mut array)?;

        Ok(array)
    }

    /// Read a string.
    pub fn string(&mut self) -> Result<String> {
        let len = self.u32()?;
        self.string_of_len(len as usize)
    }

    /// Read a string of the given `len`.
    pub fn string_of_len(&mut self, len: usize) -> Result<String> {
        let bytes = self.bytes(len)?;
        let string = String::from_utf8(bytes)?;

        Ok(string)
    }

    /// Read an unsigned 32-bit integer and return an error if it does not match the given `value`.
    pub fn expect_u32(&mut self, value: u32) -> Result<()> {
        let actual_value = self.u32()?;

        if actual_value != value {
            return Err(format!("expected {value}, got {actual_value}").into());
        }

        Ok(())
    }

    /// Read bytes and return an error if it does not match the given `bytes`.
    pub fn expect_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        if self.bytes(bytes.len())? != bytes {
            return Err("".into());
        }

        Ok(())
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

    /// Read from a byte buffer using the given `read_fn` with a new node reference state and id state.
    pub fn scoped_buffer(
        &mut self,
        read_fn: impl FnOnce(&mut Reader<Take<&mut R>, IdState, NodeState>) -> Result<()>,
    ) -> Result<()> {
        let len = self.u32()?;

        let mut r = self.take_with(len as u64, IdState::new(), NodeState::new(0));

        read_fn(&mut r)?;

        r.eof()?;

        Ok(())
    }

    /// Read a node that is not null.
    pub fn node<T: Default + Class + ReadBody<R, I, N>>(&mut self) -> Result<T> {
        match self.node_or_null()? {
            None => Err("node is null".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read a node that may be null.
    pub fn node_or_null<T: Default + Class + ReadBody<R, I, N>>(&mut self) -> Result<Option<T>> {
        let class_id = ClassId::read_or_null(self)?;

        match class_id {
            None => Ok(None),
            Some(class_id) => {
                if class_id != T::CLASS_ID {
                    return Err("class id does not match".into());
                }

                let mut node = T::default();

                T::read_body(&mut node, self)?;

                Ok(Some(node))
            }
        }
    }

    /// Returns `Ok(())` if the underlying reader has no data left.
    pub fn eof(&mut self) -> Result<()> {
        let mut buf = [0];

        match self.inner.read(&mut buf) {
            Ok(0) => Ok(()),
            _ => Err("expected end of reader".into()),
        }
    }

    /// Read all bytes until EOF in this source.
    pub fn read_to_end(&mut self) -> Result<Vec<u8>> {
        let mut buf = vec![];
        self.inner.read_to_end(&mut buf)?;

        Ok(buf)
    }
}

impl<R: Seek, I, N> Reader<R, I, N> {
    /// Skip `n` bytes.
    pub fn skip(&mut self, n: u32) -> Result<()> {
        self.inner.seek(io::SeekFrom::Current(n as i64))?;

        Ok(())
    }
}

impl<R: Read + Seek, I, N> Reader<R, I, N> {
    /// Peek an unsigned 32-bit integer without advancing the underlying reader.
    pub fn peek_u32(&mut self) -> Result<u32> {
        let value = self.u32()?;
        self.inner
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
