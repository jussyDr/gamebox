use std::{io::Read, rc::Rc};

use crate::{
    common::{ID_FLAG_BIT, ID_INDEX_MASK, ID_VERSION},
    read::Result,
};

use super::Deserializer;

/// Identifier state.
pub struct IdState {
    seen_id: bool,
    ids: Vec<Rc<str>>,
}

impl IdState {
    /// Create a new identifier state.
    pub fn new() -> Self {
        Self {
            seen_id: false,
            ids: Vec::new(),
        }
    }
}

impl Default for IdState {
    fn default() -> Self {
        Self::new()
    }
}

// impl Borrow<IdState> for IdState {
//     fn borrow(&self) -> &IdState {
//         self
//     }
// }

// impl BorrowMut<IdState> for IdState {
//     fn borrow_mut(&mut self) -> &mut IdState {
//         self
//     }
// }

// impl Borrow<IdState> for &IdState {
//     fn borrow(&self) -> &IdState {
//         (**self).borrow()
//     }
// }

// impl BorrowMut<IdState> for &mut IdState {
//     fn borrow_mut(&mut self) -> &mut IdState {
//         (**self).borrow_mut()
//     }
// }

/// Can provide a mutable reference to an identifier state.
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

impl<R: Read, I: IdStateMut, N> Deserializer<R, I, N> {
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
                self.id_state.borrow_mut().ids.push(Rc::clone(&id));

                Ok(Some(id))
            } else {
                let id = self
                    .id_state
                    .borrow()
                    .ids
                    .get(index as usize - 1)
                    .ok_or("no id with given index")?;

                Ok(Some(Rc::clone(id)))
            }
        } else {
            Err("expected id".into())
        }
    }
}

fn read_id_index<R: Read, I: IdStateMut, N>(d: &mut Deserializer<R, I, N>) -> Result<u32> {
    if !d.id_state.borrow().seen_id {
        if d.u32()? != ID_VERSION {
            return Err("invalid identifier version".into());
        }

        d.id_state.borrow_mut().seen_id = true;
    }

    d.u32()
}
