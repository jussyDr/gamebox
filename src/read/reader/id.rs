use std::{io::Read, sync::Arc};

use crate::read::{Error, error_unknown_version, reader::Reader};

/// Identifier table.
pub struct IdTable {
    seen_id: bool,
    ids: Vec<Arc<str>>,
}

impl IdTable {
    pub fn new() -> Self {
        Self {
            seen_id: false,
            ids: vec![],
        }
    }
}

impl AsMut<IdTable> for IdTable {
    fn as_mut(&mut self) -> &mut IdTable {
        self
    }
}

impl Default for IdTable {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IdTableRef: AsMut<IdTable> {}

impl<T: AsMut<IdTable>> IdTableRef for T {}

impl<R: Read, I: IdTableRef, N> Reader<R, I, N> {
    pub fn id_or_null(&mut self) -> Result<Option<Arc<str>>, Error> {
        if !self.id_table.as_mut().seen_id {
            let version = self.u32()?;

            if version != 3 {
                return Err(error_unknown_version("identifier", version));
            }

            self.id_table.as_mut().seen_id = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index & 0x40000000 == 0 {
            return Err(Error::new("expected an identifier"));
        }

        let index = index & 0x37ffffff;

        match index.checked_sub(1) {
            None => {
                let id = Arc::from(self.string()?);
                self.id_table.as_mut().ids.push(Arc::clone(&id));

                Ok(Some(id))
            }
            Some(index) => {
                let id = self
                    .id_table
                    .as_mut()
                    .ids
                    .get(index as usize)
                    .ok_or(Error::new(""))?;

                Ok(Some(Arc::clone(id)))
            }
        }
    }

    pub fn id(&mut self) -> Result<Arc<str>, Error> {
        match self.id_or_null()? {
            None => Err(Error::new("expected a non-null identifier")),
            Some(id) => Ok(id),
        }
    }
}
