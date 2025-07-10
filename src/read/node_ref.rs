use std::{any::Any, cell::OnceCell, io::Read, marker::PhantomData, sync::Arc};

use crate::{
    ClassId, ExternalNodeRef, NULL, NodeRef,
    read::{BodyReader, BodyReaderImpl, Error, ReadBody, read_node_from_body, reader::Reader},
};

/// Node reference table.
pub struct NodeRefTable {
    node_refs: Vec<OnceCell<NodeRef<dyn Any + Send + Sync>>>,
}

impl NodeRefTable {
    /// Create a new `NodeRefTable`.
    pub fn new(num_nodes: usize) -> Self {
        Self {
            node_refs: vec![OnceCell::new(); num_nodes],
        }
    }

    /// Set external.
    pub fn set_external<T>(
        &self,
        index: u32,
        external_node_ref: ExternalNodeRef<T>,
    ) -> Result<(), Error> {
        let slot = self
            .node_refs
            .get(index as usize)
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

/// Trait implemented by types that are readable from a node reference.
pub trait ReadNodeRef {
    fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error>
    where
        Self: Sized;

    fn read_internal_node_ref(
        r: &mut impl BodyReader,
        class_id: u32,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error>
    where
        Self: Sized,
    {
        Err(Error::new("expected an external node reference"))
    }

    fn none() -> Result<Self, Error>
    where
        Self: Sized,
    {
        Err(Error::new("expected a non-null node reference"))
    }
}

pub fn read_node_ref<'n, T: ReadNodeRef, R: Read>(
    r: &mut BodyReaderImpl<'n, R>,
) -> Result<T, Error> {
    let index = r.u32()?;

    if index == NULL {
        return T::none();
    }

    let index = index
        .checked_sub(1)
        .ok_or_else(|| Error::new("node reference index is zero"))?;

    let slot = r
        .node_table
        .node_refs
        .get(index as usize)
        .ok_or_else(|| Error::new("node reference index exceeds number of nodes"))?;

    let node_ref = match slot.get() {
        None => {
            let class_id = r.u32()?;

            let node_ref = NodeRef::Internal(T::read_internal_node_ref(r, class_id)?);

            slot.set(NodeRef::clone(&node_ref))
                .map_err(|_| Error::new("reentrant node reference init"))?;

            node_ref
        }
        Some(node_ref) => NodeRef::clone(node_ref),
    };

    T::from_node_ref_any(node_ref)
}

impl<T: 'static + ClassId + Default + ReadBody + Send + Sync> ReadNodeRef for Arc<T> {
    fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error> {
        match node_ref {
            NodeRef::Internal(node_ref) => {
                let node_ref = node_ref
                    .downcast()
                    .map_err(|_| Error::new("node reference type does not match"))?;

                Ok(node_ref)
            }
            _ => todo!(),
        }
    }

    fn read_internal_node_ref(
        r: &mut impl BodyReader,
        class_id: u32,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error> {
        read_node_ref_internal_impl::<T>(r, class_id)
    }
}

impl<T: 'static + ClassId + Default + ReadBody + Send + Sync> ReadNodeRef for Option<Arc<T>> {
    fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error> {
        match node_ref {
            NodeRef::Internal(node_ref) => {
                let node_ref = node_ref
                    .downcast()
                    .map_err(|_| Error::new("node reference type does not match"))?;

                Ok(Some(node_ref))
            }
            _ => todo!(),
        }
    }

    fn read_internal_node_ref(
        r: &mut impl BodyReader,
        class_id: u32,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error> {
        read_node_ref_internal_impl::<T>(r, class_id)
    }

    fn none() -> Result<Self, Error> {
        Ok(None)
    }
}

impl<T> ReadNodeRef for ExternalNodeRef<T> {
    fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error> {
        match node_ref {
            NodeRef::External(node_ref) => Ok(ExternalNodeRef {
                path: Arc::clone(&node_ref.path),
                ancestor_level: node_ref.ancestor_level,
                marker: PhantomData,
            }),
            _ => todo!(),
        }
    }
}

impl<T> ReadNodeRef for Option<ExternalNodeRef<T>> {
    fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error> {
        match node_ref {
            NodeRef::External(node_ref) => Ok(Some(ExternalNodeRef {
                path: Arc::clone(&node_ref.path),
                ancestor_level: node_ref.ancestor_level,
                marker: PhantomData,
            })),
            _ => todo!(),
        }
    }

    fn none() -> Result<Self, Error> {
        Ok(None)
    }
}

impl<T: 'static + ClassId + Default + ReadBody + Send + Sync> ReadNodeRef for NodeRef<T> {
    fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error> {
        match node_ref {
            NodeRef::Internal(node_ref) => {
                let node_ref = node_ref
                    .downcast()
                    .map_err(|_| Error::new("node reference type does not match"))?;

                Ok(NodeRef::Internal(node_ref))
            }
            NodeRef::External(node_ref) => Ok(NodeRef::External(ExternalNodeRef {
                path: Arc::clone(&node_ref.path),
                ancestor_level: node_ref.ancestor_level,
                marker: PhantomData,
            })),
        }
    }

    fn read_internal_node_ref(
        r: &mut impl BodyReader,
        class_id: u32,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error> {
        read_node_ref_internal_impl::<T>(r, class_id)
    }
}

impl<T: 'static + ClassId + Default + ReadBody + Send + Sync> ReadNodeRef for Option<NodeRef<T>> {
    fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error> {
        match node_ref {
            NodeRef::Internal(node_ref) => {
                let node_ref = node_ref
                    .downcast()
                    .map_err(|_| Error::new("node reference type does not match"))?;

                Ok(Some(NodeRef::Internal(node_ref)))
            }
            NodeRef::External(node_ref) => Ok(Some(NodeRef::External(ExternalNodeRef {
                path: Arc::clone(&node_ref.path),
                ancestor_level: node_ref.ancestor_level,
                marker: PhantomData,
            }))),
        }
    }

    fn read_internal_node_ref(
        r: &mut impl BodyReader,
        class_id: u32,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error> {
        read_node_ref_internal_impl::<T>(r, class_id)
    }

    fn none() -> Result<Self, Error> {
        Ok(None)
    }
}

fn read_node_ref_internal_impl<T: 'static + ClassId + Default + ReadBody + Send + Sync>(
    r: &mut impl BodyReader,
    class_id: u32,
) -> Result<Arc<dyn Any + Send + Sync>, Error> {
    if class_id != T::CLASS_ID {
        return Err(Error::new("class id does not match"));
    }

    let node = read_node_from_body::<T>(r)?;

    Ok(Arc::new(node))
}
