use std::{
    any::Any,
    borrow::BorrowMut,
    cmp,
    io::{self, Read, Seek, SeekFrom},
    iter,
    path::{Path, PathBuf},
};

use crate::{
    class::Class,
    read::{read_body_chunks, readable::BodyChunks, Result},
};

/// State of identifiers read in the past.
#[derive(Default)]
pub struct IdState {
    seen_id: bool,
    ids: Vec<String>,
}

/// Trait which should be used as a generic trait bound in
/// functions that need to access the `IdState`.
pub trait IdStateMut {
    fn borrow(&self) -> &IdState;
    fn borrow_mut(&mut self) -> &mut IdState;
}

impl IdStateMut for IdState {
    fn borrow(&self) -> &IdState {
        self
    }

    fn borrow_mut(&mut self) -> &mut IdState {
        self
    }
}

impl<T: IdStateMut> IdStateMut for &mut T {
    fn borrow(&self) -> &IdState {
        (**self).borrow()
    }

    fn borrow_mut(&mut self) -> &mut IdState {
        (**self).borrow_mut()
    }
}

/// State of nodes read in the past.
pub struct NodeState {
    nodes: Vec<Option<Node<Box<dyn Any>>>>,
}

impl NodeState {
    /// Create a new `NodeState` with a node limit of `num_nodes`.
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: iter::repeat_with(|| None).take(num_nodes).collect(),
        }
    }

    pub fn set_ref(&mut self, index: usize, path: PathBuf) {
        self.nodes[index - 1] = Some(Node::Ref(path));
    }
}

pub enum Node<T> {
    Inline(T),
    Ref(PathBuf),
}

/// Trait which should be used as a generic trait bound in
/// functions that need to access the `NodeState`.
pub trait NodeStateMut {
    fn borrow(&self) -> &NodeState;
    fn borrow_mut(&mut self) -> &mut NodeState;
}

impl NodeStateMut for NodeState {
    fn borrow(&self) -> &NodeState {
        self
    }

    fn borrow_mut(&mut self) -> &mut NodeState {
        self
    }
}

impl<T: NodeStateMut> NodeStateMut for &mut T {
    fn borrow(&self) -> &NodeState {
        (**self).borrow()
    }

    fn borrow_mut(&mut self) -> &mut NodeState {
        (**self).borrow_mut()
    }
}

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

    pub fn into_reader(self) -> R {
        self.reader
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

    pub fn bool32(&mut self) -> Result<bool> {
        match self.u32()? {
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
    pub fn list<T>(&mut self, read_fn: impl FnMut(&mut Self) -> Result<T>) -> Result<Vec<T>> {
        let len = self.u32()?;
        self.repeat(len as usize, read_fn)
    }

    /// Repeat the `read_fn` a total of `n` times.
    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T>,
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

    pub fn take2(&mut self, limit: u64) -> Deserializer<Take<&mut R>, &mut I, &mut N> {
        let inner = Take {
            reader: &mut self.reader,
            limit,
        };

        Deserializer::new(inner, &mut self.id_state, &mut self.node_state)
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

        if index & 0xffffc000 == 0x40000000 {
            let index = (index & 0x3fff) as u16 - 1;
            let id = self.id_state.borrow().ids[index as usize].clone();
            return Ok(Some(id));
        }

        todo!()
    }
}

impl<R: Read, I, N: NodeStateMut> Deserializer<R, I, N> {
    pub fn node_ref(&mut self) -> Result<&Path> {
        let index = self.u32()?;

        if index == 0 || index > self.node_state.borrow().nodes.len() as u32 {
            todo!()
        }

        let r = self.node_state.borrow().nodes[index as usize - 1]
            .as_ref()
            .unwrap();

        match r {
            Node::Ref(q) => Ok(q),
            _ => todo!(),
        }
    }

    pub fn flat_inline_node<T>(
        &mut self,
        class_id: u32,
        read_fn: impl Fn(&mut Self) -> Result<T>,
    ) -> Result<T> {
        let index = self.u32()?;

        if index == 0xFFFFFFFF {
            todo!()
        }

        if index == 0 || index > self.node_state.borrow().nodes.len() as u32 {
            todo!()
        }

        if self.u32()? != class_id {
            todo!()
        }

        let node = read_fn(self)?;

        Ok(node)
    }

    pub fn flat_node<T>(
        &mut self,
        class_id: u32,
        read_fn: impl Fn(&mut Self) -> Result<T>,
    ) -> Result<Node<()>> {
        let index = self.u32()?;

        if index == 0xFFFFFFFF {
            todo!()
        }

        if index == 0 || index > self.node_state.borrow().nodes.len() as u32 {
            todo!()
        }

        if self.node_state.borrow().nodes[index as usize - 1].is_some() {
            let r = self.node_state.borrow().nodes[index as usize - 1]
                .as_ref()
                .unwrap();

            match r {
                Node::Inline(q) => return Ok(Node::Inline(())),
                Node::Ref(q) => return Ok(Node::Ref(q.to_path_buf())),
                _ => todo!(),
            }
        }

        if self.u32()? != class_id {
            todo!()
        }

        let node = read_fn(self)?;

        Ok(Node::Inline(()))
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> Deserializer<R, I, N> {
    /// An inline node of type `T`.
    pub fn inline_node<T: 'static + Default + Class + BodyChunks>(&mut self) -> Result<&T> {
        match self.inline_node_or_null()? {
            Some(node) => Ok(node),
            _ => todo!(),
        }
    }

    /// Either an inline node of type `T` or null.
    pub fn inline_node_or_null<T: 'static + Default + Class + BodyChunks>(
        &mut self,
    ) -> Result<Option<&T>> {
        match self.node_or_null()? {
            None => Ok(None),
            Some(Node::Inline(node)) => Ok(Some(node)),
            _ => todo!(),
        }
    }

    /// Either an inline node of type `T` or a node reference.
    pub fn node<T: 'static + Default + Class + BodyChunks>(&mut self) -> Result<Node<&T>> {
        match self.node_or_null()? {
            None => todo!(),
            Some(node) => Ok(node),
        }
    }

    /// Either an inline node of type `T`, a node reference, or null.
    pub fn node_or_null<T: 'static + Default + Class + BodyChunks>(
        &mut self,
    ) -> Result<Option<Node<&T>>> {
        let index = self.u32()?;

        if index == 0xFFFFFFFF {
            return Ok(None);
        }

        if index == 0 || index > self.node_state.borrow().nodes.len() as u32 {
            todo!()
        }

        if self.node_state.borrow().nodes[index as usize - 1].is_some() {
            let r = self.node_state.borrow().nodes[index as usize - 1]
                .as_ref()
                .unwrap();

            match r {
                Node::Inline(q) => return Ok(Some(Node::Inline(q.downcast_ref().unwrap()))),
                Node::Ref(q) => return Ok(Some(Node::Ref(q.to_path_buf()))),
                _ => todo!(),
            }
        }

        if self.u32()? != T::class_id() {
            todo!()
        }

        let mut node = T::default();
        read_body_chunks(&mut node, self)?;

        self.node_state.borrow_mut().nodes[index as usize - 1] = Some(Node::Inline(Box::new(node)));

        let r = self.node_state.borrow().nodes[index as usize - 1]
            .as_ref()
            .unwrap();

        match r {
            Node::Inline(q) => Ok(Some(Node::Inline(q.downcast_ref().unwrap()))),
            _ => todo!(),
        }
    }

    /// Either any inline node or null.
    pub fn any_inline_node_or_null<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T>,
    ) -> Result<Option<T>> {
        let index = self.u32()?;

        if index == 0xFFFFFFFF {
            return Ok(None);
        }

        if index == 0 || index > self.node_state.borrow().nodes.len() as u32 {
            todo!()
        }

        let class_id = self.u32()?;

        let node = read_fn(self, class_id)?;

        Ok(Some(node))
    }

    /// Either any inline node, a node reference, or null.
    pub fn any_node_or_null<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T>,
    ) -> Result<Option<Node<()>>> {
        let index = self.u32()?;

        if index == 0xFFFFFFFF {
            return Ok(None);
        }

        if index == 0 || index > self.node_state.borrow().nodes.len() as u32 {
            todo!()
        }

        if self.node_state.borrow().nodes[index as usize - 1].is_some() {
            let r = self.node_state.borrow().nodes[index as usize - 1]
                .as_ref()
                .unwrap();

            match r {
                Node::Inline(q) => return Ok(Some(Node::Inline(()))),
                Node::Ref(q) => return Ok(Some(Node::Ref(q.to_path_buf()))),
                _ => todo!(),
            }
        }

        let class_id = self.u32()?;

        let node = read_fn(self, class_id)?;

        Ok(Some(Node::Inline(())))
    }
}
