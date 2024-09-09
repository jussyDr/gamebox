//! GameBox reader.

use std::{
    any::Any,
    cmp,
    io::{self, Read, Seek, SeekFrom},
    iter,
    rc::Rc,
};

use crate::{Error, Ident, Vec2};

use super::{file::read_body_chunks, readable::BodyChunks};

/// Identifier state.
pub struct IdState {
    seen_id: bool,
    ids: Vec<Rc<str>>,
}

impl IdState {
    /// Create a new [IdState].
    pub const fn new() -> Self {
        Self {
            seen_id: false,
            ids: vec![],
        }
    }
}

impl Default for IdState {
    fn default() -> Self {
        Self::new()
    }
}

/// Obtain a mutable [IdState].
pub trait IdStateMut {
    /// Get a mutable reference to an [IdState].
    fn get(&mut self) -> &mut IdState;
}

impl IdStateMut for IdState {
    fn get(&mut self) -> &mut IdState {
        self
    }
}

impl IdStateMut for &mut IdState {
    fn get(&mut self) -> &mut IdState {
        self
    }
}

/// Node state.
pub struct NodeState {
    nodes: Box<[Option<Rc<dyn Any>>]>,
}

impl NodeState {
    /// Create a new [NodeState] with the specified `num_nodes`.
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: vec![None; num_nodes].into_boxed_slice(),
        }
    }
}

/// Obtain a mutable [NodeStateMut].
pub trait NodeStateMut {
    /// Get a mutable reference to a [NodeState].
    fn get(&mut self) -> &mut NodeState;
}

impl NodeStateMut for NodeState {
    fn get(&mut self) -> &mut NodeState {
        self
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

/// GameBox reader.
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
    pub fn take<IS, NS>(
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
        let bytes = self.byte_array()?;

        Ok(f32::from_le_bytes(bytes))
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
    pub fn vec2(&mut self) -> Result<Vec2, Error> {
        let x = self.f32()?;
        let y = self.f32()?;

        Ok(Vec2 { x, y })
    }

    /// Read a 32-bit length and the corresponding bytes.
    pub fn byte_buf(&mut self) -> Result<Box<[u8]>, Error> {
        let len = self.u32()?;

        self.bytes(len as usize)
    }

    /// Read a UTF-8 string.
    pub fn string(&mut self) -> Result<String, Error> {
        let bytes = self.byte_buf()?;

        String::from_utf8(bytes.into()).map_err(|_| Error)
    }

    /// Read a 32-bit length and the corresponding list.
    pub fn list<T>(
        &mut self,
        read_elem: impl Fn(&mut Self) -> Result<T, Error>,
    ) -> Result<Box<[T]>, Error> {
        let len = self.u32()?;

        iter::repeat_with(|| read_elem(self))
            .take(len as usize)
            .collect()
    }

    /// Checks if this reader is at EOF.
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

impl<R: Read, I: IdStateMut, N> Reader<R, I, N> {
    /// Read a identifier.
    pub fn id(&mut self) -> Result<Option<Rc<str>>, Error> {
        if !self.id_state.get().seen_id {
            let version = self.u32()?;

            if version != 3 {
                return Err(Error);
            }

            self.id_state.get().seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index & 0x40000000 == 0 {
            return Err(Error);
        }

        let index = index & 0x3fffffff;

        let id = if index == 0 {
            let id = Rc::from(self.string()?);

            self.id_state.get().ids.push(Rc::clone(&id));

            id
        } else {
            let index = index - 1;

            let id = self.id_state.get().ids.get(index as usize).ok_or(Error)?;

            Rc::clone(id)
        };

        Ok(Some(id))
    }

    /// Read a identifier triple.
    pub fn ident(&mut self) -> Result<Ident, Error> {
        let id = self.id()?;

        let collection = self.u32()?;

        if collection != 0xffffffff && collection != 26 {
            return Err(Error);
        }

        let author = self.id()?;

        Ok(Ident { id, author })
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> Reader<R, I, N> {
    /// Read a node of type `T`.
    pub fn node<T: Default + BodyChunks + 'static>(&mut self) -> Result<Option<Rc<T>>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index == 0 {
            return Err(Error);
        }

        let index = index - 1;

        let slot = self
            .node_state
            .get()
            .nodes
            .get(index as usize)
            .ok_or(Error)?;

        let node = match slot {
            None => {
                let mut node = T::default();

                read_body_chunks(&mut node, self)?;

                let node: Rc<dyn Any> = Rc::new(node);

                let slot = self
                    .node_state
                    .get()
                    .nodes
                    .get_mut(index as usize)
                    .expect("slot empty");

                *slot = Some(Rc::clone(&node));

                node.downcast().expect("failed to downcast")
            }
            Some(node) => {
                let node = node.downcast_ref().ok_or(Error)?;

                Rc::clone(node)
            }
        };

        Ok(Some(node))
    }
}
