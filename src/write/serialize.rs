use std::{borrow::BorrowMut, cell::Cell, io::Write};

use elsa::FrozenMap;

use crate::{common::ID_FLAG_BIT, common::ID_INDEX_MASK, common::ID_VERSION, write::Result};

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

pub struct NodeState {
    num_nodes: u32,
}

impl NodeState {
    pub fn new() -> Self {
        Self { num_nodes: 1 }
    }

    pub const fn num_nodes(&self) -> u32 {
        self.num_nodes
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait NodeStateMut: BorrowMut<NodeState> {}

impl<T: BorrowMut<NodeState>> NodeStateMut for T {}

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

impl<W: Write, I, N: NodeStateMut> Serializer<W, I, N> {
    pub fn node_index(&mut self) -> Result {
        self.u32(self.node_state.borrow().num_nodes)?;
        self.node_state.borrow_mut().num_nodes += 1;

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
