use std::{borrow::BorrowMut, io::Write};

use indexmap::IndexSet;

use crate::write::Result;

pub struct IdState {
    written_id: bool,
    ids: IndexSet<String>,
}

impl IdState {
    pub fn new() -> Self {
        Self {
            written_id: false,
            ids: IndexSet::new(),
        }
    }
}

pub trait IdStateMut {
    fn borrow(&self) -> &IdState;
    fn borrow_mut(&mut self) -> &mut IdState;
}

impl IdStateMut for IdState {
    fn borrow(&self) -> &IdState {
        self
    }

    fn borrow_mut(&mut self) -> &mut IdState {
        self
    }
}

impl<T: IdStateMut> IdStateMut for &mut T {
    fn borrow(&self) -> &IdState {
        (**self).borrow()
    }

    fn borrow_mut(&mut self) -> &mut IdState {
        (**self).borrow_mut()
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
    pub fn u8(&mut self, val: u8) -> Result<()> {
        self.byte_array(val.to_le_bytes())
    }

    pub fn u16(&mut self, val: u16) -> Result<()> {
        self.byte_array(val.to_le_bytes())
    }

    pub fn u32(&mut self, val: u32) -> Result<()> {
        self.byte_array(val.to_le_bytes())
    }

    pub fn f32(&mut self, val: f32) -> Result<()> {
        self.byte_array(val.to_le_bytes())
    }

    pub fn bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer.write_all(bytes)?;
        Ok(())
    }

    pub fn byte_array<const L: usize>(&mut self, array: [u8; L]) -> Result<()> {
        self.bytes(&array)
    }

    pub fn string(&mut self, s: &str) -> Result<()> {
        self.u32(s.len() as u32)?;
        self.bytes(s.as_bytes())
    }

    pub fn byte_buffer(
        &mut self,
        write_fn: impl Fn(&mut Serializer<&mut Vec<u8>, &mut I, &mut N>) -> Result<()>,
    ) -> Result<()> {
        let mut data = vec![];

        let mut s = Serializer::new(&mut data, &mut self.id_state, &mut self.node_state);
        write_fn(&mut s)?;

        self.u32(data.len() as u32)?;
        self.bytes(&data)
    }

    pub fn something(
        &mut self,
        write_fn: impl Fn(&mut Serializer<&mut Vec<u8>, IdState, NodeState>) -> Result<()>,
    ) -> Result<()> {
        let mut data = vec![];

        let mut s = Serializer::new(&mut data, IdState::new(), NodeState::new());
        write_fn(&mut s)?;

        self.u32(data.len() as u32)?;
        self.bytes(&data)
    }
}

impl<W: Write, I: IdStateMut, N> Serializer<W, I, N> {
    pub fn id(&mut self, id: &str) -> Result<()> {
        write_id_version(self)?;

        match self.id_state.borrow().ids.get_index_of(id) {
            None => {
                self.id_state.borrow_mut().ids.insert(id.to_owned());

                self.u32(0x40000000)?;
                self.string(id)
            }
            Some(index) => self.u32(0x40000000 | ((index as u32 + 1) & 0x00003fff)),
        }
    }

    pub fn null_id(&mut self) -> Result<()> {
        write_id_version(self)?;

        self.u32(0xffffffff)
    }
}

impl<W: Write, I, N: NodeStateMut> Serializer<W, I, N> {
    pub fn node_index(&mut self) -> Result<()> {
        self.u32(self.node_state.borrow().num_nodes)?;
        self.node_state.borrow_mut().num_nodes += 1;

        Ok(())
    }
}

fn write_id_version<W: Write, I: IdStateMut, N>(s: &mut Serializer<W, I, N>) -> Result<()> {
    if !s.id_state.borrow().written_id {
        s.u32(3)?;

        s.id_state.borrow_mut().written_id = true;
    }

    Ok(())
}
