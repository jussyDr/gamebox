use std::{borrow::BorrowMut, io::Write};

use crate::write::Result;

pub struct IdState {
    written_id: bool,
}

impl IdState {
    pub fn new() -> Self {
        Self { written_id: false }
    }
}

pub trait IdStateMut: BorrowMut<IdState> {}

impl<T: BorrowMut<IdState>> IdStateMut for T {}

pub struct NodeState {
    num_nodes: u32,
}

impl NodeState {
    pub fn new() -> Self {
        Self { num_nodes: 0 }
    }

    pub fn num_nodes(&self) -> u32 {
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
}

impl<W: Write, I: IdStateMut, N> Serializer<W, I, N> {
    pub fn id(&mut self, id: &str) -> Result<()> {
        if !self.id_state.borrow().written_id {
            self.u32(3)?;

            self.id_state.borrow_mut().written_id = true;
        }

        self.u32(0x40000000)?;
        self.string(id)
    }

    pub fn null_id(&mut self) -> Result<()> {
        if !self.id_state.borrow().written_id {
            self.u32(3)?;

            self.id_state.borrow_mut().written_id = true;
        }

        self.u32(0xffffffff)
    }
}

impl<W: Write, I, N: NodeStateMut> Serializer<W, I, N> {
    pub fn node_index(&mut self) -> Result<()> {
        self.node_state.borrow_mut().num_nodes += 1;
        self.u32(self.node_state.borrow().num_nodes)
    }
}
