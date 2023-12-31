use std::{
    any::Any,
    cell::{Cell, OnceCell},
    cmp,
    io::{self, BufRead, Read, Seek, SeekFrom},
    iter,
    mem::size_of,
    path::{Path, PathBuf},
    rc::Rc,
};

use elsa::FrozenVec;

use crate::{
    common::{ClassId, ID_FLAG_BIT, ID_INDEX_MASK, ID_VERSION},
    read::Result,
};

use super::readable::ReadBody;

pub struct IdState {
    seen_id: Cell<bool>,
    ids: FrozenVec<Rc<str>>,
}

impl IdState {
    pub fn new() -> Self {
        Self {
            seen_id: Cell::new(false),
            ids: FrozenVec::new(),
        }
    }
}

pub trait IdStateRef {
    fn borrow(&self) -> &IdState;
}

impl IdStateRef for IdState {
    fn borrow(&self) -> &IdState {
        self
    }
}

impl<T: IdStateRef> IdStateRef for &T {
    fn borrow(&self) -> &IdState {
        (**self).borrow()
    }
}

pub enum NodeRef<T: ?Sized> {
    Internal { node: Rc<T> },
    External { path: Rc<Path> },
}

pub struct NodeState {
    nodes: Box<[OnceCell<NodeRef<dyn Any>>]>,
}

impl NodeState {
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: repeat_n_with(num_nodes, OnceCell::new),
        }
    }

    pub fn get(&self, index: usize) -> Result<&OnceCell<NodeRef<dyn Any>>> {
        self.nodes
            .get(index - 1)
            .ok_or("node index out of range".into())
    }

    pub fn set(&self, index: usize, node_ref: NodeRef<dyn Any>) -> Result<()> {
        self.get(index)?
            .set(node_ref)
            .map_err(|_| "node already read".into())
    }

    pub fn set_ref(&mut self, index: usize, path: PathBuf) -> Result<()> {
        self.set(
            index,
            NodeRef::External {
                path: Rc::from(path),
            },
        )
    }
}

pub trait NodeStateRef {
    fn borrow(&self) -> &NodeState;
}

impl NodeStateRef for NodeState {
    fn borrow(&self) -> &NodeState {
        self
    }
}

impl<T: NodeStateRef> NodeStateRef for &T {
    fn borrow(&self) -> &NodeState {
        (**self).borrow()
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
        unimplemented!()
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

    pub fn take(&mut self, limit: u64) -> Deserializer<Take<&mut R>, &I, &N> {
        let inner = Take {
            reader: &mut self.reader,
            limit,
        };

        Deserializer::new(inner, &self.id_state, &self.node_state)
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
        let x = self.u8()?;
        bool_from_u8(x)
    }

    pub fn bool32(&mut self) -> Result<bool> {
        let x = self.u32()?;
        bool_from_u8(x as u8)
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
        let string = String::from_utf8(bytes)?;
        Ok(string)
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<T>> {
        repeat_n_with(n, || read_fn(self))
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
            return Err("list sizes do not match".into());
        }

        vec.into_iter().map(|x| read_fn(self, x)).collect()
    }

    pub fn eof(&mut self) -> Result<()> {
        let mut buf = [0];

        match self.reader.read(&mut buf) {
            Ok(0) => Ok(()),
            _ => Err("expected end of reader".into()),
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

impl<R: Read, I: IdStateRef, N> Deserializer<R, I, N> {
    pub fn null_id(&mut self) -> Result<()> {
        let index = read_id_index(self)?;

        if index != 0xffffffff {
            return Err("expected null id".into());
        }

        Ok(())
    }

    pub fn id(&mut self) -> Result<Rc<str>> {
        match self.id_or_null()? {
            None => Err("id is null".into()),
            Some(id) => Ok(id),
        }
    }

    pub fn id_or_null(&mut self) -> Result<Option<Rc<str>>> {
        let index = read_id_index(self)?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index & !ID_INDEX_MASK == ID_FLAG_BIT {
            let index = (index & ID_INDEX_MASK) as u16;

            if index == 0 {
                let id = Rc::from(self.string()?);
                self.id_state.borrow().ids.push(Rc::clone(&id));

                Ok(Some(id))
            } else {
                let id = self
                    .id_state
                    .borrow()
                    .ids
                    .get_clone(index as usize - 1)
                    .ok_or("no id with given index")?;

                Ok(Some(id))
            }
        } else {
            Err("expected id".into())
        }
    }
}

impl<R: Read, I, N: NodeStateRef> Deserializer<R, I, N> {
    pub fn external_node_ref(&mut self) -> Result<Rc<Path>> {
        let index = match self.u32()? {
            0xffffffff => return Err("node index is null".into()),
            index => index,
        };

        let node_ref = self
            .node_state
            .borrow()
            .get(index as usize)?
            .get()
            .ok_or("node is null")?;

        match node_ref {
            NodeRef::Internal { .. } => Err("expected external node ref".into()),
            NodeRef::External { path } => Ok(Rc::clone(path)),
        }
    }
}

impl<R: Read + Seek, I: IdStateRef, N: NodeStateRef> Deserializer<R, I, N> {
    pub fn internal_node_ref<T: 'static + Default + ClassId + ReadBody>(
        &mut self,
    ) -> Result<Rc<T>> {
        match self.internal_node_ref_or_null()? {
            None => Err("node is null".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn internal_node_ref_or_null<T: 'static + Default + ClassId + ReadBody>(
        &mut self,
    ) -> Result<Option<Rc<T>>> {
        match self.node_ref_or_null()? {
            None => Ok(None),
            Some(NodeRef::Internal { node }) => Ok(Some(node)),
            Some(NodeRef::External { .. }) => Err("expected internal node ref".into()),
        }
    }

    pub fn node_ref<T: 'static + Default + ClassId + ReadBody>(&mut self) -> Result<NodeRef<T>> {
        match self.node_ref_or_null()? {
            None => Err("node is null".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn node_ref_or_null<T: 'static + Default + ClassId + ReadBody>(
        &mut self,
    ) -> Result<Option<NodeRef<T>>> {
        let index = match self.u32()? {
            0xffffffff => return Ok(None),
            index => index,
        };

        match self.node_state.borrow().get(index as usize)?.get() {
            None => {
                let class_id = self.u32()?;

                if class_id != T::class_id() {
                    return Err("class id does not match".into());
                }

                let mut node = T::default();

                T::read_body(&mut node, self)?;

                let node = Rc::new(node);

                self.node_state.borrow().set(
                    index as usize,
                    NodeRef::Internal {
                        node: Rc::<T>::clone(&node),
                    },
                )?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::Internal { node }) => {
                let node: Rc<T> = Rc::clone(node).downcast().map_err(|_| "wrong node type")?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::External { path }) => Ok(Some(NodeRef::External {
                path: Rc::clone(path),
            })),
        }
    }

    pub fn node<T: Default + ClassId + ReadBody>(&mut self) -> Result<T> {
        match self.node_or_null()? {
            None => Err("node is null".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn node_or_null<T: Default + ClassId + ReadBody>(&mut self) -> Result<Option<T>> {
        let class_id = self.u32()?;

        if class_id == 0xffffffff {
            return Ok(None);
        }

        if class_id != T::class_id() {
            return Err("class id does not match".into());
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
            None => Err("node is null".into()),
            Some(NodeRef::Internal { node }) => Ok(node),
            Some(NodeRef::External { .. }) => Err("expected internal node ref".into()),
        }
    }

    pub fn any_node_ref_or_null(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Rc<dyn Any>>,
    ) -> Result<Option<NodeRef<dyn Any>>> {
        let index = match self.u32()? {
            0xffffffff => return Ok(None),
            index => index,
        };

        match self.node_state.borrow().get(index as usize)?.get() {
            None => {
                let class_id = self.u32()?;

                let node = read_fn(self, class_id)?;

                self.node_state.borrow().set(
                    index as usize,
                    NodeRef::Internal {
                        node: Rc::clone(&node),
                    },
                )?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::Internal { node }) => Ok(Some(NodeRef::Internal {
                node: Rc::clone(node),
            })),
            Some(NodeRef::External { path }) => Ok(Some(NodeRef::External {
                path: Rc::clone(path),
            })),
        }
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

fn read_id_index<R: Read, I: IdStateRef, N>(d: &mut Deserializer<R, I, N>) -> Result<u32> {
    if !d.id_state.borrow().seen_id.get() {
        if d.u32()? != ID_VERSION {
            return Err("invalid identifier version".into());
        }

        d.id_state.borrow().seen_id.set(true);
    }

    d.u32()
}
