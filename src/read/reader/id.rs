use std::{io::Read, sync::Arc};

use crate::{
    read::{Error, ErrorKind},
    ID_MARKER_BIT,
};

use super::Reader;

/// Identifier state.
pub struct IdState {
    seen_id: bool,
    ids: Vec<Arc<str>>,
}

impl IdState {
    /// Create a new `IdState`.
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

/// A mutable reference to an `IdState`.
pub trait IdStateMut {
    /// Obtain a mutable reference to an `IdState`.
    fn get_mut(&mut self) -> &mut IdState;
}

impl IdStateMut for IdState {
    fn get_mut(&mut self) -> &mut IdState {
        self
    }
}

impl<T: IdStateMut> IdStateMut for &mut T {
    fn get_mut(&mut self) -> &mut IdState {
        (**self).get_mut()
    }
}

impl<R: Read, I: IdStateMut, N> Reader<R, I, N> {
    /// Read an identifier that may be null.
    pub fn id_or_null(&mut self) -> Result<Option<Arc<str>>, Error> {
        if !self.id_state.get_mut().seen_id {
            let version = self.u32()?;

            if version != 3 {
                return Err(Error::version("identifier", version));
            }

            self.id_state.get_mut().seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index == 0x00000019 {
            return Ok(None);
        }

        if index == 0x0000001a {
            return Ok(None);
        }

        if index == 0x00002713 {
            return Ok(None);
        }

        if index & ID_MARKER_BIT == 0 {
            return Err(Error::new(ErrorKind::Format("expected identifier".into())));
        }

        let index = index & 0x3fffffff;

        let id = match index.checked_sub(1) {
            Some(index) => {
                let id = self
                    .id_state
                    .get_mut()
                    .ids
                    .get(index as usize)
                    .ok_or_else(|| Error::new(ErrorKind::Format("identifier index".into())))?;

                Arc::clone(id)
            }
            None => {
                let id = Arc::from(self.string()?);
                self.id_state.get_mut().ids.push(Arc::clone(&id));

                id
            }
        };

        Ok(Some(id))
    }

    /// Read an identifier.
    pub fn id(&mut self) -> Result<Arc<str>, Error> {
        match self.id_or_null()? {
            Some(id) => Ok(id),
            None => Err(Error::null("identifier")),
        }
    }
}
