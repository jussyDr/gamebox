//! Low-level GameBox writer.

use std::{
    any::Any,
    hash::{Hash, Hasher},
    io::{Error, Seek, Write},
    sync::Arc,
};

use indexmap::{indexset, IndexSet};

use crate::{Class, Vec2, Vec3, ID_MARKER_BIT};

use super::{write_body, BodyChunks};

pub trait ToLe {
    /// Convert `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op.
    fn to_le(self) -> Self;
}

impl ToLe for u8 {
    fn to_le(self) -> Self {
        self
    }
}

impl ToLe for u32 {
    fn to_le(self) -> Self {
        self.to_le()
    }
}

impl ToLe for f32 {
    fn to_le(self) -> Self {
        Self::from_bits(self.to_bits().to_le())
    }
}

impl ToLe for Vec2 {
    fn to_le(mut self) -> Self {
        self.x = self.x.to_le();
        self.y = self.y.to_le();

        self
    }
}

impl ToLe for Vec3 {
    fn to_le(mut self) -> Self {
        self.x = self.x.to_le();
        self.y = self.y.to_le();
        self.z = self.z.to_le();

        self
    }
}

/// Identifier state.
pub struct IdState {
    seen_id: bool,
    ids: IndexSet<Arc<str>>,
}

impl IdState {
    pub fn new() -> Self {
        Self {
            seen_id: false,
            ids: indexset![],
        }
    }
}

impl Default for IdState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IdStateMut {
    fn get_mut(&mut self) -> &mut IdState;
}

impl IdStateMut for IdState {
    fn get_mut(&mut self) -> &mut IdState {
        self
    }
}

impl<T: IdStateMut> IdStateMut for &mut T {
    fn get_mut(&mut self) -> &mut IdState {
        (**self).get_mut()
    }
}

struct InternalNode {
    node: Arc<dyn Any>,
}

impl PartialEq for InternalNode {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl Eq for InternalNode {}

impl Hash for InternalNode {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        todo!()
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

/// Low-level GameBox writer.
pub struct Writer<W, I, N> {
    inner: W,
    id_state: I,
    node_state: N,
}

impl<W, I, N> Writer<W, I, N> {
    /// Create a new writer.
    pub const fn new(inner: W, id_state: I, node_state: N) -> Self {
        Self {
            inner,
            id_state,
            node_state,
        }
    }

    pub fn into_inner(self) -> W {
        self.inner
    }

    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    pub fn to_buf_inline(
        &mut self,
        mut write_fn: impl FnMut(&mut Writer<Vec<u8>, &mut I, &mut N>) -> Result<(), Error>,
    ) -> Result<Vec<u8>, Error> {
        let mut w = Writer::new(vec![], &mut self.id_state, &mut self.node_state);

        write_fn(&mut w)?;

        Ok(w.inner)
    }
}

impl<W: Write, I, N> Writer<W, I, N> {
    /// Write bytes.
    pub fn bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.inner.write_all(bytes)?;

        Ok(())
    }

    /// Write an unsigned 8-bit integer.
    pub fn u8(&mut self, value: u8) -> Result<(), Error> {
        self.bytes(&[value])
    }

    /// Write an unsigned 16-bit integer.
    pub fn u16(&mut self, value: u16) -> Result<(), Error> {
        self.bytes(&value.to_le_bytes())
    }

    /// Write an unsigned 32-bit integer.
    pub fn u32(&mut self, value: u32) -> Result<(), Error> {
        self.bytes(&value.to_le_bytes())
    }

    /// Write an unsigned 64-bit integer.
    pub fn u64(&mut self, value: u64) -> Result<(), Error> {
        self.bytes(&value.to_le_bytes())
    }

    pub fn bool(&mut self, value: bool) -> Result<(), Error> {
        self.u32(if value { 1 } else { 0 })
    }

    pub fn vec2(&mut self, vec: Vec2) -> Result<(), Error> {
        todo!()
    }

    pub fn vec3(&mut self, vec: Vec3) -> Result<(), Error> {
        todo!()
    }

    pub fn byte_buf(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.u32(bytes.len() as u32)?;
        self.bytes(bytes)
    }

    pub fn byte_buf_inline(
        &mut self,
        write_fn: impl FnMut(&mut Writer<Vec<u8>, &mut I, &mut N>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let buf = self.to_buf_inline(write_fn)?;
        self.byte_buf(&buf)
    }

    pub fn string(&mut self, value: &str) -> Result<(), Error> {
        self.byte_buf(value.as_bytes())
    }

    pub fn string_or_empty(&mut self, value: Option<&String>) -> Result<(), Error> {
        match value {
            Some(value) => self.string(value),
            None => self.u32(0),
        }
    }
}

impl<W: Write, I: IdStateMut, N> Writer<W, I, N> {
    pub fn id_or_null(&mut self, id: Option<&Arc<str>>) -> Result<(), Error> {
        if !self.id_state.get_mut().seen_id {
            self.u32(3)?;

            self.id_state.get_mut().seen_id = true;
        }

        match id {
            Some(id) => match self.id_state.get_mut().ids.get_index_of(id) {
                Some(index) => {
                    self.u32(((index as u32) + 1) | ID_MARKER_BIT)?;
                }
                None => {
                    self.u32(ID_MARKER_BIT)?;
                    self.string(&id)?;

                    self.id_state.get_mut().ids.insert(Arc::clone(id));
                }
            },
            None => {
                self.u32(0xffffffff)?;
            }
        }

        Ok(())
    }

    pub fn id(&mut self, id: &Arc<str>) -> Result<(), Error> {
        self.id_or_null(Some(id))
    }
}

impl<W: Write + Seek, I, N: NodeStateMut> Writer<W, I, N> {
    pub fn internal_node_ref_or_null<T: 'static + Class + BodyChunks>(
        &mut self,
        node: Option<Arc<T>>,
    ) -> Result<(), Error> {
        match node {
            Some(node) => {
                let internal_node = InternalNode {
                    node: Arc::clone(&node) as Arc<dyn Any>,
                };

                match self.node_state.get_mut().nodes.get_index_of(&internal_node) {
                    Some(index) => {
                        self.u32(index as u32 + 1)?;
                    }
                    None => {
                        let index = self.node_state.get_mut().nodes.len() as u32 + 1;
                        self.u32(index)?;
                        self.u32(T::CLASS_ID)?;
                        write_body(self, node.as_ref())?;

                        self.node_state.get_mut().nodes.insert(internal_node);
                    }
                }
            }
            None => {
                self.u32(0xffffffff)?;
            }
        }

        Ok(())
    }

    pub fn internal_node_ref<T: 'static + Class + BodyChunks>(
        &mut self,
        node: Arc<T>,
    ) -> Result<(), Error> {
        self.internal_node_ref_or_null(Some(node))
    }
}
