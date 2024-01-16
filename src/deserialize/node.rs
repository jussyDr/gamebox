use std::{any::Any, io::Read, mem::replace, path::Path, rc::Rc};

use crate::{
    common::{Class, NULL},
    read::{readable::ReadBody, Result},
};

use super::{repeat_n_with, Deserializer};

/// Reference to a node of type `T`.
pub enum NodeRef<T> {
    /// Internal node reference.
    Internal {
        /// The referenced node.
        node: T,
    },
    /// External node reference.
    External {
        /// Game path to the referenced node.
        path: Rc<Path>,
    },
}

enum NodeStateEntry {
    NodeRef(NodeRef<Rc<dyn Any>>),
    Unique,
}

/// Node reference state.
pub struct NodeState {
    nodes: Box<[Option<NodeStateEntry>]>,
}

impl NodeState {
    /// Create a new node reference state.
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: repeat_n_with(num_nodes, || None),
        }
    }

    /// Get the node reference at the given `index`.
    pub fn get_node_ref(&self, index: usize) -> Result<Option<&NodeRef<Rc<dyn Any>>>> {
        let entry = self.nodes.get(index).ok_or("node index out of range")?;

        match entry {
            None => Ok(None),
            Some(NodeStateEntry::NodeRef(node_ref)) => Ok(Some(node_ref)),
            Some(NodeStateEntry::Unique) => Err("".into()),
        }
    }

    /// Set a node reference at the given `index`.
    pub fn set_node_ref(&mut self, index: usize, node_ref: NodeRef<Rc<dyn Any>>) -> Result<()> {
        set_node_state_entry(self, index, NodeStateEntry::NodeRef(node_ref))
    }
}

fn set_node_state_entry(
    node_state: &mut NodeState,
    index: usize,
    entry: NodeStateEntry,
) -> Result<()> {
    let slot = node_state
        .nodes
        .get_mut(index)
        .ok_or("node index out of range")?;

    if slot.is_some() {
        return Err("node already read".into());
    }

    *slot = Some(entry);

    Ok(())
}

/// Can obtain a mutable reference to a node reference state.
pub trait NodeStateMut {
    /// Obtain an immutable reference to a node reference state.
    fn borrow(&self) -> &NodeState;

    /// Obtain a mutable reference to a node reference state.
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

impl<R: Read, I, N: NodeStateMut> Deserializer<R, I, N> {
    /// Read an internal node reference that is not null.
    pub fn internal_node_ref<T: 'static + Default + Class + ReadBody<R, I, N>>(
        &mut self,
    ) -> Result<Rc<T>> {
        match self.internal_node_ref_or_null()? {
            None => Err("node is null".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read an internal node reference that may be null.
    pub fn internal_node_ref_or_null<T: 'static + Default + Class + ReadBody<R, I, N>>(
        &mut self,
    ) -> Result<Option<Rc<T>>> {
        match self.node_ref_or_null()? {
            None => Ok(None),
            Some(NodeRef::Internal { node }) => Ok(Some(node)),
            Some(NodeRef::External { .. }) => Err("expected internal node ref".into()),
        }
    }

    /// Read an external node reference that is not null.
    pub fn external_node_ref(&mut self) -> Result<Rc<Path>> {
        let index = match self.u32()? {
            0 => return Err("".into()),
            NULL => return Err("node index is null".into()),
            index => index - 1,
        };

        let entry = self
            .node_state
            .borrow()
            .get_node_ref(index as usize)?
            .ok_or("node is null")?;

        match entry {
            NodeRef::External { path } => Ok(Rc::clone(path)),
            _ => Err("expected external node ref".into()),
        }
    }

    /// Read a node reference that may be internal or external and that is not null.
    pub fn node_ref<T: 'static + Default + Class + ReadBody<R, I, N>>(
        &mut self,
    ) -> Result<NodeRef<Rc<T>>> {
        match self.node_ref_or_null()? {
            None => Err("node is null".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read a node reference that may be internal or external and that may be null.
    pub fn node_ref_or_null<T: 'static + Default + Class + ReadBody<R, I, N>>(
        &mut self,
    ) -> Result<Option<NodeRef<Rc<T>>>> {
        let index = match self.u32()? {
            0 => return Err("".into()),
            NULL => return Ok(None),
            index => index as usize - 1,
        };

        match self.node_state.borrow().get_node_ref(index)? {
            None => {
                let class_id = self.u32()?;

                if class_id != T::CLASS_ID.get() {
                    return Err("class id does not match".into());
                }

                let mut node = T::default();

                T::read_body(&mut node, self)?;

                let node = Rc::new(node);

                self.node_state.borrow_mut().set_node_ref(
                    index,
                    NodeRef::Internal {
                        node: Rc::<T>::clone(&node),
                    },
                )?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::Internal { node }) => {
                let node: Rc<T> = Rc::clone(node).downcast().map_err(|_| "wrong node type")?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::External { path }) => Ok(Some(NodeRef::External {
                path: Rc::clone(path),
            })),
        }
    }

    /// Read an internal node reference that is not null.
    pub fn any_internal_node_ref(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Rc<dyn Any>>,
    ) -> Result<Rc<dyn Any>> {
        match self.any_node_ref_or_null(read_fn)? {
            None => Err("node is null".into()),
            Some(NodeRef::Internal { node }) => Ok(node),
            Some(NodeRef::External { .. }) => Err("expected internal node ref".into()),
        }
    }

    /// Read a node reference that may be internal or external and that may be null.
    pub fn any_node_ref_or_null(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Rc<dyn Any>>,
    ) -> Result<Option<NodeRef<Rc<dyn Any>>>> {
        let index = match self.u32()? {
            0 => return Err("".into()),
            NULL => return Ok(None),
            index => index as usize - 1,
        };

        match self.node_state.borrow().get_node_ref(index)? {
            None => {
                let class_id = self.u32()?;

                let node = read_fn(self, class_id)?;

                self.node_state.borrow_mut().set_node_ref(
                    index,
                    NodeRef::Internal {
                        node: Rc::clone(&node),
                    },
                )?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::Internal { node }) => Ok(Some(NodeRef::Internal {
                node: Rc::clone(node),
            })),
            Some(NodeRef::External { path }) => Ok(Some(NodeRef::External {
                path: Rc::clone(path),
            })),
        }
    }

    /// Read an unique internal node reference that is not null.
    pub fn unique_internal_node_ref<T: 'static + Default + Class + ReadBody<R, I, N>>(
        &mut self,
    ) -> Result<T> {
        match self.unique_internal_node_ref_or_null()? {
            None => Err("".into()),
            Some(node) => Ok(node),
        }
    }

    /// Read an unique internal node reference that may be null.
    pub fn unique_internal_node_ref_or_null<T: 'static + Default + Class + ReadBody<R, I, N>>(
        &mut self,
    ) -> Result<Option<T>> {
        match self.unique_node_ref_or_null()? {
            None => Ok(None),
            Some(NodeRef::Internal { node }) => Ok(Some(node)),
            Some(NodeRef::External { .. }) => Err("".into()),
        }
    }

    /// Read an unique internal node reference.
    pub fn unique_node_ref<T: 'static + Default + Class + ReadBody<R, I, N>>(
        &mut self,
    ) -> Result<NodeRef<T>> {
        match self.unique_node_ref_or_null()? {
            None => Err("".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read an unique node reference that may be internal or external and that may be null.
    pub fn unique_node_ref_or_null<T: 'static + Default + Class + ReadBody<R, I, N>>(
        &mut self,
    ) -> Result<Option<NodeRef<T>>> {
        let index = match self.u32()? {
            0 => return Err("".into()),
            NULL => return Ok(None),
            index => index as usize - 1,
        };

        match self.node_state.borrow().get_node_ref(index)? {
            None => {
                let class_id = self.u32()?;

                if class_id != T::CLASS_ID.get() {
                    return Err("class id does not match".into());
                }

                set_node_state_entry(self.node_state.borrow_mut(), index, NodeStateEntry::Unique)?;

                let mut node = T::default();

                T::read_body(&mut node, self)?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::Internal { .. }) => Err("".into()),
            Some(NodeRef::External { .. }) => {
                let old = replace(
                    self.node_state.borrow_mut().nodes[index]
                        .as_mut()
                        .ok_or("")?,
                    NodeStateEntry::Unique,
                );

                match old {
                    NodeStateEntry::NodeRef(NodeRef::External { path }) => {
                        Ok(Some(NodeRef::External { path }))
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    /// Read an unique node reference that may be internal or external and that may be null.
    pub fn any_unique_node_ref_or_null<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T>,
    ) -> Result<Option<NodeRef<T>>> {
        let index = match self.u32()? {
            0 => return Err("".into()),
            NULL => return Ok(None),
            index => index as usize - 1,
        };

        match self.node_state.borrow().get_node_ref(index)? {
            None => {
                let class_id = self.u32()?;

                set_node_state_entry(self.node_state.borrow_mut(), index, NodeStateEntry::Unique)?;

                let node = read_fn(self, class_id)?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::Internal { .. }) => Err("".into()),
            Some(NodeRef::External { .. }) => {
                let old = replace(
                    self.node_state.borrow_mut().nodes[index]
                        .as_mut()
                        .ok_or("")?,
                    NodeStateEntry::Unique,
                );

                match old {
                    NodeStateEntry::NodeRef(NodeRef::External { path }) => {
                        Ok(Some(NodeRef::External { path }))
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
