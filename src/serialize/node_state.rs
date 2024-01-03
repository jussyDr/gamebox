use std::{
    any::{Any, TypeId},
    cell::Cell,
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
};

use elsa::FrozenMap;

use crate::{
    common::{ClassId, NODE_END},
    write::{writable::WriteBody, Result},
};

use super::{IdStateRef, Serializer};

trait CachableNode {
    fn eq(&self, other: &dyn CachableNode) -> bool;

    fn hash(&self) -> u64;

    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for dyn CachableNode {
    fn eq(&self, other: &Self) -> bool {
        CachableNode::eq(self, other)
    }
}

impl Eq for dyn CachableNode {}

impl Hash for dyn CachableNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(CachableNode::hash(self));
    }
}

impl<T: 'static + Eq + Hash> CachableNode for T {
    fn eq(&self, other: &dyn CachableNode) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            return self == other;
        }

        false
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        Hash::hash(&(TypeId::of::<T>(), self), &mut hasher);
        hasher.finish()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Node state.
pub struct NodeState {
    num_nodes: Cell<u32>,
    nodes: FrozenMap<Box<dyn CachableNode>, u32>,
}

impl NodeState {
    /// Create a new node state.
    pub fn new() -> Self {
        Self {
            num_nodes: Cell::new(0),
            nodes: FrozenMap::new(),
        }
    }

    /// The number of nodes written.
    pub fn num_nodes(&self) -> u32 {
        self.num_nodes.get() + 1
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Can obtain a immutable reference to a node state.
pub trait NodeStateRef {
    /// Obtain a immutable reference to a node state.
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

impl<W: Write, I: IdStateRef, N: NodeStateRef> Serializer<W, I, N> {
    /// Write a cachable node reference.
    pub fn node_ref<T: 'static + Eq + Hash + ClassId + WriteBody>(&mut self, node: T) -> Result {
        match self
            .node_state
            .borrow()
            .nodes
            .get_copy(&node as &dyn CachableNode)
        {
            None => {
                let index = self.node_state.borrow().num_nodes.get() + 1;

                self.u32(index)?;

                self.node_state.borrow().num_nodes.set(index);

                self.u32(T::class_id())?;

                node.write_body(self)?;

                self.u32(NODE_END)?;

                self.node_state
                    .borrow()
                    .nodes
                    .insert_copy(Box::new(node), index);
            }
            Some(index) => self.u32(index)?,
        }

        Ok(())
    }

    /// Write an unique non-cached node reference.
    pub fn unique_node_ref<T: ClassId + WriteBody>(&mut self, node: &T) -> Result {
        let index = self.node_state.borrow().num_nodes.get() + 1;

        self.u32(index)?;

        self.node_state.borrow().num_nodes.set(index);

        self.u32(T::class_id())?;

        node.write_body(self)?;

        self.u32(NODE_END)?;

        Ok(())
    }
}
