use std::{any::Any, io::Read, marker::PhantomData, sync::Arc};

use crate::{
    ClassId, ExternalNodeRef, NodeRef,
    read::{
        Error, ReadBody, error_unknown_version, read_node_from_body,
        reader::{HeaderReader, IdTable, NodeTable, Reader, header::TryFromId},
    },
};

/// Trait implemented by types that are readable from a node reference.
pub trait ReadNodeRef {
    fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error>
    where
        Self: Sized;

    fn read_node_ref_internal(
        r: &mut impl BodyReader,
        class_id: u32,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error>
    where
        Self: Sized,
    {
        Err(Error::new("expected external node reference"))
    }

    fn none() -> Result<Self, Error>
    where
        Self: Sized,
    {
        Err(Error::new("expected non-null node reference"))
    }
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

    fn read_node_ref_internal(
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

    fn read_node_ref_internal(
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

    fn read_node_ref_internal(
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

    fn read_node_ref_internal(
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

/// Body reader.
pub trait BodyReader: HeaderReader {
    /// Read a node reference.
    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<T, Error>
    where
        Self: Sized;

    /// Read a node.
    fn node<T: Default + ClassId + ReadBody>(&mut self) -> Result<T, Error>
    where
        Self: Sized,
    {
        let node = self.node_generic(|r, class_id| {
            if class_id != T::CLASS_ID {
                todo!("{:08X?}", class_id);
            }

            let mut node = T::default();
            node.read_body(r)?;

            Ok(node)
        })?;

        Ok(node)
    }

    /// Read a node.
    fn node_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let node = self.node_or_null_generic(read_fn)?;

        match node {
            None => Err(Error::new("node is null")),
            Some(node) => Ok(node),
        }
    }

    /// Read a node.
    fn node_or_null_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<Option<T>, Error> {
        let class_id = self.u32()?;

        if class_id == 0xffffffff {
            return Ok(None);
        }

        let node = read_fn(self, class_id)?;

        Ok(Some(node))
    }
}

/// Implementation of the `BodyReader` trait.
pub struct BodyReaderImpl<'r, 'n, R> {
    /// Reader.
    pub reader: &'r mut R,
    /// Identifier table.
    pub id_table: IdTable,
    /// Node table.
    pub node_table: &'n NodeTable,
}

impl<'r, 'n, R: Read> Read for BodyReaderImpl<'r, 'n, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<'r, 'n, R: Read> HeaderReader for BodyReaderImpl<'r, 'n, R> {
    fn id<T: TryFromId>(&mut self) -> Result<T, Error> {
        let id = id_or_null(self)?;

        T::try_from_id(id)
    }
}

fn id_or_null<'r, 'n, R: Read>(
    r: &mut BodyReaderImpl<'r, 'n, R>,
) -> Result<Option<Arc<str>>, Error> {
    if !r.id_table.seen_id {
        let version = r.u32()?;

        if version != 3 {
            return Err(error_unknown_version("identifier", version));
        }

        r.id_table.seen_id = true;
    }

    let index = r.u32()?;

    if index == 0xffffffff {
        return Ok(None);
    }

    if index == 0x0000001a {
        // Not sure what this is yet.
        return Ok(Some(Arc::from("")));
    }

    if index == 0x00002713 {
        // Not sure what this is yet.
        return Ok(Some(Arc::from("")));
    }

    if index & 0x40000000 == 0 {
        return Err(Error::new("expected an identifier"));
    }

    let index = index & 0x37ffffff;

    match index.checked_sub(1) {
        None => {
            let id = Arc::from(r.string()?);
            r.id_table.ids.push(Arc::clone(&id));

            Ok(Some(id))
        }
        Some(index) => {
            let id = r
                .id_table
                .ids
                .get(index as usize)
                .ok_or_else(|| Error::new(""))?;

            Ok(Some(Arc::clone(id)))
        }
    }
}

impl<'r, 'n, R: Read> BodyReader for BodyReaderImpl<'r, 'n, R> {
    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<T, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return T::none();
        }

        let index = index
            .checked_sub(1)
            .ok_or_else(|| Error::new("node reference index is zero"))?;

        let slot = self
            .node_table
            .nodes
            .get(index as usize)
            .ok_or_else(|| Error::new("node reference index exceeds number of nodes"))?;

        let node_ref = match slot.get() {
            None => {
                let class_id = self.u32()?;

                let node_ref = NodeRef::Internal(T::read_node_ref_internal(self, class_id)?);

                slot.set(NodeRef::clone(&node_ref))
                    .map_err(|_| Error::new("reentrant init"))?;

                node_ref
            }
            Some(node_ref) => NodeRef::clone(&node_ref),
        };

        T::from_node_ref_any(node_ref)
    }
}
