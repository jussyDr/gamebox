use std::{
    any::Any,
    cmp,
    io::{self, BufRead, Read, Seek, SeekFrom},
    iter,
    mem::size_of,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{class::ClassId, read::Result};

use super::readable::ReadBody;

pub struct IdState {
    seen_id: bool,
    ids: Vec<Rc<str>>,
}

impl IdState {
    pub fn new() -> Self {
        Self {
            seen_id: false,
            ids: Vec::new(),
        }
    }
}

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

pub enum NodeRef<T: ?Sized> {
    Internal(Rc<T>),
    External { path: Rc<Path> },
}

pub struct NodeState {
    nodes: Box<[Option<NodeRef<dyn Any>>]>,
}

impl NodeState {
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: iter::repeat_with(|| None).take(num_nodes).collect(),
        }
    }

    pub fn set_ref(&mut self, index: usize, path: PathBuf) {
        let entry = self.nodes.get_mut(index - 1).unwrap();

        if entry.is_some() {
            todo!()
        }

        *entry = Some(NodeRef::External { path: path.into() })
    }
}

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
        todo!();
    }
}

pub struct Deserializer<R, I, N> {
    reader: R,
    id_state: I,
    node_state: N,
}

impl<R, I, N> Deserializer<R, I, N> {
    pub fn new(reader: R, id_state: I, node_state: N) -> Self {
        Self {
            reader,
            id_state,
            node_state,
        }
    }

    pub fn into_inner(self) -> R {
        self.reader
    }

    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    pub fn take(&mut self, limit: u64) -> Deserializer<Take<&mut R>, &mut I, &mut N> {
        let inner = Take {
            reader: &mut self.reader,
            limit,
        };

        Deserializer::new(inner, &mut self.id_state, &mut self.node_state)
    }

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
    #[inline]
    pub fn u8(&mut self) -> Result<u8> {
        let bytes = self.byte_array()?;
        Ok(u8::from_le_bytes(bytes))
    }

    #[inline]
    pub fn u16(&mut self) -> Result<u16> {
        let bytes = self.byte_array()?;
        Ok(u16::from_le_bytes(bytes))
    }

    #[inline]
    pub fn u32(&mut self) -> Result<u32> {
        let bytes = self.byte_array()?;
        Ok(u32::from_le_bytes(bytes))
    }

    #[inline]
    pub fn u64(&mut self) -> Result<u64> {
        let bytes = self.byte_array()?;
        Ok(u64::from_le_bytes(bytes))
    }

    #[inline]
    pub fn i16(&mut self) -> Result<i16> {
        let bytes = self.byte_array()?;
        Ok(i16::from_le_bytes(bytes))
    }

    #[inline]
    pub fn i32(&mut self) -> Result<i32> {
        let bytes = self.byte_array()?;
        Ok(i32::from_le_bytes(bytes))
    }

    #[inline]
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

    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; n];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    #[inline]
    pub fn byte_array<const L: usize>(&mut self) -> Result<[u8; L]> {
        let mut array = [0; L];
        self.reader.read_exact(&mut array)?;
        Ok(array)
    }

    pub fn string(&mut self) -> Result<String> {
        let len = self.u32()?;
        let bytes = self.bytes(len as usize)?;
        let string = String::from_utf8(bytes).unwrap();
        Ok(string)
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<T>> {
        iter::repeat_with(|| read_fn(self)).take(n).collect()
    }

    pub fn list<T>(&mut self, read_fn: impl FnMut(&mut Self) -> Result<T>) -> Result<Vec<T>> {
        let len = self.u32()?;
        self.repeat(len as usize, read_fn)
    }

    pub fn list_zipped_with<T, U>(
        &mut self,
        vec: Vec<U>,
        mut read_fn: impl FnMut(&mut Self, U) -> Result<T>,
    ) -> Result<Vec<T>> {
        let len = self.u32()?;

        if len as usize != vec.len() {
            todo!()
        }

        vec.into_iter().map(|x| read_fn(self, x)).collect()
    }

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
    pub fn peek_u32(&mut self) -> Result<u32> {
        let value = self.u32()?;
        self.reader
            .seek(io::SeekFrom::Current(-(size_of::<u32>() as i64)))?;
        Ok(value)
    }
}

impl<R: Read, I: IdStateMut, N> Deserializer<R, I, N> {
    pub fn null_id(&mut self) -> Result<()> {
        let index = self.id_index()?;

        if index != 0xffffffff {
            todo!()
        }

        Ok(())
    }

    pub fn id(&mut self) -> Result<Rc<str>> {
        match self.id_or_null()? {
            None => todo!(),
            Some(id) => Ok(id),
        }
    }

    pub fn id_or_null(&mut self) -> Result<Option<Rc<str>>> {
        let index = self.id_index()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index & 0xffffc000 == 0x40000000 {
            let index = (index & 0x00003fff) as u16;

            if index == 0 {
                let id = Rc::from(self.string()?);
                self.id_state.borrow_mut().ids.push(Rc::clone(&id));

                Ok(Some(id))
            } else {
                let id = Rc::clone(self.id_state.borrow().ids.get(index as usize - 1).unwrap());

                Ok(Some(id))
            }
        } else {
            todo!()
        }
    }

    fn id_index(&mut self) -> Result<u32> {
        if !self.id_state.borrow().seen_id {
            let version = self.u32()?;

            if version != 3 {
                todo!()
            }

            self.id_state.borrow_mut().seen_id = true;
        }

        self.u32()
    }
}

impl<R: Read, I, N: NodeStateMut> Deserializer<R, I, N> {
    pub fn external_node_ref(&mut self) -> Result<Rc<Path>> {
        let index = match self.u32()? {
            0xffffffff => todo!(),
            index => index - 1,
        };

        let node_ref = self
            .node_state
            .borrow()
            .nodes
            .get(index as usize)
            .unwrap()
            .as_ref()
            .unwrap();

        match node_ref {
            NodeRef::Internal(_) => todo!(),
            NodeRef::External { path } => Ok(Rc::clone(path)),
        }
    }
}

impl<R: Read + Seek, I: IdStateMut, N: NodeStateMut> Deserializer<R, I, N> {
    pub fn internal_node_ref<T: 'static + Default + ClassId + ReadBody>(
        &mut self,
    ) -> Result<Rc<T>> {
        match self.internal_node_ref_or_null()? {
            None => todo!(),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn internal_node_ref_or_null<T: 'static + Default + ClassId + ReadBody>(
        &mut self,
    ) -> Result<Option<Rc<T>>> {
        match self.node_ref_or_null()? {
            None => Ok(None),
            Some(NodeRef::Internal(node_ref)) => Ok(Some(node_ref)),
            Some(NodeRef::External { .. }) => todo!(),
        }
    }

    pub fn node_ref<T: 'static + Default + ClassId + ReadBody>(&mut self) -> Result<NodeRef<T>> {
        match self.node_ref_or_null()? {
            None => todo!(),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn node_ref_or_null<T: 'static + Default + ClassId + ReadBody>(
        &mut self,
    ) -> Result<Option<NodeRef<T>>> {
        let index = match self.u32()? {
            0xffffffff => return Ok(None),
            index => index - 1,
        };

        let node_ref_entry = self.node_state.borrow().nodes.get(index as usize).unwrap();

        match node_ref_entry {
            None => {
                let class_id = self.u32()?;

                if class_id != T::class_id() {
                    todo!()
                }

                let mut node = T::default();

                T::read_body(&mut node, self)?;

                let node = Rc::new(node);

                let node_ref_entry = self
                    .node_state
                    .borrow_mut()
                    .nodes
                    .get_mut(index as usize)
                    .unwrap();

                if node_ref_entry.is_some() {
                    todo!()
                }

                *node_ref_entry = Some(NodeRef::Internal(Rc::<T>::clone(&node)));

                Ok(Some(NodeRef::Internal(node)))
            }
            Some(NodeRef::Internal(node_ref)) => Ok(Some(NodeRef::Internal(
                Rc::clone(node_ref).downcast().unwrap(),
            ))),
            Some(NodeRef::External { path }) => Ok(Some(NodeRef::External {
                path: Rc::clone(path),
            })),
        }
    }

    pub fn node<T: Default + ClassId + ReadBody>(&mut self) -> Result<T> {
        match self.node_or_null()? {
            None => todo!(),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn node_or_null<T: Default + ClassId + ReadBody>(&mut self) -> Result<Option<T>> {
        let class_id = self.u32()?;

        if class_id == 0xffffffff {
            return Ok(None);
        }

        if class_id != T::class_id() {
            todo!()
        }

        let mut node = T::default();

        T::read_body(&mut node, self)?;

        Ok(Some(node))
    }

    pub fn any_internal_node_ref(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Rc<dyn Any>>,
    ) -> Result<Rc<dyn Any>> {
        match self.any_node_ref_or_null(read_fn)? {
            None => todo!(),
            Some(NodeRef::Internal(node_ref)) => Ok(node_ref),
            Some(NodeRef::External { .. }) => todo!(),
        }
    }

    pub fn any_node_ref_or_null(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Rc<dyn Any>>,
    ) -> Result<Option<NodeRef<dyn Any>>> {
        let index = match self.u32()? {
            0xffffffff => return Ok(None),
            index => index - 1,
        };

        let node_ref_entry = self.node_state.borrow().nodes.get(index as usize).unwrap();

        match node_ref_entry {
            None => {
                let class_id = self.u32()?;

                let node = read_fn(self, class_id)?;

                let node_ref_entry = self
                    .node_state
                    .borrow_mut()
                    .nodes
                    .get_mut(index as usize)
                    .unwrap();

                if node_ref_entry.is_some() {
                    todo!()
                }

                *node_ref_entry = Some(NodeRef::Internal(Rc::clone(&node)));

                Ok(Some(NodeRef::Internal(node)))
            }
            Some(NodeRef::Internal(node_ref)) => Ok(Some(NodeRef::Internal(Rc::clone(node_ref)))),
            Some(NodeRef::External { path }) => Ok(Some(NodeRef::External {
                path: Rc::clone(path),
            })),
        }
    }
}
