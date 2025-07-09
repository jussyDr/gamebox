use std::{io::Read, sync::Arc};

use crate::{
    ClassId, ExternalNodeRef, NodeRef,
    read::{
        Error, ReadBody,
        reader::{HeaderReader, IdTable, NodeTable},
    },
};

pub trait ReadNodeRef {
    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

impl<T> ReadNodeRef for Arc<T> {
    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Self, Error> {
        todo!()
    }
}

impl<T> ReadNodeRef for Option<Arc<T>> {
    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Self, Error> {
        todo!()
    }
}

impl<T> ReadNodeRef for ExternalNodeRef<T> {
    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Self, Error> {
        todo!()
    }
}

impl<T> ReadNodeRef for Option<ExternalNodeRef<T>> {
    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Self, Error> {
        todo!()
    }
}

impl<T> ReadNodeRef for NodeRef<T> {
    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Self, Error> {
        todo!()
    }
}

impl<T> ReadNodeRef for Option<NodeRef<T>> {
    fn read_node_ref(
        r: &mut impl BodyReader,
        class_id: Option<ClassIdOrSubExtension>,
    ) -> Result<Self, Error> {
        todo!()
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
            return T::read_node_ref(self, None);
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

                *self.node_table().nodes.get_mut(index as usize).unwrap() =
                    Some(NodeRef::Internal(node_ref));

                Ok(node_ref)
            }
            Some(NodeRef::Internal(node_ref)) => {
                todo!()
            }
            Some(NodeRef::External(node_ref)) => {
                todo!()
            }
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
