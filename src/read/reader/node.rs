use std::{
    any::Any,
    io::{Read, Seek},
    iter,
    marker::PhantomData,
    sync::Arc,
};

use crate::{
    read::{Error, ErrorKind, ReadBody, TraceEntry},
    Class, ExternalNodeRef, NodeRef,
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
        external_node_ref: ExternalNodeRef<dyn Any + Send + Sync>,
    ) -> Result<(), Error> {
        self.set_node_ref(index, NodeRef::External(external_node_ref))
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
    fn get_entry_or_null(
        &mut self,
    ) -> Result<Option<(usize, &mut Option<NodeRef<dyn Any + Send + Sync>>)>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or(Error::new(ErrorKind::Format("node reference index".into())))?;

        let entry = self.node_state.get_mut().get_entry(index as usize)?;

        Ok(Some((index as usize, entry)))
    }

    pub fn external_node_ref_or_null<T>(&mut self) -> Result<Option<ExternalNodeRef<T>>, Error> {
        match self.get_entry_or_null()? {
            None => Ok(None),
            Some((_, Some(NodeRef::External(external_node_ref)))) => Ok(Some(ExternalNodeRef {
                ancestor_level: external_node_ref.ancestor_level,
                use_file: external_node_ref.use_file,
                path: Arc::clone(&external_node_ref.path),
                phantom: PhantomData,
            })),
            _ => Err(Error::new(ErrorKind::Format(
                "expected an external node reference".into(),
            ))),
        }
    }

    pub fn external_node_ref<T>(&mut self) -> Result<ExternalNodeRef<T>, Error> {
        match self.external_node_ref_or_null()? {
            Some(external_node_ref) => Ok(external_node_ref),
            None => Err(Error::new(ErrorKind::Format(
                "node reference is null".into(),
            ))),
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
        match self.get_entry_or_null()? {
            None => Ok(None),
            Some((_, Some(node_ref))) => match node_ref {
                NodeRef::Internal(internal_node_ref) => Ok(Some(NodeRef::Internal(
                    Arc::clone(&internal_node_ref).downcast().unwrap(),
                ))),
                NodeRef::External(external_node_ref) => {
                    Ok(Some(NodeRef::External(ExternalNodeRef {
                        ancestor_level: external_node_ref.ancestor_level,
                        use_file: external_node_ref.use_file,
                        path: Arc::clone(&external_node_ref.path),
                        phantom: PhantomData,
                    })))
                }
            },
            Some((index, _)) => {
                let node: Arc<dyn Any + Send + Sync> = Arc::new(self.node::<T>()?);

                self.node_state
                    .get_mut()
                    .set_node_ref(index, NodeRef::Internal(Arc::clone(&node)))?;

                Ok(Some(NodeRef::Internal(node.downcast().unwrap())))
            }
        }
    }

    pub fn node_ref<T: 'static + Class + ReadBody>(&mut self) -> Result<NodeRef<T>, Error> {
        match self.node_ref_or_null()? {
            Some(node_ref) => Ok(node_ref),
            None => Err(Error::new(ErrorKind::Format(
                "node reference is null".into(),
            ))),
        }
    }

    pub fn internal_node_ref<T: 'static + Class + ReadBody>(&mut self) -> Result<Arc<T>, Error> {
        match self.node_ref()? {
            NodeRef::Internal(node) => Ok(node),
            _ => Err(Error::new(ErrorKind::Format(
                "expected an internal node reference".into(),
            ))),
        }
    }

    pub fn internal_node_ref_or_null<T: 'static + Class + ReadBody>(
        &mut self,
    ) -> Result<Option<Arc<T>>, Error> {
        match self.node_ref_or_null()? {
            Some(NodeRef::Internal(node)) => Ok(Some(node)),
            None => Ok(None),
            _ => Err(Error::new(ErrorKind::Format(
                "expected an internal node reference".into(),
            ))),
        }
    }

    pub fn test_or_ext<T>(
        &mut self,
        mut read_fn: impl FnMut(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<Option<ExternalNodeRef<()>>, Error> {
        let index = self.u32()? - 1;

        match self.node_state.get_mut().get_entry(index as usize)? {
            None => {
                let class_id = self.u32()?;
                let node = Arc::new(read_fn(self, class_id)?);

                Ok(None)
            }
            Some(node_ref) => match node_ref {
                NodeRef::Internal { .. } => todo!(),
                NodeRef::External(external_node_ref) => Ok(Some(ExternalNodeRef {
                    use_file: external_node_ref.use_file,
                    ancestor_level: external_node_ref.ancestor_level,
                    path: Arc::clone(&external_node_ref.path),
                    phantom: PhantomData,
                })),
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
