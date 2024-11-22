use std::{
    any::Any,
    cell::OnceCell,
    io::Read,
    iter,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{Quat, Vec3};

use super::{Error, ErrorKind, ReadBody, TraceEntry};

pub struct IdState {
    seen_id: bool,
    ids: Vec<Arc<str>>,
}

impl IdState {
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

pub trait IdStateMut {
    fn get_mut(&mut self) -> &mut IdState;
}

impl IdStateMut for IdState {
    fn get_mut(&mut self) -> &mut IdState {
        self
    }
}

pub struct NodeState {
    node_refs: Box<[OnceCell<NodeRef<dyn Any + Send + Sync>>]>,
}

impl NodeState {
    pub fn new(num_node_refs: usize) -> Self {
        Self {
            node_refs: iter::repeat_with(OnceCell::new)
                .take(num_node_refs)
                .collect(),
        }
    }

    fn get_entry(&self, index: usize) -> Result<&OnceCell<NodeRef<dyn Any + Send + Sync>>, Error> {
        self.node_refs
            .get(index)
            .ok_or(Error::new(ErrorKind::Format("index")))
    }

    fn set_node_ref(
        &self,
        index: usize,
        node_ref: NodeRef<dyn Any + Send + Sync>,
    ) -> Result<(), Error> {
        self.get_entry(index)?
            .set(node_ref)
            .map_err(|_| Error::new(ErrorKind::Format("index")))
    }

    pub fn set_external_node_ref(
        &mut self,
        index: usize,
        external_node_ref: ExternalNodeRef,
    ) -> Result<(), Error> {
        self.set_node_ref(index, NodeRef::External(external_node_ref))
    }
}

pub enum NodeRef<T: ?Sized> {
    Internal { node: Arc<T> },
    External(ExternalNodeRef),
}

impl NodeRef<dyn Any + Send + Sync> {
    fn downcast<T: 'static + Any + Send + Sync>(self) -> Result<NodeRef<T>, Error> {
        match self {
            Self::Internal { node } => Ok(NodeRef::Internal {
                node: node
                    .downcast()
                    .map_err(|_| Error::new(ErrorKind::Format("node type")))?,
            }),
            Self::External(external_node_ref) => Ok(NodeRef::External(external_node_ref)),
        }
    }
}

impl<T: ?Sized> Clone for NodeRef<T> {
    fn clone(&self) -> Self {
        match *self {
            Self::Internal { ref node } => Self::Internal {
                node: Arc::clone(node),
            },
            Self::External(ref external_node_ref) => Self::External(external_node_ref.clone()),
        }
    }
}

impl<T: Default> Default for NodeRef<T> {
    fn default() -> Self {
        Self::Internal {
            node: Arc::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExternalNodeRef {
    pub(crate) path: Arc<Path>,
    pub(crate) ancestor_level: u8,
}

impl ExternalNodeRef {
    pub fn path(&self, source_path: &Path) -> PathBuf {
        let mut path = source_path.to_path_buf();

        path.pop();

        for _ in 0..self.ancestor_level {
            path.pop();
        }

        path.push(self.path.clone());

        path
    }
}

impl Default for ExternalNodeRef {
    fn default() -> Self {
        Self {
            path: PathBuf::new().into(),
            ancestor_level: 0,
        }
    }
}

pub trait NodeStateMut {
    fn get_mut(&mut self) -> &mut NodeState;
}

impl NodeStateMut for NodeState {
    fn get_mut(&mut self) -> &mut NodeState {
        self
    }
}

pub struct Reader<R, I, N> {
    inner: R,
    id_state: I,
    node_state: N,
}

impl<R, I, N> Reader<R, I, N> {
    pub const fn new(inner: R, id_state: I, node_state: N) -> Self {
        Self {
            inner,
            id_state,
            node_state,
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read, I, N> Reader<R, I, N> {
    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; n];

        self.inner
            .read_exact(&mut buf)
            .map_err(|io_err| Error::new(ErrorKind::Io(io_err)))?;

        Ok(buf)
    }

    pub fn byte_array<const S: usize>(&mut self) -> Result<[u8; S], Error> {
        let mut buf = [0; S];

        self.inner
            .read_exact(&mut buf)
            .map_err(|io_err| Error::new(ErrorKind::Io(io_err)))?;

        Ok(buf)
    }

    pub fn byte_buf(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.u32()?;

        self.bytes(size as usize)
    }

    pub fn i16(&mut self) -> Result<i16, Error> {
        let bytes = self.byte_array()?;

        Ok(i16::from_le_bytes(bytes))
    }

    pub fn u8(&mut self) -> Result<u8, Error> {
        let [byte] = self.byte_array()?;

        Ok(byte)
    }

    pub fn u16(&mut self) -> Result<u16, Error> {
        let bytes = self.byte_array()?;

        Ok(u16::from_le_bytes(bytes))
    }

    pub fn u32(&mut self) -> Result<u32, Error> {
        let bytes = self.byte_array()?;

        Ok(u32::from_le_bytes(bytes))
    }

    pub fn u64(&mut self) -> Result<u64, Error> {
        let bytes = self.byte_array()?;

        Ok(u64::from_le_bytes(bytes))
    }

    pub fn bool(&mut self) -> Result<bool, Error> {
        bool_from_u8(self.u32()? as u8)
    }

    pub fn bool8(&mut self) -> Result<bool, Error> {
        bool_from_u8(self.u8()?)
    }

    pub fn string(&mut self) -> Result<String, Error> {
        let bytes = self.byte_buf()?;

        String::from_utf8(bytes).map_err(|_| Error::new(ErrorKind::Format("not utf8")))
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        mut read_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        iter::repeat_with(|| read_fn(self)).take(n).collect()
    }

    pub fn expect_eof(&mut self) -> Result<(), Error> {
        let mut buf = [0];

        let n = self
            .inner
            .read(&mut buf)
            .map_err(|io_err| Error::new(ErrorKind::Io(io_err)))?;

        if n != 0 {
            return Err(Error::new(ErrorKind::Format("expected EOF")));
        }

        Ok(())
    }

    pub fn f32(&mut self) -> Result<f32, Error> {
        let bytes = self.byte_array()?;

        Ok(f32::from_le_bytes(bytes))
    }

    pub fn vec3(&mut self) -> Result<Vec3, Error> {
        let x = self.f32()?;
        let y = self.f32()?;
        let z = self.f32()?;

        Ok(Vec3 { x, y, z })
    }

    pub fn quat(&mut self) -> Result<Quat, Error> {
        let x = self.f32()?;
        let y = self.f32()?;
        let z = self.f32()?;
        let w = self.f32()?;

        Ok(Quat { x, y, z, w })
    }

    pub fn box3d(&mut self) -> Result<(), Error> {
        self.f32()?;
        self.f32()?;
        self.f32()?;
        self.f32()?;
        self.f32()?;
        self.f32()?;

        Ok(())
    }

    pub fn list<T>(
        &mut self,
        mut read_elem_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        iter::repeat_with(|| read_elem_fn(self))
            .take(len as usize)
            .collect()
    }

    pub fn list_with_version<T>(
        &mut self,
        read_elem_fn: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let version = self.u32()?;

        if version != 10 {
            return Err(Error::new(ErrorKind::Unsupported("list version")));
        }

        self.list(read_elem_fn)
    }
}

fn bool_from_u8(value: u8) -> Result<bool, Error> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::new(ErrorKind::Format("expected a boolean"))),
    }
}

impl<R: Read, I: IdStateMut, N> Reader<R, I, N> {
    pub fn id_or_null(&mut self) -> Result<Option<Arc<str>>, Error> {
        if !self.id_state.get_mut().seen_id {
            let version = self.u32()?;

            if version != 3 {
                return Err(Error::new(ErrorKind::Unsupported("identifier version")));
            }

            self.id_state.get_mut().seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index == 0x0000001a {
            return Ok(Some(Arc::from("Unassigned")));
        }

        if index & 0x40000000 == 0 {
            return Err(Error::new(ErrorKind::Format("index")));
        }

        let index = index & 0x3fffffff;

        let id = match index.checked_sub(1) {
            Some(index) => {
                let id = self
                    .id_state
                    .get_mut()
                    .ids
                    .get(index as usize)
                    .ok_or(Error::new(ErrorKind::Format("index")))?;

                Arc::clone(id)
            }
            None => {
                let id = Arc::from(self.string()?);
                self.id_state.get_mut().ids.push(Arc::clone(&id));

                id
            }
        };

        Ok(Some(id))
    }

    pub fn id(&mut self) -> Result<Arc<str>, Error> {
        match self.id_or_null()? {
            Some(id) => Ok(id),
            None => Err(Error::new(ErrorKind::Unsupported("null identifier"))),
        }
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> Reader<R, I, N> {
    pub fn node_ref_or_null<T: 'static + ReadBody>(&mut self) -> Result<Option<NodeRef<T>>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let ref_index = index
            .checked_sub(1)
            .ok_or(Error::new(ErrorKind::Format("index")))?;

        match self
            .node_state
            .get_mut()
            .get_entry(ref_index as usize)?
            .get()
        {
            Some(node_ref) => Ok(Some(node_ref.clone().downcast()?)),
            entry => {
                let class_id = self.u32()?;

                if class_id != T::CLASS_ID {
                    return Err(Error::new(ErrorKind::Format("class id")));
                }

                let mut node = T::default();

                match node.read_body(self) {
                    Ok(()) => {}
                    Err(mut error) => {
                        error.trace.push_front(TraceEntry {
                            class_id: T::CLASS_ID,
                            chunk_num: None,
                        });

                        return Err(error);
                    }
                }

                let node_ref: NodeRef<dyn Any + Send + Sync> = NodeRef::Internal {
                    node: Arc::new(node),
                };

                self.node_state
                    .get_mut()
                    .get_entry(ref_index as usize)?
                    .set(node_ref.clone())
                    .map_err(|_| Error::new(ErrorKind::Format("index")))?;

                Ok(Some(node_ref.downcast().unwrap()))
            }
        }
    }

    pub fn node_ref<T: 'static + ReadBody>(&mut self) -> Result<NodeRef<T>, Error> {
        let index = self.u32()?;

        let ref_index = index
            .checked_sub(1)
            .ok_or(Error::new(ErrorKind::Format("index")))?;

        match self
            .node_state
            .get_mut()
            .get_entry(ref_index as usize)?
            .get()
        {
            Some(node_ref) => Ok(node_ref.clone().downcast()?),
            entry => {
                let class_id = self.u32()?;

                if class_id != T::CLASS_ID {
                    return Err(Error::new(ErrorKind::Format("class id")));
                }

                let mut node = T::default();

                match node.read_body(self) {
                    Ok(()) => {}
                    Err(mut error) => {
                        error.trace.push_front(TraceEntry {
                            class_id: T::CLASS_ID,
                            chunk_num: None,
                        });

                        return Err(error);
                    }
                }

                let node_ref: NodeRef<dyn Any + Send + Sync> = NodeRef::Internal {
                    node: Arc::new(node),
                };

                self.node_state
                    .get_mut()
                    .get_entry(ref_index as usize)?
                    .set(node_ref.clone())
                    .map_err(|_| Error::new(ErrorKind::Format("index")))?;

                Ok(node_ref.downcast().unwrap())
            }
        }
    }

    pub fn internal_node_ref<T: 'static + ReadBody>(&mut self) -> Result<Arc<T>, Error> {
        match self.node_ref()? {
            NodeRef::Internal { node } => Ok(node),
            _ => Err(Error::new(ErrorKind::Format(
                "expected an internal node reference",
            ))),
        }
    }

    pub fn external_node_ref<T>(&mut self) -> Result<ExternalNodeRef, Error> {
        let index = self.u32()?;
        let ref_index = index
            .checked_sub(1)
            .ok_or(Error::new(ErrorKind::Format("index")))?;

        match self
            .node_state
            .get_mut()
            .get_entry(ref_index as usize)?
            .get()
        {
            Some(NodeRef::External(external_node_ref)) => Ok(external_node_ref.clone()),
            _ => Err(Error::new(ErrorKind::Format("wat"))),
        }
    }

    pub fn external_node_ref_or_null<T>(&mut self) -> Result<Option<ExternalNodeRef>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let ref_index = index
            .checked_sub(1)
            .ok_or(Error::new(ErrorKind::Format("index")))?;

        match self
            .node_state
            .get_mut()
            .get_entry(ref_index as usize)?
            .get()
        {
            Some(NodeRef::External(external_node_ref)) => Ok(Some(external_node_ref.clone())),
            _ => Err(Error::new(ErrorKind::Format("wat"))),
        }
    }
}
