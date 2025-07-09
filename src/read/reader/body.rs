use std::{
    any::{Any, type_name, type_name_of_val},
    io::Read,
    marker::PhantomData,
    sync::Arc,
};

use crate::{
    ClassId, ExternalNodeRef, NodeRef,
    read::{
        Error, ReadBody, read_node_from_body,
        reader::{HeaderReader, IdTable, NodeTable},
    },
};

pub trait ReadNodeRef {
    fn from_any(node_ref: Option<NodeRef<dyn Any + Send + Sync>>) -> Result<Self, Error>
    where
        Self: Sized;

    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Option<NodeRef<dyn Any + Send + Sync>>, Error>
    where
        Self: Sized;
}

impl<T: 'static + ClassId + Default + ReadBody + Send + Sync> ReadNodeRef for Arc<T> {
    fn from_any(node_ref: Option<NodeRef<dyn Any + Send + Sync>>) -> Result<Self, Error> {
        match node_ref {
            Some(NodeRef::Internal(node_ref)) => {
                let node_ref = node_ref
                    .downcast()
                    .map_err(|_| Error::new("node reference type does not match"))?;

                Ok(node_ref)
            }
            _ => todo!(),
        }
    }

    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Option<NodeRef<dyn Any + Send + Sync>>, Error> {
        match class_id {
            Some(ClassIdOrSubExtension::ClassId(class_id)) => {
                if class_id != T::CLASS_ID {
                    todo!()
                }

                let node = read_node_from_body::<T>(r)?;

                Ok(Some(NodeRef::Internal(Arc::new(node))))
            }
            _ => todo!(),
        }
    }
}

impl<T: 'static + ClassId + Default + ReadBody + Send + Sync> ReadNodeRef for Option<Arc<T>> {
    fn from_any(node_ref: Option<NodeRef<dyn Any + Send + Sync>>) -> Result<Self, Error> {
        match node_ref {
            Some(NodeRef::Internal(node_ref)) => {
                let node_ref = node_ref
                    .downcast()
                    .map_err(|_| Error::new("node reference type does not match"))?;

                Ok(Some(node_ref))
            }
            None => Ok(None),
            _ => todo!(),
        }
    }

    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Option<NodeRef<dyn Any + Send + Sync>>, Error> {
        match class_id {
            Some(ClassIdOrSubExtension::ClassId(class_id)) => {
                if class_id != T::CLASS_ID {
                    todo!()
                }

                let node = read_node_from_body::<T>(r)?;

                Ok(Some(NodeRef::Internal(Arc::new(node))))
            }
            None => Ok(None),
            _ => todo!(),
        }
    }
}

impl<T> ReadNodeRef for ExternalNodeRef<T> {
    fn from_any(node_ref: Option<NodeRef<dyn Any + Send + Sync>>) -> Result<Self, Error> {
        match node_ref {
            Some(NodeRef::External(node_ref)) => Ok(ExternalNodeRef {
                path: Arc::clone(&node_ref.path),
                ancestor_level: node_ref.ancestor_level,
                marker: PhantomData,
            }),
            _ => todo!(),
        }
    }

    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Option<NodeRef<dyn Any + Send + Sync>>, Error> {
        todo!()
    }
}

impl<T> ReadNodeRef for Option<ExternalNodeRef<T>> {
    fn from_any(node_ref: Option<NodeRef<dyn Any + Send + Sync>>) -> Result<Self, Error> {
        match node_ref {
            Some(NodeRef::External(node_ref)) => Ok(Some(ExternalNodeRef {
                path: Arc::clone(&node_ref.path),
                ancestor_level: node_ref.ancestor_level,
                marker: PhantomData,
            })),
            None => Ok(None),
            _ => todo!(),
        }
    }

    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Option<NodeRef<dyn Any + Send + Sync>>, Error> {
        match class_id {
            None => Ok(None),
            _ => todo!(),
        }
    }
}

impl<T: 'static + ClassId + Default + ReadBody + Send + Sync> ReadNodeRef for NodeRef<T> {
    fn from_any(node_ref: Option<NodeRef<dyn Any + Send + Sync>>) -> Result<Self, Error> {
        match node_ref {
            Some(NodeRef::Internal(node_ref)) => {
                let node_ref = node_ref
                    .downcast()
                    .map_err(|_| Error::new("node reference type does not match"))?;

                Ok(NodeRef::Internal(node_ref))
            }
            Some(NodeRef::External(node_ref)) => Ok(NodeRef::External(ExternalNodeRef {
                path: Arc::clone(&node_ref.path),
                ancestor_level: node_ref.ancestor_level,
                marker: PhantomData,
            })),
            _ => todo!(),
        }
    }

    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Option<NodeRef<dyn Any + Send + Sync>>, Error> {
        match class_id {
            Some(ClassIdOrSubExtension::ClassId(class_id)) => {
                if class_id != T::CLASS_ID {
                    todo!()
                }

                let node = read_node_from_body::<T>(r)?;

                Ok(Some(NodeRef::Internal(Arc::new(node))))
            }
            _ => todo!(),
        }
    }
}

impl<T: 'static + ClassId + Default + ReadBody + Send + Sync> ReadNodeRef for Option<NodeRef<T>> {
    fn from_any(node_ref: Option<NodeRef<dyn Any + Send + Sync>>) -> Result<Self, Error> {
        match node_ref {
            Some(NodeRef::Internal(node_ref)) => {
                let node_ref = node_ref
                    .downcast()
                    .map_err(|_| Error::new("node reference type does not match"))?;

                Ok(Some(NodeRef::Internal(node_ref)))
            }
            Some(NodeRef::External(node_ref)) => Ok(Some(NodeRef::External(ExternalNodeRef {
                path: Arc::clone(&node_ref.path),
                ancestor_level: node_ref.ancestor_level,
                marker: PhantomData,
            }))),
            None => Ok(None),
        }
    }

    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Option<NodeRef<dyn Any + Send + Sync>>, Error> {
        match class_id {
            Some(ClassIdOrSubExtension::ClassId(class_id)) => {
                if class_id != T::CLASS_ID {
                    todo!()
                }

                let node = read_node_from_body::<T>(r)?;

                Ok(Some(NodeRef::Internal(Arc::new(node))))
            }
            None => Ok(None),
            _ => todo!(),
        }
    }
}

pub enum ClassIdOrSubExtension {
    ClassId(u32),
    SubExtension(String),
}

/// Body reader.
pub trait BodyReader: HeaderReader {
    /// Node table.
    fn node_table(&mut self) -> &mut NodeTable;

    /// WIP.
    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<T, Error>
    where
        Self: Sized,
    {
        let index = self.u32()?;

        if index == 0xffffffff {
            return T::from_any(T::read_node_ref(self, None)?);
        }

        let index = index
            .checked_sub(1)
            .ok_or_else(|| Error::new("node reference index is zero"))?;

        let slot = self
            .node_table()
            .nodes
            .get(index as usize)
            .ok_or_else(|| Error::new("node reference index exceeds number of nodes"))?;

        match slot {
            None => {
                let class_id = self.u32()?;

                let node_ref =
                    T::read_node_ref(self, Some(ClassIdOrSubExtension::ClassId(class_id)))?;

                match node_ref {
                    Some(NodeRef::Internal(node_ref)) => {
                        *self.node_table().nodes.get_mut(index as usize).unwrap() =
                            Some(NodeRef::Internal(Arc::clone(&node_ref)));

                        T::from_any(Some(NodeRef::Internal(node_ref)))
                    }
                    _ => todo!(),
                }
            }
            Some(node_ref) => T::from_any(Some(NodeRef::clone(node_ref))),
        }
    }

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

/// Body reader.
pub struct BodyReaderImpl<R, I, N> {
    /// Reader.
    pub reader: R,
    /// Identifier table.
    pub id_table: I,
    /// Node table.
    pub node_table: N,
}

impl<R: Read, I, N> Read for BodyReaderImpl<R, I, N> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read, I: AsMut<IdTable>, N> HeaderReader for BodyReaderImpl<R, I, N> {
    fn id_table(&mut self) -> &mut IdTable {
        self.id_table.as_mut()
    }
}

impl<R: Read, I: AsMut<IdTable>, N: AsMut<NodeTable>> BodyReader for BodyReaderImpl<R, I, N> {
    fn node_table(&mut self) -> &mut NodeTable {
        self.node_table.as_mut()
    }
}
