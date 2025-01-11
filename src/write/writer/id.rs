use std::{io::Write, sync::Arc};

use indexmap::{indexset, IndexSet};

use crate::{write::Error, ID_MARKER_BIT};

use super::Writer;

/// Identifier state.
pub struct IdState {
    seen_id: bool,
    ids: IndexSet<Arc<str>>,
}

impl IdState {
    pub fn new() -> Self {
        Self {
            seen_id: false,
            ids: indexset![],
        }
    }
}

impl Default for IdState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IdStateMut {
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

impl<W: Write, I: IdStateMut, N> Writer<W, I, N> {
    pub fn id_or_null(&mut self, id: Option<&Arc<str>>) -> Result<(), Error> {
        if !self.id_state.get_mut().seen_id {
            self.u32(3)?;

            self.id_state.get_mut().seen_id = true;
        }

        match id {
            Some(id) => match self.id_state.get_mut().ids.get_index_of(id) {
                Some(index) => {
                    self.u32(((index as u32) + 1) | ID_MARKER_BIT)?;
                }
                None => {
                    self.u32(ID_MARKER_BIT)?;
                    self.string(&id)?;

                    self.id_state.get_mut().ids.insert(Arc::clone(id));
                }
            },
            None => {
                self.u32(0xffffffff)?;
            }
        }

        Ok(())
    }

    pub fn id(&mut self, id: &Arc<str>) -> Result<(), Error> {
        self.id_or_null(Some(id))
    }
}
