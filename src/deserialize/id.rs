use std::{cell::Cell, io::Read, rc::Rc};

use elsa::FrozenVec;

use crate::{
    common::{ID_FLAG_BIT, ID_INDEX_MASK, ID_VERSION},
    read::Result,
};

use super::Deserializer;

/// Identifier state.
pub struct IdState {
    seen_id: Cell<bool>,
    ids: FrozenVec<Rc<str>>,
}

impl IdState {
    /// Create a new identifier state.
    pub fn new() -> Self {
        Self {
            seen_id: Cell::new(false),
            ids: FrozenVec::new(),
        }
    }
}

impl Default for IdState {
    fn default() -> Self {
        Self::new()
    }
}

/// Can provide an immutable reference to an identifier state.
pub trait IdStateRef {
    /// Obtain an immutable reference to an identifier state.
    fn borrow(&self) -> &IdState;
}

impl IdStateRef for IdState {
    fn borrow(&self) -> &IdState {
        self
    }
}

impl<T: IdStateRef> IdStateRef for &T {
    fn borrow(&self) -> &IdState {
        (**self).borrow()
    }
}

impl<R: Read, I: IdStateRef, N> Deserializer<R, I, N> {
    /// Read an identifier that is null.
    pub fn null_id(&mut self) -> Result<()> {
        let index = read_id_index(self)?;

        if index != 0xffffffff {
            return Err("expected null id".into());
        }

        Ok(())
    }

    /// Read an identifier that is not null.
    pub fn id(&mut self) -> Result<Rc<str>> {
        match self.id_or_null()? {
            None => Err("id is null".into()),
            Some(id) => Ok(id),
        }
    }

    /// Read an identifier that may be null.
    pub fn id_or_null(&mut self) -> Result<Option<Rc<str>>> {
        let index = read_id_index(self)?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index & !ID_INDEX_MASK == ID_FLAG_BIT {
            let index = (index & ID_INDEX_MASK) as u16;

            if index == 0 {
                let id = Rc::from(self.string()?);
                self.id_state.borrow().ids.push(Rc::clone(&id));

                Ok(Some(id))
            } else {
                let id = self
                    .id_state
                    .borrow()
                    .ids
                    .get_clone(index as usize - 1)
                    .ok_or("no id with given index")?;

                Ok(Some(id))
            }
        } else {
            Err("expected id".into())
        }
    }
}

fn read_id_index<R: Read, I: IdStateRef, N>(d: &mut Deserializer<R, I, N>) -> Result<u32> {
    if !d.id_state.borrow().seen_id.get() {
        if d.u32()? != ID_VERSION {
            return Err("invalid identifier version".into());
        }

        d.id_state.borrow().seen_id.set(true);
    }

    d.u32()
}
