use std::{io::Read, sync::Arc};

use crate::{
    Class, DynClass, ExternalNodeRef, NodeRef,
    read::{
        Error, ReadBody,
        reader::{IdTableRef, Reader},
    },
};

pub struct NodeTable {
    nodes: Vec<Option<NodeRef>>,
}

impl NodeTable {
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: vec![Option::None; num_nodes],
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
            .ok_or(Error("node index exceeds number of nodes".into()))?;

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

impl<R: Read, I: IdTableRef, N: NodeTableRef> Reader<R, I, N> {
    pub fn node_ref_generic_or_null(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn DynClass>, Error>,
    ) -> Result<Option<NodeRef>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or(Error("node index is zero".into()))?;

        let slot = self
            .node_state
            .as_mut()
            .nodes
            .get_mut(index as usize)
            .ok_or(Error("node index exceeds number of nodes".into()))?;

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

                Ok(Some(NodeRef::Internal(node)))
            }
            Some(node_ref) => Ok(Some(NodeRef::clone(node_ref))),
        }
    }

    pub fn internal_node_ref_generic_or_null(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn DynClass>, Error>,
    ) -> Result<Option<Arc<dyn DynClass>>, Error> {
        let node_ref = self.node_ref_generic_or_null(read_fn)?;

        match node_ref {
            None => Ok(None),
            Some(NodeRef::Internal(node)) => Ok(Some(node)),
            Some(NodeRef::External(_)) => Err(Error("expected an internal node reference".into())),
        }
    }

    pub fn node_ref_generic(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn DynClass>, Error>,
    ) -> Result<NodeRef, Error> {
        let node_ref = self.node_ref_generic_or_null(read_fn)?;

        match node_ref {
            None => Err(Error("node reference is null".into())),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn internal_node_ref_generic(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn DynClass>, Error>,
    ) -> Result<Arc<dyn DynClass>, Error> {
        let node_ref = self.node_ref_generic(read_fn)?;

        match node_ref {
            NodeRef::Internal(node) => Ok(node),
            NodeRef::External(_) => Err(Error("expected an internal node reference".into())),
        }
    }

    pub fn node_ref_or_null<T: Default + Class + ReadBody + 'static>(
        &mut self,
    ) -> Result<Option<NodeRef>, Error> {
        let node_ref = self.node_ref_generic_or_null(|r, class_id| {
            let mut node = T::default();

            if class_id != node.class_id() {
                todo!()
            }

            node.read_body(r)?;

            Ok(Arc::new(node))
        })?;

        match node_ref {
            None => Ok(None),
            Some(node_ref) => Ok(Some(node_ref)),
        }
    }

    pub fn node_ref<T: Default + Class + ReadBody + 'static>(&mut self) -> Result<NodeRef, Error> {
        let node_ref = self.node_ref_or_null::<T>()?;

        match node_ref {
            None => Err(Error("node reference is null".into())),
            Some(node_ref) => Ok(node_ref),
        }
    }

    pub fn internal_node_ref_or_null<T: Default + Class + ReadBody + 'static>(
        &mut self,
    ) -> Result<Option<Arc<T>>, Error> {
        let node = self.internal_node_ref_generic_or_null(|r, class_id| {
            let mut node = T::default();

            if class_id != node.class_id() {
                todo!()
            }

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

    pub fn internal_node_ref<T: Default + Class + ReadBody + 'static>(
        &mut self,
    ) -> Result<Arc<T>, Error> {
        let node = self.internal_node_ref_or_null::<T>()?;

        match node {
            None => Err(Error("node reference is null".into())),
            Some(node) => Ok(node),
        }
    }

    pub fn external_node_ref_or_null(&mut self) -> Result<Option<ExternalNodeRef>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or(Error("node index is zero".into()))?;

        let slot = self
            .node_state
            .as_mut()
            .nodes
            .get(index as usize)
            .ok_or(Error("node index exceeds number of nodes".into()))?;

        match slot {
            Some(NodeRef::External(node_ref)) => Ok(Some(node_ref.clone())),
            _ => todo!(),
        }
    }

    pub fn external_node_ref(&mut self) -> Result<ExternalNodeRef, Error> {
        let index = self
            .u32()?
            .checked_sub(1)
            .ok_or(Error("node index is zero".into()))?;

        let slot = self
            .node_state
            .as_mut()
            .nodes
            .get(index as usize)
            .ok_or(Error("node index exceeds number of nodes".into()))?;

        match slot {
            None => {
                todo!()
            }
            Some(NodeRef::Internal(_)) => {
                todo!()
            }
            Some(NodeRef::External(external_node_ref)) => Ok(external_node_ref.clone()),
        }
    }

    pub fn node<T: Default + Class + ReadBody>(&mut self) -> Result<T, Error> {
        let node = self.node_generic(|r, class_id| {
            let mut node = T::default();

            if class_id != node.class_id() {
                todo!("{:08X?}", class_id);
            }

            node.read_body(r)?;

            Ok(node)
        })?;

        Ok(node)
    }
}
