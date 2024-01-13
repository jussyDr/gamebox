//! Low-level GameBox serialization.

mod id;
mod node;

pub use id::*;
pub use node::*;

use std::io::Write;

use crate::write::Result;

/// Low-level GameBox serializer.
pub struct Serializer<W, I, N> {
    writer: W,
    id_state: I,
    node_state: N,
}

impl<W, I, N> Serializer<W, I, N> {
    /// Create a new `Serializer` with the given `id_state` and `node_state`.
    pub fn new(writer: W, id_state: I, node_state: N) -> Self {
        Self {
            writer,
            id_state,
            node_state,
        }
    }

    /// Gets a mutable reference to the underlying writer.
    pub fn get_writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }
}

impl<W: Write, I, N> Serializer<W, I, N> {
    /// Write an unsigned 8-bit integer.
    #[inline]
    pub fn u8(&mut self, x: u8) -> Result {
        self.byte_array(x.to_le_bytes())
    }

    /// Write an unsigned 16-bit integer.
    #[inline]
    pub fn u16(&mut self, x: u16) -> Result {
        self.byte_array(x.to_le_bytes())
    }

    /// Write an unsigned 32-bit integer.
    #[inline]
    pub fn u32(&mut self, x: u32) -> Result {
        self.byte_array(x.to_le_bytes())
    }

    /// Write a 32-bit floating point number.
    #[inline]
    pub fn f32(&mut self, x: f32) -> Result {
        self.byte_array(x.to_le_bytes())
    }

    /// Write the given `bytes`.
    pub fn bytes(&mut self, bytes: &[u8]) -> Result {
        self.writer.write_all(bytes)?;
        Ok(())
    }

    /// Write the bytes in the given `array`.
    #[inline]
    pub fn byte_array<const L: usize>(&mut self, array: [u8; L]) -> Result {
        self.bytes(&array)
    }

    /// Write a string.
    pub fn string(&mut self, s: &str) -> Result {
        self.u32(s.len() as u32)?;
        self.bytes(s.as_bytes())
    }

    /// Write a byte buffer with contents written by the given `write_fn`.
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

    /// Write a byte buffer with contents written by the given `write_fn` and with a new `IdState` and `NodeState`.
    pub fn scoped_buffer(
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
