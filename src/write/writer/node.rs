use std::{
    any::Any,
    hash::{Hash, Hasher},
    io::Write,
    sync::Arc,
};

use indexmap::{indexset, IndexSet};

use crate::{
    write::{writable::WriteBody, Error},
    Class,
};

use super::{IdStateMut, Writer};

trait DynAnyEqHash: Any {
    fn dyn_eq(&self, other: &dyn Any) -> bool;

    fn dyn_hash(&self, state: &mut dyn Hasher);
}

impl<T: 'static + Eq + Hash> DynAnyEqHash for T {
    fn dyn_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref() == Some(self)
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        self.hash(&mut state);
    }
}

struct InternalNode {
    node: Arc<dyn DynAnyEqHash>,
}

impl PartialEq for InternalNode {
    fn eq(&self, other: &Self) -> bool {
        self.node.dyn_eq(other)
    }
}

impl Eq for InternalNode {}

impl Hash for InternalNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.dyn_hash(state);
    }
}

/// Node state.
pub struct NodeState {
    nodes: IndexSet<InternalNode>,
}

impl NodeState {
    pub fn new() -> Self {
        Self { nodes: indexset![] }
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self::new()
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

impl<T: NodeStateMut> NodeStateMut for &mut T {
    fn get_mut(&mut self) -> &mut NodeState {
        (**self).get_mut()
    }
}

impl<W: Write, I: IdStateMut, N: NodeStateMut> Writer<W, I, N> {
    pub fn node_or_null<T: Class + WriteBody>(&mut self, value: Option<&T>) -> Result<(), Error> {
        if let Some(value) = value {
            self.u32(T::CLASS_ID)?;
            value.write_body(self)?;
        } else {
            self.u32(0xffffffff)?;
        }

        Ok(())
    }

    pub fn node<T: Class + WriteBody>(&mut self, value: &T) -> Result<(), Error> {
        self.node_or_null(Some(value))
    }

    pub fn internal_node_ref_or_null<T: 'static + Class + WriteBody + Eq + Hash>(
        &mut self,
        node: Option<&Arc<T>>,
    ) -> Result<(), Error> {
        match node {
            Some(node) => {
                let internal_node = InternalNode {
                    node: Arc::clone(node) as Arc<dyn DynAnyEqHash>,
                };

                match self.node_state.get_mut().nodes.get_index_of(&internal_node) {
                    Some(index) => {
                        self.u32(index as u32 + 1)?;
                    }
                    None => {
                        self.node_state.get_mut().nodes.insert(internal_node);

                        let index = self.node_state.get_mut().nodes.len() as u32;

                        self.u32(index)?;
                        self.node(node.as_ref())?;
                    }
                }
            }
            None => {
                self.u32(0xffffffff)?;
            }
        }

        Ok(())
    }

    pub fn internal_node_ref<T: 'static + Class + WriteBody + Eq + Hash>(
        &mut self,
        node: &Arc<T>,
    ) -> Result<(), Error> {
        self.internal_node_ref_or_null(Some(node))
    }
}
