use std::{any::Any, io::Read, sync::Arc};

use crate::{
    ClassId, ExternalNodeRef, NodeRef, SubExtensions,
    read::{
        Error, ReadBody,
        reader::{IdTableRef, Reader, repeat_n_with},
    },
    sub_extension,
};

/// Node table.
pub struct NodeTable {
    nodes: Vec<Option<NodeRef<Arc<dyn Any + Send + Sync>>>>,
}

impl NodeTable {
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: repeat_n_with(num_nodes, || None),
        }
    }

    pub fn set_external(
        &mut self,
        index: u32,
        external_node_ref: ExternalNodeRef,
    ) -> Result<(), Error> {
        let slot = self
            .nodes
            .get_mut(index as usize)
            .ok_or(Error::new("node index exceeds number of nodes"))?;

        if slot.is_some() {
            todo!()
        }

        *slot = Some(NodeRef::External(external_node_ref));

        Ok(())
    }
}

impl AsMut<NodeTable> for NodeTable {
    fn as_mut(&mut self) -> &mut NodeTable {
        self
    }
}

pub trait NodeTableRef: AsMut<NodeTable> {}

impl<T: AsMut<NodeTable>> NodeTableRef for T {}

impl<R: Read, I, N> Reader<R, I, N> {
    pub fn node_or_null_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<Option<T>, Error> {
        let class_id = self.u32()?;

        if class_id == 0xffffffff {
            return Ok(None);
        }

        let node = read_fn(self, class_id)?;

        Ok(Some(node))
    }

    pub fn node_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let node = self.node_or_null_generic(read_fn)?;

        match node {
            None => Err(Error::new("node is null")),
            Some(node) => Ok(node),
        }
    }
}

impl<R: Read, I, N: NodeTableRef> Reader<R, I, N> {
    pub fn external_node_ref_or_null<T: SubExtensions>(
        &mut self,
    ) -> Result<Option<ExternalNodeRef>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or(Error::new("node index is zero"))?;

        let slot = self
            .node_state
            .as_mut()
            .nodes
            .get(index as usize)
            .ok_or(Error::new("node index exceeds number of nodes"))?;

        match slot {
            Some(NodeRef::External(node_ref)) => {
                let sub_extension = sub_extension(&node_ref.path).unwrap();

                if !T::has_sub_extension(sub_extension) {
                    todo!("{}", sub_extension);
                }

                Ok(Some(node_ref.clone()))
            }
            _ => todo!(),
        }
    }

    pub fn external_node_ref<T: SubExtensions>(&mut self) -> Result<ExternalNodeRef, Error> {
        match self.external_node_ref_or_null::<T>()? {
            None => todo!(),
            Some(node_ref) => Ok(node_ref),
        }
    }
}

impl<R: Read, I: IdTableRef, N: NodeTableRef> Reader<R, I, N> {
    pub fn node_ref_generic_or_null<T: Clone + Downcast>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Any + Send + Sync>, Error>,
    ) -> Result<Option<NodeRef<T>>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or(Error::new("node index is zero"))?;

        let slot = self
            .node_state
            .as_mut()
            .nodes
            .get_mut(index as usize)
            .ok_or(Error::new("node index exceeds number of nodes"))?;

        match slot {
            None => {
                let node = self.node_generic(read_fn)?;

                let slot = self
                    .node_state
                    .as_mut()
                    .nodes
                    .get_mut(index as usize)
                    .unwrap();

                *slot = Some(NodeRef::Internal(Arc::clone(&node)));

                Ok(Some(NodeRef::Internal(T::downcast(node).unwrap())))
            }
            Some(node_ref) => match node_ref {
                NodeRef::Internal(x) => {
                    Ok(Some(NodeRef::Internal(T::downcast(x.clone()).unwrap())))
                }
                NodeRef::External(x) => Ok(Some(NodeRef::External(x.clone()))),
            },
        }
    }

    pub fn internal_node_ref_generic_or_null<T: Clone + Downcast>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Any + Send + Sync>, Error>,
    ) -> Result<Option<T>, Error> {
        let node_ref = self.node_ref_generic_or_null(read_fn)?;

        match node_ref {
            None => Ok(None),
            Some(NodeRef::Internal(node)) => Ok(Some(node)),
            Some(NodeRef::External(_)) => Err(Error::new("expected an internal node reference")),
        }
    }

    pub fn node_ref_generic<T: Clone + Downcast>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Any + Send + Sync>, Error>,
    ) -> Result<NodeRef<T>, Error> {
        let node_ref = self.node_ref_generic_or_null(read_fn)?;

        match node_ref {
            None => Err(Error::new("node reference is null")),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn internal_node_ref_generic<T: Clone + Downcast>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Any + Send + Sync>, Error>,
    ) -> Result<T, Error> {
        let node_ref = self.node_ref_generic(read_fn)?;

        match node_ref {
            NodeRef::Internal(node) => Ok(node),
            NodeRef::External(_) => Err(Error::new("expected an internal node reference")),
        }
    }

    pub fn node_ref_or_null<T: Default + Send + Sync + ClassId + ReadBody + 'static>(
        &mut self,
    ) -> Result<Option<NodeRef<Arc<T>>>, Error> {
        let node_ref = self.node_ref_generic_or_null(|r, class_id| {
            if class_id != T::CLASS_ID {
                todo!()
            }

            let mut node = T::default();
            node.read_body(r)?;

            Ok(Arc::new(node))
        })?;

        match node_ref {
            None => Ok(None),
            Some(node_ref) => Ok(Some(node_ref)),
        }
    }

    pub fn node_ref<T: Default + Send + Sync + ClassId + ReadBody + 'static>(
        &mut self,
    ) -> Result<NodeRef<Arc<T>>, Error> {
        let node_ref = self.node_ref_or_null::<T>()?;

        match node_ref {
            None => Err(Error::new("node reference is null")),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn internal_node_ref_or_null<T: Default + Send + Sync + ClassId + ReadBody + 'static>(
        &mut self,
    ) -> Result<Option<Arc<T>>, Error> {
        let node: Option<Arc<T>> = self.internal_node_ref_generic_or_null(|r, class_id| {
            if class_id != T::CLASS_ID {
                todo!()
            }

            let mut node = T::default();

            node.read_body(r)?;

            Ok(Arc::new(node))
        })?;

        match node {
            None => Ok(None),
            Some(node) => {
                let ptr = Arc::into_raw(node);
                unsafe { Ok(Some(Arc::from_raw(ptr.cast()))) }
            }
        }
    }

    pub fn internal_node_ref<T: Default + Send + Sync + ClassId + ReadBody + 'static>(
        &mut self,
    ) -> Result<Arc<T>, Error> {
        let node = self.internal_node_ref_or_null::<T>()?;

        match node {
            None => Err(Error::new("node reference is null")),
            Some(node) => Ok(node),
        }
    }

    pub fn node<T: Default + ClassId + ReadBody>(&mut self) -> Result<T, Error> {
        let node = self.node_generic(|r, class_id| {
            if class_id != T::CLASS_ID {
                todo!("{:08X?}", class_id);
            }

            let mut node = T::default();
            node.read_body(r)?;

            Ok(node)
        })?;

        Ok(node)
    }
}

pub trait Downcast: Sized {
    fn downcast(value: Arc<dyn Any + Send + Sync>) -> Option<Self>;
}

impl<T: 'static + Send + Sync> Downcast for Arc<T> {
    fn downcast(value: Arc<dyn Any + Send + Sync>) -> Option<Self> {
        value.downcast().ok()
    }
}
