use std::{any::Any, io::Write, rc::Rc};

use indexmap::IndexSet;

use crate::{Ident, Vec2};

use super::Error;

pub trait Num {
    fn write<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error>;
}

impl Num for f32 {
    fn write<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
        w.f32(*self)
    }
}

pub struct IdState {
    ids: IndexSet<String>,
}

pub trait IdStateRef {
    fn get(&self) -> &IdState;
}

/// Allows to get a mutable reference to an [IdState].
pub trait IdStateMut: IdStateRef {
    /// Get a mutable reference to an [IdState].
    fn get_mut(&mut self) -> &mut IdState;
}

pub struct NodeState {
    nodes: IndexSet<Rc<dyn Any>>,
}

impl NodeState {
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
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
}

impl<W: Write, I, N> Writer<W, I, N> {
    /// Write the given `bytes`.
    pub fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> Result<(), Error> {
        self.inner.write_all(bytes.as_ref()).map_err(|_| Error)
    }

    /// Write a 32-bit floating point number.
    pub fn f32(&mut self, value: f32) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write an unsigned 8-bit integer.
    pub fn u8(&mut self, value: u8) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write an unsigned 16-bit integer.
    pub fn u16(&mut self, value: u16) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write an unsigned 32-bit integer.
    pub fn u32(&mut self, value: u32) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write an unsigned 64-bit integer.
    pub fn u64(&mut self, value: u64) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write an unsigned 128-bit integer.
    pub fn u128(&mut self, value: u128) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write a boolean.
    pub fn bool(&mut self, value: bool) -> Result<(), Error> {
        self.u32(value as u32)
    }

    /// Write a string.
    pub fn string(&mut self, s: impl AsRef<str>) -> Result<(), Error> {
        self.u32(s.as_ref().len() as u32)?;
        self.bytes(s.as_ref().as_bytes())?;

        Ok(())
    }

    /// Write a 2-dimensional vector.
    pub fn vec2(&mut self, value: &Vec2<impl Num>) -> Result<(), Error> {
        value.x.write(self)?;
        value.y.write(self)
    }
}

impl<W: Write, I: IdStateMut, N> Writer<W, I, N> {
    /// Write an identifier.
    pub fn id(&mut self, id: Option<impl AsRef<str>>) -> Result<(), Error> {
        if self.id_state.get().ids.is_empty() {
            self.u32(3)?;
        }

        match id {
            None => self.u32(0xffffffff)?,
            Some(id) => match self.id_state.get().ids.get_index_of(id.as_ref()) {
                None => {
                    self.u32(0x40000000)?;
                    self.string(id.as_ref())?;

                    self.id_state.get_mut().ids.insert(id.as_ref().to_owned());
                }
                Some(index) => {
                    let index = (index + 1) as u32;

                    self.u32(0x40000000 | index)?;
                }
            },
        }

        Ok(())
    }

    /// Write an identifier triple.
    pub fn ident(&mut self, ident: &Ident) -> Result<(), Error> {
        self.id(ident.id.as_ref())?;

        if ident.collection.is_some() {
            self.u32(26)?;
        } else {
            self.u32(0xffffffff)?;
        }

        self.id(ident.author.as_ref())
    }
}
