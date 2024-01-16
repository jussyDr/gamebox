use std::{any::Any, io::Read, path::Path, rc::Rc};

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

/// Node reference state.
pub struct NodeState {
    #[allow(clippy::type_complexity)]
    nodes: Box<[Option<NodeRef<Rc<dyn Any>>>]>,
}

impl NodeState {
    /// Create a new node reference state.
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: repeat_n_with(num_nodes, || None),
        }
    }

    /// Get a node reference with the given `index`.
    pub fn get(&self, index: usize) -> Result<Option<&NodeRef<Rc<dyn Any>>>> {
        let node_ref = self.nodes.get(index - 1).ok_or("node index out of range")?;

        Ok(node_ref.as_ref())
    }

    /// Set a node reference at the given `index`.
    pub fn set(&mut self, index: usize, node_ref: NodeRef<Rc<dyn Any>>) -> Result<()> {
        let entry = self
            .nodes
            .get_mut(index - 1)
            .ok_or("node index out of range")?;

        if entry.is_some() {
            return Err("node already read".into());
        }

        *entry = Some(node_ref);

        Ok(())
    }
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
            NULL => return Err("node index is null".into()),
            index => index,
        };

        let node_ref = self
            .node_state
            .borrow()
            .get(index as usize)?
            .ok_or("node is null")?;

        match node_ref {
            NodeRef::Internal { .. } => Err("expected external node ref".into()),
            NodeRef::External { path } => Ok(Rc::clone(path)),
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
            NULL => return Ok(None),
            index => index,
        };

        match self.node_state.borrow().get(index as usize)? {
            None => {
                let class_id = self.u32()?;

                if class_id != T::CLASS_ID.get() {
                    return Err("class id does not match".into());
                }

                let mut node = T::default();

                T::read_body(&mut node, self)?;

                let node = Rc::new(node);

                self.node_state.borrow_mut().set(
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
            0xffffffff => return Ok(None),
            index => index,
        };

        match self.node_state.borrow().get(index as usize)? {
            None => {
                let class_id = self.u32()?;

                let node = read_fn(self, class_id)?;

                self.node_state.borrow_mut().set(
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
            NULL => return Ok(None),
            index => index,
        };

        match self.node_state.borrow().get(index as usize)? {
            None => {
                let class_id = self.u32()?;

                if class_id != T::CLASS_ID.get() {
                    return Err("class id does not match".into());
                }

                self.node_state
                    .borrow_mut()
                    .set(index as usize, NodeRef::Internal { node: Rc::new(()) })?;

                let mut node = T::default();

                T::read_body(&mut node, self)?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::Internal { .. }) => Err("".into()),
            Some(NodeRef::External { path }) => Ok(Some(NodeRef::External {
                path: Rc::clone(path),
            })),
        }
    }

    /// Read an unique node reference that may be internal or external and that may be null.
    pub fn any_unique_node_ref_or_null(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Box<dyn Any>>,
    ) -> Result<Option<NodeRef<Box<dyn Any>>>> {
        let index = match self.u32()? {
            0xffffffff => return Ok(None),
            index => index,
        };

        match self.node_state.borrow().get(index as usize)? {
            None => {
                let class_id = self.u32()?;

                self.node_state
                    .borrow_mut()
                    .set(index as usize, NodeRef::Internal { node: Rc::new(()) })?;

                let node = read_fn(self, class_id)?;

                Ok(Some(NodeRef::Internal { node }))
            }
            Some(NodeRef::Internal { .. }) => Err("".into()),
            Some(NodeRef::External { path }) => Ok(Some(NodeRef::External {
                path: Rc::clone(path),
            })),
        }
    }
}
