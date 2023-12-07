use std::{
    borrow::BorrowMut,
    cmp,
    io::{self, Read, Seek, SeekFrom},
    iter,
};

use crate::read::Result;

/// State of identifiers read in the past.
#[derive(Default)]
pub struct IdState {
    seen_id: bool,
    ids: Vec<String>,
}

/// Trait which should be used as a generic trait bound in
/// functions that need to access the `IdState`.
pub trait IdStateMut: BorrowMut<IdState> {}

impl<T: BorrowMut<IdState>> IdStateMut for T {}

/// State of nodes read in the past.
pub struct NodeState {
    num_nodes: u32,
}

impl NodeState {
    /// Create a new `NodeState` with a node limit of `num_nodes`.
    pub fn new(num_nodes: u32) -> Self {
        Self { num_nodes }
    }
}

/// Trait which should be used as a generic trait bound in
/// functions that need to access the `NodeState`.
pub trait NodeStateMut: BorrowMut<NodeState> {}

impl<T: BorrowMut<NodeState>> NodeStateMut for T {}

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

impl<R: Seek> Seek for Take<R> {
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
        todo!();
    }
}

/// Used for reading binary data in GameBox files.
pub struct Deserializer<R, I, N> {
    reader: R,
    id_state: I,
    node_state: N,
}

impl<R, I, N> Deserializer<R, I, N> {
    /// Create a new `Deserializer`.
    pub fn new(reader: R, id_state: I, node_state: N) -> Self {
        Self {
            reader,
            id_state,
            node_state,
        }
    }
}

impl<R: Read, I, N> Deserializer<R, I, N> {
    /// Deserialize an 8-bit unsigned integer.
    pub fn u8(&mut self) -> Result<u8> {
        let bytes = self.byte_array()?;
        Ok(u8::from_le_bytes(bytes))
    }

    /// Deserialize a 16-bit unsigned integer.
    pub fn u16(&mut self) -> Result<u16> {
        let bytes = self.byte_array()?;
        Ok(u16::from_le_bytes(bytes))
    }

    /// Deserialize a 32-bit unsigned integer.
    pub fn u32(&mut self) -> Result<u32> {
        let bytes = self.byte_array()?;
        Ok(u32::from_le_bytes(bytes))
    }

    /// Deserialize a 16-bit signed integer.
    pub fn i16(&mut self) -> Result<i16> {
        let bytes = self.byte_array()?;
        Ok(i16::from_le_bytes(bytes))
    }

    /// Deserialize a 32-bit floating point number.
    pub fn f32(&mut self) -> Result<f32> {
        let bytes = self.byte_array()?;
        Ok(f32::from_le_bytes(bytes))
    }

    pub fn bool8(&mut self) -> Result<bool> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => todo!(),
        }
    }

    /// Deserialize `n` bytes.
    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; n];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Deserialize an array of `L` bytes.
    pub fn byte_array<const L: usize>(&mut self) -> Result<[u8; L]> {
        let mut array = [0; L];
        self.reader.read_exact(&mut array)?;
        Ok(array)
    }

    /// Deserialize a string.
    pub fn string(&mut self) -> Result<String> {
        let len = self.u32()?;
        let bytes = self.bytes(len as usize)?;
        let string = String::from_utf8(bytes).unwrap();
        Ok(string)
    }

    /// Deserialize a list.
    pub fn list<T>(&mut self, read_fn: impl Fn(&mut Self) -> Result<T>) -> Result<Vec<T>> {
        let len = self.u32()?;
        self.repeat(len as usize, read_fn)
    }

    /// Repeat the `read_fn` a total of `n` times.
    pub fn repeat<T>(
        &mut self,
        n: usize,
        read_fn: impl Fn(&mut Self) -> Result<T>,
    ) -> Result<Vec<T>> {
        iter::repeat_with(|| read_fn(self)).take(n).collect()
    }

    /// Create an adapter which will read at most `limit` bytes from this deserializer.
    pub fn take<IS, NS>(
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

    /// Check if we are at the end of the reader.
    pub fn end(&mut self) -> Result<()> {
        let mut buf = [0];

        match self.reader.read(&mut buf) {
            Ok(0) => Ok(()),
            _ => todo!(),
        }
    }
}

impl<R: Seek, I, N> Deserializer<R, I, N> {
    pub fn skip(&mut self, n: u32) -> Result<()> {
        self.reader.seek(io::SeekFrom::Current(n as i64))?;
        Ok(())
    }
}

impl<R: Read + Seek, I, N> Deserializer<R, I, N> {
    /// Peek `n` bytes.
    pub fn peek_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let bytes = self.bytes(n)?;
        self.reader.seek(io::SeekFrom::Current(-(n as i64)))?;
        Ok(bytes)
    }
}

impl<R: Read, I: IdStateMut, N> Deserializer<R, I, N> {
    /// Deserialize an identifier.
    pub fn id(&mut self) -> Result<String> {
        match self.id_or_null()? {
            None => todo!(),
            Some(id) => Ok(id),
        }
    }

    /// Deserialize an identifier which could possibly be `null`.
    pub fn id_or_null(&mut self) -> Result<Option<String>> {
        if !self.id_state.borrow().seen_id {
            if self.u32()? != 3 {
                todo!()
            }

            self.id_state.borrow_mut().seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index == 0x40000000 {
            let id = self.string()?;
            self.id_state.borrow_mut().ids.push(id.clone());
            return Ok(Some(id));
        }

        todo!()
    }
}

impl<R: Read, I, N: NodeStateMut> Deserializer<R, I, N> {
    pub fn node(
        &mut self,
        class_id: u32,
        read_fn: impl FnOnce(&mut Self) -> Result<()>,
    ) -> Result<()> {
        match self.node_or_null(class_id, read_fn)? {
            None => todo!(),
            Some(id) => Ok(id),
        }
    }

    /// Deserialize a node with the given `class_id` using the given `read_fn`.
    pub fn node_or_null(
        &mut self,
        class_id: u32,
        read_fn: impl FnOnce(&mut Self) -> Result<()>,
    ) -> Result<Option<()>> {
        let index = self.u32()?;

        if index == 0xFFFFFFFF {
            return Ok(None);
        }

        if index == 0 || index > self.node_state.borrow().num_nodes {
            todo!()
        }

        if self.u32()? != class_id {
            todo!()
        }

        read_fn(self)?;

        Ok(Some(()))
    }
}
