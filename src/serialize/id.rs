use std::{collections::HashMap, io::Write};

use crate::{
    common::{ID_FLAG_BIT, ID_INDEX_MASK, ID_VERSION, NULL},
    write::Result,
};

use super::Serializer;

/// Identifier state.
pub struct IdState {
    written_id: bool,
    ids: HashMap<String, u16>,
}

impl IdState {
    /// Create a new identifier state.
    pub fn new() -> Self {
        Self {
            written_id: false,
            ids: HashMap::new(),
        }
    }
}

impl Default for IdState {
    fn default() -> Self {
        Self::new()
    }
}

/// Can obtain a mutable reference to an identifier state.
pub trait IdStateMut {
    /// Obtain an immutable reference to an identifier state.
    fn borrow(&self) -> &IdState;

    /// Obtain a mutable reference to an identifier state.
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

impl<W: Write, I: IdStateMut, N> Serializer<W, I, N> {
    /// Write an identifier.
    pub fn id(&mut self, id: &str) -> Result {
        write_id_version(self)?;

        match self.id_state.borrow().ids.get(id) {
            None => {
                let index = self.id_state.borrow().ids.len() as u16 + 1;

                self.id_state.borrow_mut().ids.insert(id.to_owned(), index);

                self.u32(ID_FLAG_BIT)?;
                self.string(id)
            }
            Some(&index) => self.u32(ID_FLAG_BIT | ((index as u32) & ID_INDEX_MASK)),
        }
    }

    /// Write a null identifier.
    pub fn null_id(&mut self) -> Result {
        write_id_version(self)?;

        self.u32(NULL)
    }
}

fn write_id_version<W: Write, I: IdStateMut, N>(s: &mut Serializer<W, I, N>) -> Result {
    if !s.id_state.borrow().written_id {
        s.u32(ID_VERSION)?;

        s.id_state.borrow_mut().written_id = true;
    }

    Ok(())
}
