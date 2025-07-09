use std::{any::Any, marker::PhantomData};

use once_cell::unsync::OnceCell;

use crate::{ExternalNodeRef, NodeRef, read::Error};

/// Node table.
pub struct NodeTable {
    /// Nodes.
    pub nodes: Vec<OnceCell<NodeRef<dyn Any + Send + Sync>>>,
}

impl NodeTable {
    /// Create a new node table.
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: vec![OnceCell::new(); num_nodes],
        }
    }

    /// Set external.
    pub fn set_external<T>(
        &mut self,
        index: u32,
        external_node_ref: ExternalNodeRef<T>,
    ) -> Result<(), Error> {
        let slot = self
            .nodes
            .get_mut(index as usize)
            .ok_or_else(|| Error::new("node index exceeds number of nodes"))?;

        slot.set(NodeRef::External(ExternalNodeRef {
            path: external_node_ref.path,
            ancestor_level: external_node_ref.ancestor_level,
            marker: PhantomData,
        }))
        .map_err(|_| Error::new(""))?;

        Ok(())
    }
}

impl AsMut<NodeTable> for NodeTable {
    fn as_mut(&mut self) -> &mut NodeTable {
        self
    }
}
