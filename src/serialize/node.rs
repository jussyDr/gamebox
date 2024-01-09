use std::{
    any::{Any, TypeId},
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    io::Write,
};

use crate::{
    common::{Class, NODE_END},
    write::{writable::WriteBody, Error, Result},
};

use super::{IdStateMut, Serializer};

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
        match other.as_any().downcast_ref::<T>() {
            None => false,
            Some(other) => self == other,
        }
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
    num_nodes: u32,
    nodes: HashMap<Box<dyn CachableNode>, u32>,
}

impl NodeState {
    /// Create a new node state.
    pub fn new() -> Self {
        Self {
            num_nodes: 0,
            nodes: HashMap::new(),
        }
    }

    /// The number of nodes written.
    pub fn num_nodes(&self) -> u32 {
        self.num_nodes + 1
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Can obtain a mutable reference to a node state.
pub trait NodeStateMut {
    /// Obtain an immutable reference to a node state.
    fn borrow(&self) -> &NodeState;

    /// Obtain a mutable reference to a node state.
    fn borrow_mut(&mut self) -> &mut NodeState;
}

impl NodeStateMut for NodeState {
    fn borrow(&self) -> &NodeState {
        self
    }

    fn borrow_mut(&mut self) -> &mut NodeState {
        self
    }
}

impl<T: NodeStateMut> NodeStateMut for &mut T {
    fn borrow(&self) -> &NodeState {
        (**self).borrow()
    }
    fn borrow_mut(&mut self) -> &mut NodeState {
        (**self).borrow_mut()
    }
}

impl<W: Write, I: IdStateMut, N: NodeStateMut> Serializer<W, I, N> {
    /// Write a cachable node reference.
    pub fn node_ref<T: 'static + Eq + Hash + Class + WriteBody>(&mut self, node: T) -> Result {
        match self
            .node_state
            .borrow()
            .nodes
            .get(&node as &dyn CachableNode)
        {
            None => {
                let index = write_node_ref(self, &node)?;

                self.node_state
                    .borrow_mut()
                    .nodes
                    .insert(Box::new(node), index);
            }
            Some(&index) => self.u32(index)?,
        }

        Ok(())
    }

    /// Write an unique non-cached node reference.
    pub fn unique_node_ref<T: Class + WriteBody>(&mut self, node: &T) -> Result {
        write_node_ref(self, node)?;

        Ok(())
    }
}

fn write_node_ref<W: Write, I: IdStateMut, N: NodeStateMut, T: Class + WriteBody>(
    s: &mut Serializer<W, I, N>,
    node: &T,
) -> std::result::Result<u32, Error> {
    let index = s.node_state.borrow().num_nodes + 1;

    s.u32(index)?;

    s.node_state.borrow_mut().num_nodes = index;

    s.u32(T::CLASS_ID.get())?;

    node.write_body(s)?;

    s.u32(NODE_END)?;

    Ok(index)
}
