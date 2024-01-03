use std::{cell::Cell, io::Write};

use elsa::FrozenMap;

use crate::{
    common::{ID_FLAG_BIT, ID_INDEX_MASK, ID_VERSION},
    write::Result,
};

use super::Serializer;

/// Identifier state.
pub struct IdState {
    written_id: Cell<bool>,
    ids: FrozenMap<String, u16>,
}

impl IdState {
    /// Create a new identifier state.
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

/// Can obtain a immutable reference to an identifier state.
pub trait IdStateRef {
    /// Obtain a immutable reference to an identifier state.
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

impl<W: Write, I: IdStateRef, N> Serializer<W, I, N> {
    /// Write an identifier.
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

    /// Write a null identifier.
    pub fn null_id(&mut self) -> Result {
        write_id_version(self)?;

        self.u32(0xffffffff)
    }
}

fn write_id_version<W: Write, I: IdStateRef, N>(s: &mut Serializer<W, I, N>) -> Result {
    if !s.id_state.borrow().written_id.get() {
        s.u32(ID_VERSION)?;

        s.id_state.borrow().written_id.set(true);
    }

    Ok(())
}
