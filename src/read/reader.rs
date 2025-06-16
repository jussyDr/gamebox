//! Reader

use std::{
    cell::OnceCell,
    io::{self, Read},
    iter,
    path::Path,
    sync::Arc,
};

use crate::{
    Quat, Vec3,
    read::{Class, Error},
};

/// Reader
pub struct Reader<R, I, N> {
    inner: R,
    ids: I,
    nodes: N,
}

/// Reference to a node.
#[derive(Clone, Debug)]
pub enum NodeRef {
    Internal(Arc<dyn Class>),
    /// Reference to a node in an external file.
    External(ExternalNodeRef),
}

/// Reference to a node in an external file.
#[derive(Clone, Debug)]
pub struct ExternalNodeRef {
    pub path: Arc<Path>,
    pub ancestor_level: u32,
}

impl Default for ExternalNodeRef {
    fn default() -> Self {
        Self {
            path: Arc::from(Path::new("")),
            ancestor_level: 0,
        }
    }
}

pub struct NodeRefs {
    node_refs: Vec<OnceCell<NodeRef>>,
}

impl NodeRefs {
    pub fn new(num_nodes: usize) -> Self {
        Self {
            node_refs: vec![OnceCell::new(); num_nodes],
        }
    }

    pub fn set_external(
        &self,
        index: u32,
        external_node_ref: ExternalNodeRef,
    ) -> Result<(), Error> {
        self.node_refs
            .get(index as usize)
            .ok_or(Error("node index exceeds number of nodes"))?
            .set(NodeRef::External(external_node_ref))
            .map_err(|_| Error(""))
    }
}

pub trait NodesMut {
    fn get(&self) -> &NodeRefs;

    fn get_mut(&mut self) -> &mut NodeRefs;
}

impl NodesMut for NodeRefs {
    fn get(&self) -> &NodeRefs {
        self
    }

    fn get_mut(&mut self) -> &mut NodeRefs {
        self
    }
}

impl<R, I, N> Reader<R, I, N> {
    /// New
    pub const fn new(inner: R, ids: I, nodes: N) -> Self {
        Self { inner, ids, nodes }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

fn map_io_error(_io_error: io::Error) -> Error {
    Error("IO error")
}

impl<R: Read, I, N> Reader<R, I, N> {
    /// Read `n` bytes.
    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; n];
        self.inner.read_exact(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    /// Read `L` bytes as an array.
    pub fn byte_array<const L: usize>(&mut self) -> Result<[u8; L], Error> {
        let mut buf = [0; L];
        self.inner.read_exact(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    /// Read an unsigned 8-bit integer.
    pub fn u8(&mut self) -> Result<u8, Error> {
        let bytes = self.byte_array()?;

        Ok(u8::from_le_bytes(bytes))
    }

    /// Read an unsigned 16-bit integer.
    pub fn u16(&mut self) -> Result<u16, Error> {
        let bytes = self.byte_array()?;

        Ok(u16::from_le_bytes(bytes))
    }

    /// Read an unsigned 32-bit integer.
    pub fn u32(&mut self) -> Result<u32, Error> {
        let bytes = self.byte_array()?;

        Ok(u32::from_le_bytes(bytes))
    }

    /// Read an unsigned 64-bit integer.
    pub fn u64(&mut self) -> Result<u64, Error> {
        let bytes = self.byte_array()?;

        Ok(u64::from_le_bytes(bytes))
    }

    /// Read a 32-bit floating point number
    pub fn f32(&mut self) -> Result<f32, Error> {
        let bytes = self.byte_array()?;

        Ok(f32::from_le_bytes(bytes))
    }

    /// Read an 8-bit boolean value.
    pub fn bool8(&mut self) -> Result<bool, Error> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error("expected an 8-bit boolean")),
        }
    }

    /// Read a 32-bit boolean value.
    pub fn bool32(&mut self) -> Result<bool, Error> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error("expected a 32-bit boolean")),
        }
    }

    /// Read a 3-dimensional vector.
    pub fn vec3(&mut self) -> Result<Vec3, Error> {
        let x = self.f32()?;
        let y = self.f32()?;
        let z = self.f32()?;

        Ok(Vec3 { x, y, z })
    }

    /// Read a quaternion.
    pub fn quat(&mut self) -> Result<Quat, Error> {
        let x = self.f32()?;
        let y = self.f32()?;
        let z = self.f32()?;
        let w = self.f32()?;

        Ok(Quat { x, y, z, w })
    }

    /// Read an UTF-8 encoded string.
    pub fn string(&mut self) -> Result<String, Error> {
        let len = self.u32()?;
        let bytes = self.bytes(len as usize)?;

        String::from_utf8(bytes).map_err(|_| Error("expected an UTF-8 encoded string"))
    }

    pub fn repeat<T>(
        &mut self,
        n: usize,
        read_elem: impl Fn(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        iter::repeat_with(|| read_elem(self)).take(n).collect()
    }

    pub fn list<T>(
        &mut self,
        read_elem: impl Fn(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let len = self.u32()?;

        self.repeat(len as usize, read_elem)
    }

    pub fn expect_eof(&mut self) -> Result<(), Error> {
        let mut buf = [0];
        let n = self.inner.read(&mut buf).map_err(map_io_error)?;

        if n != 0 {
            return Err(Error("expected EOF"));
        }

        Ok(())
    }

    /// Read all bytes until EOF.
    pub fn read_to_end(&mut self) -> Result<Vec<u8>, Error> {
        let mut buf = vec![];
        self.inner.read_to_end(&mut buf).map_err(map_io_error)?;

        Ok(buf)
    }

    pub fn node_or_null_generic<T>(
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

    pub fn node_generic<T>(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let node = self.node_or_null_generic(read_fn)?;

        match node {
            None => Err(Error("node is null")),
            Some(node) => Ok(node),
        }
    }
}

impl<R: Read, I, N: NodesMut> Reader<R, I, N> {
    pub fn node_ref_generic(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Class>, Error>,
    ) -> Result<NodeRef, Error> {
        let index = self
            .u32()?
            .checked_sub(1)
            .ok_or(Error("node index is zero"))?;

        let slot = self
            .nodes
            .get()
            .node_refs
            .get(index as usize)
            .ok_or(Error("node index exceeds number of nodes"))?;

        match slot.get() {
            None => {
                let node = self.node_generic(read_fn)?;

                let slot = self.nodes.get().node_refs.get(index as usize).unwrap();

                slot.set(NodeRef::Internal(Arc::clone(&node))).unwrap();

                Ok(NodeRef::Internal(node))
            }
            Some(node_ref) => Ok(NodeRef::clone(node_ref)),
        }
    }

    pub fn internal_node_ref_generic(
        &mut self,
        read_fn: impl Fn(&mut Self, u32) -> Result<Arc<dyn Class>, Error>,
    ) -> Result<Arc<dyn Class>, Error> {
        let node_ref = self.node_ref_generic(read_fn)?;

        match node_ref {
            NodeRef::Internal(node) => Ok(node),
            NodeRef::External(_) => Err(Error("expected an internal node reference")),
        }
    }

    pub fn external_node_ref(&mut self) -> Result<ExternalNodeRef, Error> {
        let index = self
            .u32()?
            .checked_sub(1)
            .ok_or(Error("node index is zero"))?;

        let slot = self
            .nodes
            .get()
            .node_refs
            .get(index as usize)
            .ok_or(Error("node index exceeds number of nodes"))?;

        match slot.get() {
            None => {
                todo!()
            }
            Some(NodeRef::Internal(_)) => {
                todo!()
            }
            Some(NodeRef::External(external_node_ref)) => Ok(external_node_ref.clone()),
        }
    }
}
