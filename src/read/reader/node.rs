use std::{any::Any, sync::Arc};

use crate::{
    ExternalNodeRef, NodeRef,
    read::{Error, reader::repeat_n_with},
};

/// Node table.
pub struct NodeTable {
    /// Nodes.
    pub nodes: Vec<Option<NodeRef<Arc<dyn Any + Send + Sync>>>>,
}

impl NodeTable {
    /// Create a new node table.
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: repeat_n_with(num_nodes, || None),
        }
    }

    /// Set external.
    pub fn set_external(
        &mut self,
        index: u32,
        external_node_ref: ExternalNodeRef,
    ) -> Result<(), Error> {
        let slot = self
            .nodes
            .get_mut(index as usize)
            .ok_or_else(|| Error::new("node index exceeds number of nodes"))?;

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

pub trait Downcast: Sized {
    fn downcast(value: Arc<dyn Any + Send + Sync>) -> Option<Self>;
}

impl<T: 'static + Send + Sync> Downcast for Arc<T> {
    fn downcast(value: Arc<dyn Any + Send + Sync>) -> Option<Self> {
        value.downcast().ok()
    }
}
