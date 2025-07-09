use std::io::Read;

use crate::{
    ClassId, NodeRef,
    read::{
        Error, IdTable, NodeRefTable, ReadBody,
        id::{TryFromId, read_id},
        node_ref::{ReadNodeRef, read_node_ref},
        reader::{HeaderReader, Reader},
    },
};

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
pub struct BodyReaderImpl<'n, R> {
    /// Reader.
    pub reader: R,
    /// Identifier table.
    pub id_table: IdTable,
    /// Node table.
    pub node_table: &'n NodeRefTable, // Needs to be a reference because of borrow checker magic.
}

impl<'n, R: Read> Read for BodyReaderImpl<'n, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<'n, R: Read> HeaderReader for BodyReaderImpl<'n, R> {
    fn id<T: TryFromId>(&mut self) -> Result<T, Error> {
        read_id(&mut self.reader, &mut self.id_table)
    }
}

impl<'n, R: Read> BodyReader for BodyReaderImpl<'n, R> {
    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<T, Error> {
        read_node_ref(self)
    }
}
