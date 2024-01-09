use std::{
    any::Any,
    cell::OnceCell,
    io::{Read, Seek},
    path::Path,
    rc::Rc,
};

use crate::{
    common::Class,
    read::{readable::ReadBody, Result},
};

use super::{repeat_n_with, Deserializer, IdStateMut};

/// Reference to a node of type `T`.
pub enum NodeRef<T: ?Sized> {
    /// Internal node reference.
    Internal {
        /// The referenced node.
        node: Rc<T>,
    },
    /// External node reference.
    External {
        /// Path to the referenced node.
        path: Rc<Path>,
    },
}

/// Node reference state.
pub struct NodeState {
    nodes: Box<[OnceCell<NodeRef<dyn Any>>]>,
}

impl NodeState {
    /// Create a new node state.
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: repeat_n_with(num_nodes, OnceCell::new),
        }
    }

    /// Get a node reference with the given `index`.
    pub fn get(&self, index: usize) -> Result<&OnceCell<NodeRef<dyn Any>>> {
        self.nodes
            .get(index - 1)
            .ok_or("node index out of range".into())
    }

    /// Set a node reference at the given `index`.
    pub fn set(&self, index: usize, node_ref: NodeRef<dyn Any>) -> Result<()> {
        self.get(index)?
            .set(node_ref)
            .map_err(|_| "node already read".into())
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

impl<R: Read, I, N: NodeStateMut> Deserializer<R, I, N> {
    /// Read an external node reference that is not null.
    pub fn external_node_ref(&mut self) -> Result<Rc<Path>> {
        let index = match self.u32()? {
            0xffffffff => return Err("node index is null".into()),
            index => index,
        };

        let node_ref = self
            .node_state
            .borrow()
            .get(index as usize)?
            .get()
            .ok_or("node is null")?;

        match node_ref {
            NodeRef::Internal { .. } => Err("expected external node ref".into()),
            NodeRef::External { path } => Ok(Rc::clone(path)),
        }
    }
}

impl<R: Read + Seek, I: IdStateMut, N: NodeStateMut> Deserializer<R, I, N> {
    /// Read an internal node reference that is not null.
    pub fn internal_node_ref<T: 'static + Default + Class + ReadBody>(&mut self) -> Result<Rc<T>> {
        match self.internal_node_ref_or_null()? {
            None => Err("node is null".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read an internal node reference that may be null.
    pub fn internal_node_ref_or_null<T: 'static + Default + Class + ReadBody>(
        &mut self,
    ) -> Result<Option<Rc<T>>> {
        match self.node_ref_or_null()? {
            None => Ok(None),
            Some(NodeRef::Internal { node }) => Ok(Some(node)),
            Some(NodeRef::External { .. }) => Err("expected internal node ref".into()),
        }
    }

    /// Read a node reference that may be internal or external and that is not null.
    pub fn node_ref<T: 'static + Default + Class + ReadBody>(&mut self) -> Result<NodeRef<T>> {
        match self.node_ref_or_null()? {
            None => Err("node is null".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read a node reference that may be internal or external and that may be null.
    pub fn node_ref_or_null<T: 'static + Default + Class + ReadBody>(
        &mut self,
    ) -> Result<Option<NodeRef<T>>> {
        let index = match self.u32()? {
            0xffffffff => return Ok(None),
            index => index,
        };

        match self.node_state.borrow().get(index as usize)?.get() {
            None => {
                let class_id = self.u32()?;

                if class_id != T::CLASS_ID.get() {
                    return Err("class id does not match".into());
                }

                let mut node = T::default();

                T::read_body(&mut node, self)?;

                let node = Rc::new(node);

                self.node_state.borrow().set(
                    index as usize,
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

    /// Read a node that is not null.
    pub fn node<T: Default + Class + ReadBody>(&mut self) -> Result<T> {
        match self.node_or_null()? {
            None => Err("node is null".into()),
            Some(node_ref) => Ok(node_ref),
        }
    }

    /// Read a node that may be null.
    pub fn node_or_null<T: Default + Class + ReadBody>(&mut self) -> Result<Option<T>> {
        let class_id = self.u32()?;

        if class_id == 0xffffffff {
            return Ok(None);
        }

        if class_id != T::CLASS_ID.get() {
            return Err("class id does not match".into());
        }

        let mut node = T::default();

        T::read_body(&mut node, self)?;

        Ok(Some(node))
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
    ) -> Result<Option<NodeRef<dyn Any>>> {
        let index = match self.u32()? {
            0xffffffff => return Ok(None),
            index => index,
        };

        match self.node_state.borrow().get(index as usize)?.get() {
            None => {
                let class_id = self.u32()?;

                let node = read_fn(self, class_id)?;

                self.node_state.borrow().set(
                    index as usize,
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
}
