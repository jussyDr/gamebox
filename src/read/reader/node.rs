use std::{any::Any, io::Read, sync::Arc};

use crate::read::{
    file::{read_body_chunks, read_body_chunks_inline},
    readable::{BodyChunks, BodyChunksInline},
    Error,
};

use super::{IdStateMut, Reader};

/// Node state.
pub struct NodeState {
    nodes: Box<[Option<Arc<dyn Any + Send + Sync>>]>,
}

impl NodeState {
    /// Create a new [NodeState] with the specified number of nodes.
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: vec![None; num_nodes].into_boxed_slice(),
        }
    }
}

/// Allows to get a reference to a [NodeState].
pub trait NodeStateRef {
    /// Get a reference to a [NodeState].
    fn get(&self) -> &NodeState;
}

impl NodeStateRef for NodeState {
    fn get(&self) -> &NodeState {
        self
    }
}

impl<T: NodeStateRef> NodeStateRef for &mut T {
    fn get(&self) -> &NodeState {
        (**self).get()
    }
}

/// Allows to get a mutable reference to a [NodeState].
pub trait NodeStateMut: NodeStateRef {
    /// Get a mutable reference to a [NodeState].
    fn get_mut(&mut self) -> &mut NodeState;
}

impl NodeStateMut for NodeState {
    fn get_mut(&mut self) -> &mut NodeState {
        self
    }
}

impl<T: NodeStateMut> NodeStateMut for &mut T {
    fn get_mut(&mut self) -> &mut NodeState {
        (**self).get_mut()
    }
}

impl<R: Read, I: IdStateMut, N> Reader<R, I, N> {
    /// TODO.
    pub fn node_inline<T: Default + BodyChunksInline>(&mut self) -> Result<Option<T>, Error> {
        let class_id = self.u32()?;

        if class_id == 0xffffffff {
            return Ok(None);
        }

        let mut node = T::default();

        read_body_chunks_inline(&mut node, self)?;

        Ok(Some(node))
    }

    /// TODO.
    pub fn node_inline_non_null<T: Default + BodyChunksInline>(&mut self) -> Result<T, Error> {
        match self.node_inline()? {
            None => Err(Error),
            Some(node) => Ok(node),
        }
    }

    /// TODO.
    pub fn node_inline_v2<T: Default + BodyChunksInline>(&mut self) -> Result<Option<T>, Error> {
        let mut node = T::default();

        read_body_chunks_inline(&mut node, self)?;

        Ok(Some(node))
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> Reader<R, I, N> {
    /// Read a node of type `T`.
    pub fn node<T: Default + Send + Sync + BodyChunks + 'static>(
        &mut self,
    ) -> Result<Option<Arc<T>>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index == 0 {
            return Err(Error);
        }

        let index = index - 1;

        let slot = self
            .node_state
            .get()
            .nodes
            .get(index as usize)
            .ok_or(Error)?;

        let node = match slot {
            None => {
                let class_id = self.u32()?;

                let mut node = T::default();

                read_body_chunks(&mut node, self)?;

                let node: Arc<dyn Any + Send + Sync> = Arc::new(node);

                let slot = self
                    .node_state
                    .get_mut()
                    .nodes
                    .get_mut(index as usize)
                    .expect("slot empty");

                *slot = Some(Arc::clone(&node));

                node.downcast().expect("failed to downcast")
            }
            Some(node) => Arc::clone(node).downcast().map_err(|_| Error)?,
        };

        Ok(Some(node))
    }

    /// Read a non null node of type `T`.
    pub fn node_non_null<T: Default + Send + Sync + BodyChunks + 'static>(
        &mut self,
    ) -> Result<Arc<T>, Error> {
        match self.node()? {
            None => Err(Error),
            Some(node) => Ok(node),
        }
    }
}
