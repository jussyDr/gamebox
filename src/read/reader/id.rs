use std::{io::Read, sync::Arc};

use crate::{read::Error, Ident};

use super::Reader;

/// Identifier state.
pub struct IdState {
    seen_id: bool,
    ids: Vec<Arc<str>>,
}

impl IdState {
    /// Create a new [IdState].
    pub const fn new() -> Self {
        Self {
            seen_id: false,
            ids: vec![],
        }
    }
}

impl Default for IdState {
    fn default() -> Self {
        Self::new()
    }
}

/// Allows to get a reference to an [IdState].
pub trait IdStateRef {
    /// Get a reference to an [IdState].
    fn get(&self) -> &IdState;
}

impl IdStateRef for IdState {
    fn get(&self) -> &IdState {
        self
    }
}

impl IdStateRef for &mut IdState {
    fn get(&self) -> &IdState {
        self
    }
}

/// Allows to get a mutable reference to an [IdState].
pub trait IdStateMut: IdStateRef {
    /// Get a mutable reference to an [IdState].
    fn get_mut(&mut self) -> &mut IdState;
}

impl IdStateMut for IdState {
    fn get_mut(&mut self) -> &mut IdState {
        self
    }
}

impl IdStateMut for &mut IdState {
    fn get_mut(&mut self) -> &mut IdState {
        self
    }
}

impl<R: Read, I: IdStateRef, N> Reader<R, I, N> {
    /// Read a identifier.
    pub fn id_ref(&mut self) -> Result<Option<Arc<str>>, Error> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index & 0x40000000 == 0 {
            return Err(Error);
        }

        let index = index & 0x3fffffff;

        if index == 0 {
            todo!()
        }

        let index = index - 1;

        let id = self.id_state.get().ids.get(index as usize).ok_or(Error)?;

        Ok(Some(Arc::clone(id)))
    }
}

impl<R: Read, I: IdStateMut, N> Reader<R, I, N> {
    /// Read a identifier.
    pub fn id(&mut self) -> Result<Option<Arc<str>>, Error> {
        if !self.id_state.get().seen_id {
            let version = self.u32()?;

            if version != 3 {
                return Err(Error);
            }

            self.id_state.get_mut().seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index & 0x40000000 == 0 {
            return Err(Error);
        }

        let index = index & 0x3fffffff;

        let id = if index == 0 {
            let id = Arc::from(self.string()?);

            self.id_state.get_mut().ids.push(Arc::clone(&id));

            id
        } else {
            let index = index - 1;

            let id = self.id_state.get().ids.get(index as usize).ok_or(Error)?;

            Arc::clone(id)
        };

        Ok(Some(id))
    }

    /// Read a non null identifier.
    pub fn id_non_null(&mut self) -> Result<Arc<str>, Error> {
        match self.id()? {
            None => Err(Error),
            Some(id) => Ok(id),
        }
    }

    /// Read a identifier triple.
    pub fn ident(&mut self) -> Result<Ident, Error> {
        let id = self.id()?;

        let collection = match self.u32()? {
            0xffffffff => None,
            26 => Some(()),
            0x00002713 => Some(()),
            _ => return Err(Error),
        };

        let author = self.id()?;

        Ok(Ident {
            id,
            collection,
            author,
        })
    }
}
