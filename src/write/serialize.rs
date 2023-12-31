use std::{
    any::{Any, TypeId},
    cell::Cell,
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
};

use elsa::FrozenMap;

use crate::{
    common::ID_FLAG_BIT,
    common::ID_INDEX_MASK,
    common::{ClassId, ID_VERSION, NODE_END},
    write::Result,
};

use super::writable::WriteBody;

pub struct IdState {
    written_id: Cell<bool>,
    ids: FrozenMap<String, u16>,
}

impl IdState {
    pub fn new() -> Self {
        Self {
            written_id: Cell::new(false),
            ids: FrozenMap::new(),
        }
    }
}

impl Default for IdState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IdStateRef {
    fn borrow(&self) -> &IdState;
}

impl IdStateRef for IdState {
    fn borrow(&self) -> &IdState {
        self
    }
}

impl<T: IdStateRef> IdStateRef for &mut T {
    fn borrow(&self) -> &IdState {
        (**self).borrow()
    }
}

trait Node {
    fn eq(&self, other: &dyn Node) -> bool;

    fn hash(&self) -> u64;

    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for Box<dyn Node> {
    fn eq(&self, other: &Self) -> bool {
        Node::eq(self, other)
    }
}

impl Eq for Box<dyn Node> {}

impl Hash for Box<dyn Node> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(Node::hash(self));
    }
}

impl<T: 'static + Eq + Hash> Node for T {
    fn eq(&self, other: &dyn Node) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            return self == other;
        }

        false
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        Hash::hash(&(TypeId::of::<T>(), self), &mut hasher);
        hasher.finish()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NodeState {
    num_nodes: Cell<u32>,
    nodes: FrozenMap<Box<dyn Node>, u32>,
}

impl NodeState {
    pub fn new() -> Self {
        Self {
            num_nodes: Cell::new(0),
            nodes: FrozenMap::new(),
        }
    }

    pub fn num_nodes(&self) -> u32 {
        self.num_nodes.get() + 1
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait NodeStateRef {
    fn borrow(&self) -> &NodeState;
}

impl NodeStateRef for NodeState {
    fn borrow(&self) -> &NodeState {
        self
    }
}

impl<T: NodeStateRef> NodeStateRef for &T {
    fn borrow(&self) -> &NodeState {
        (**self).borrow()
    }
}

pub struct Serializer<W, I, N> {
    writer: W,
    id_state: I,
    node_state: N,
}

impl<W, I, N> Serializer<W, I, N> {
    pub fn new(writer: W, id_state: I, node_state: N) -> Self {
        Self {
            writer,
            id_state,
            node_state,
        }
    }

    pub fn get_mut(&mut self) -> &mut W {
        &mut self.writer
    }
}

impl<W: Write, I, N> Serializer<W, I, N> {
    pub fn u8(&mut self, val: u8) -> Result {
        self.byte_array(val.to_le_bytes())
    }

    pub fn u16(&mut self, val: u16) -> Result {
        self.byte_array(val.to_le_bytes())
    }

    pub fn u32(&mut self, val: u32) -> Result {
        self.byte_array(val.to_le_bytes())
    }

    pub fn f32(&mut self, val: f32) -> Result {
        self.byte_array(val.to_le_bytes())
    }

    pub fn bytes(&mut self, bytes: &[u8]) -> Result {
        self.writer.write_all(bytes)?;
        Ok(())
    }

    pub fn byte_array<const L: usize>(&mut self, array: [u8; L]) -> Result {
        self.bytes(&array)
    }

    pub fn string(&mut self, s: &str) -> Result {
        self.u32(s.len() as u32)?;
        self.bytes(s.as_bytes())
    }

    pub fn buffer(
        &mut self,
        write_fn: impl Fn(&mut Serializer<&mut Vec<u8>, &mut I, &mut N>) -> Result,
    ) -> Result {
        let mut data = vec![];

        let mut s = Serializer::new(&mut data, &mut self.id_state, &mut self.node_state);
        write_fn(&mut s)?;

        self.u32(data.len() as u32)?;
        self.bytes(&data)
    }

    pub fn something(
        &mut self,
        write_fn: impl Fn(&mut Serializer<&mut Vec<u8>, IdState, NodeState>) -> Result,
    ) -> Result {
        let mut data = vec![];

        let mut s = Serializer::new(&mut data, IdState::new(), NodeState::new());
        write_fn(&mut s)?;

        self.u32(data.len() as u32)?;
        self.bytes(&data)
    }
}

impl<W: Write, I: IdStateRef, N> Serializer<W, I, N> {
    pub fn id(&mut self, id: &str) -> Result {
        write_id_version(self)?;

        match self.id_state.borrow().ids.get_copy(id) {
            None => {
                let index = self.id_state.borrow().ids.len() as u16 + 1;

                self.id_state.borrow().ids.insert_copy(id.to_owned(), index);

                self.u32(ID_FLAG_BIT)?;
                self.string(id)
            }
            Some(index) => self.u32(ID_FLAG_BIT | ((index as u32) & ID_INDEX_MASK)),
        }
    }

    pub fn null_id(&mut self) -> Result {
        write_id_version(self)?;

        self.u32(0xffffffff)
    }
}

impl<W: Write, I: IdStateRef, N: NodeStateRef> Serializer<W, I, N> {
    pub fn node_ref<T: 'static + Eq + Hash + ClassId + WriteBody + Clone>(
        &mut self,
        node: T,
    ) -> Result {
        let node_2: Box<dyn Node> = Box::new(node.clone());

        match self.node_state.borrow().nodes.get_copy(&node_2) {
            None => {
                let index = self.node_state.borrow().num_nodes.get() + 1;

                self.u32(index)?;

                self.node_state.borrow().num_nodes.set(index);

                self.u32(T::class_id())?;

                node.write_body(self)?;

                self.u32(NODE_END)?;

                self.node_state.borrow().nodes.insert_copy(node_2, index);
            }
            Some(index) => self.u32(index)?,
        }

        Ok(())
    }

    pub fn unique_node_ref<T: ClassId + WriteBody>(&mut self, node: &T) -> Result {
        let index = self.node_state.borrow().num_nodes.get() + 1;

        self.u32(index)?;

        self.node_state.borrow().num_nodes.set(index);

        self.u32(T::class_id())?;

        node.write_body(self)?;

        self.u32(NODE_END)?;

        Ok(())
    }
}

fn write_id_version<W: Write, I: IdStateRef, N>(s: &mut Serializer<W, I, N>) -> Result {
    if !s.id_state.borrow().written_id.get() {
        s.u32(ID_VERSION)?;

        s.id_state.borrow().written_id.set(true);
    }

    Ok(())
}
