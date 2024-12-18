use std::{
    any::Any,
    io::{Read, Seek},
    iter,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    read::{Error, ErrorKind, ReadBody, TraceEntry},
    Class,
};

use super::{IdStateMut, Reader};

/// Node state.
pub struct NodeState {
    node_refs: Box<[Option<NodeRef<dyn Any + Send + Sync>>]>,
}

impl NodeState {
    pub fn new(num_node_refs: usize) -> Self {
        Self {
            node_refs: iter::repeat_with(|| None).take(num_node_refs).collect(),
        }
    }

    fn get_entry(
        &mut self,
        index: usize,
    ) -> Result<&mut Option<NodeRef<dyn Any + Send + Sync>>, Error> {
        self.node_refs
            .get_mut(index)
            .ok_or(Error::new(ErrorKind::Format("index zz".into())))
    }

    fn set_node_ref(
        &mut self,
        index: usize,
        node_ref: NodeRef<dyn Any + Send + Sync>,
    ) -> Result<(), Error> {
        match self.get_entry(index)? {
            Some(_) => Err(Error::new(ErrorKind::Format("index yy".into()))),
            entry => {
                *entry = Some(node_ref);

                Ok(())
            }
        }
    }

    pub fn set_external_node_ref(
        &mut self,
        index: usize,
        external_node_ref: ExternalNodeRef,
    ) -> Result<(), Error> {
        self.set_node_ref(index, NodeRef::External(external_node_ref))
    }
}

/// Reference to a node.
#[derive(Debug)]
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
                    .map_err(|_| Error::new(ErrorKind::Format("node type".into())))?,
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

/// Reference to a node in an external file.
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

pub struct NullNodeState;

impl NodeStateMut for NullNodeState {
    fn get_mut(&mut self) -> &mut NodeState {
        unimplemented!()
    }
}

impl<R: Read, I, N: NodeStateMut> Reader<R, I, N> {
    pub fn external_node_ref<T>(&mut self) -> Result<ExternalNodeRef, Error> {
        let index = self.u32()?;
        let ref_index = index
            .checked_sub(1)
            .ok_or(Error::new(ErrorKind::Format("index q".into())))?;

        match self.node_state.get_mut().get_entry(ref_index as usize)? {
            Some(NodeRef::External(external_node_ref)) => Ok(external_node_ref.clone()),
            _ => Err(Error::new(ErrorKind::Format("wat".into()))),
        }
    }

    pub fn external_node_ref_or_null<T>(&mut self) -> Result<Option<ExternalNodeRef>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let ref_index = index
            .checked_sub(1)
            .ok_or(Error::new(ErrorKind::Format("index v".into())))?;

        match self.node_state.get_mut().get_entry(ref_index as usize)? {
            Some(NodeRef::External(external_node_ref)) => Ok(Some(external_node_ref.clone())),
            _ => Err(Error::new(ErrorKind::Format("wat".into()))),
        }
    }
}

impl<R: Read + Seek, I: IdStateMut, N: NodeStateMut> Reader<R, I, N> {
    pub fn node_or_null<T: 'static + Class + ReadBody>(&mut self) -> Result<Option<T>, Error> {
        let class_id = self.u32()?;

        if class_id == 0xffffffff {
            return Ok(None);
        }

        if class_id != T::CLASS_ID {
            return Err(Error::new(ErrorKind::Format("class id".into())));
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

        Ok(Some(node))
    }

    pub fn node<T: 'static + Class + ReadBody>(&mut self) -> Result<T, Error> {
        match self.node_or_null()? {
            Some(node) => Ok(node),
            None => Err(Error::new(ErrorKind::Format("".into()))),
        }
    }

    pub fn node_ref_or_null<T: 'static + Class + ReadBody>(
        &mut self,
    ) -> Result<Option<NodeRef<T>>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let ref_index = index
            .checked_sub(1)
            .ok_or(Error::new(ErrorKind::Format("index e".into())))?;

        match self.node_state.get_mut().get_entry(ref_index as usize)? {
            Some(node_ref) => Ok(Some(node_ref.clone().downcast()?)),
            entry => {
                let class_id = self.u32()?;

                if class_id != T::CLASS_ID {
                    return Err(Error::new(ErrorKind::Format("class id".into())));
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
                    .set_node_ref(ref_index as usize, node_ref.clone())?;

                Ok(Some(node_ref.downcast().unwrap()))
            }
        }
    }

    pub fn node_ref<T: 'static + Class + ReadBody>(&mut self) -> Result<NodeRef<T>, Error> {
        let index = self.u32()?;

        let ref_index = index
            .checked_sub(1)
            .ok_or(Error::new(ErrorKind::Format("index b".into())))?;

        match self.node_state.get_mut().get_entry(ref_index as usize)? {
            Some(node_ref) => Ok(node_ref.clone().downcast()?),
            entry => {
                let class_id = self.u32()?;

                if class_id != T::CLASS_ID {
                    return Err(Error::new(ErrorKind::Format("class id".into())));
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
                    .set_node_ref(ref_index as usize, node_ref.clone())?;

                Ok(node_ref.downcast().unwrap())
            }
        }
    }

    pub fn internal_node_ref<T: 'static + Class + ReadBody>(&mut self) -> Result<Arc<T>, Error> {
        match self.node_ref()? {
            NodeRef::Internal { node } => Ok(node),
            _ => Err(Error::new(ErrorKind::Format(
                "expected an internal node reference".into(),
            ))),
        }
    }

    pub fn internal_node_ref_or_null<T: 'static + Class + ReadBody>(
        &mut self,
    ) -> Result<Option<Arc<T>>, Error> {
        match self.node_ref_or_null()? {
            Some(NodeRef::Internal { node }) => Ok(Some(node)),
            None => Ok(None),
            _ => Err(Error::new(ErrorKind::Format(
                "expected an internal node reference".into(),
            ))),
        }
    }

    pub fn test_or_ext<T>(
        &mut self,
        mut read_fn: impl FnMut(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<(), Error> {
        let index = self.u32()? - 1;

        match self.node_state.get_mut().get_entry(index as usize)? {
            None => {
                let class_id = self.u32()?;
                let node = Arc::new(read_fn(self, class_id)?);

                Ok(())
            }
            Some(node_ref) => match node_ref {
                NodeRef::Internal { .. } => todo!(),
                NodeRef::External(_) => Ok(()),
            },
        }
    }

    pub fn test_or_ext_or_null<T>(
        &mut self,
        mut read_fn: impl FnMut(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<(), Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(());
        }

        let index = index - 1;

        match self.node_state.get_mut().get_entry(index as usize)? {
            None => {
                let class_id = self.u32()?;
                let node = Arc::new(read_fn(self, class_id)?);

                Ok(())
            }
            Some(node_ref) => match node_ref {
                NodeRef::Internal { .. } => todo!(),
                NodeRef::External(_) => Ok(()),
            },
        }
    }

    pub fn test<T>(
        &mut self,
        mut read_fn: impl FnMut(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<Arc<T>, Error> {
        let index = self.u32()? - 1;

        match self.node_state.get_mut().get_entry(index as usize)? {
            None => {
                let class_id = self.u32()?;
                let node = Arc::new(read_fn(self, class_id)?);

                Ok(node)
            }
            Some(node_ref) => match node_ref {
                NodeRef::Internal { .. } => todo!(),
                NodeRef::External(_) => todo!(),
            },
        }
    }

    pub fn test_or_null<T>(
        &mut self,
        mut read_fn: impl FnMut(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<Option<Arc<T>>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index - 1;

        match self.node_state.get_mut().get_entry(index as usize)? {
            None => {
                let class_id = self.u32()?;
                let node = Arc::new(read_fn(self, class_id)?);

                Ok(Some(node))
            }
            Some(node_ref) => match node_ref {
                NodeRef::Internal { .. } => todo!(),
                NodeRef::External(_) => todo!(),
            },
        }
    }
}
