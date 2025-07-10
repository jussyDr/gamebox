use std::sync::Arc;

use crate::{
    ID_MARKER_BIT, NULL,
    read::{Error, error_unknown_version, reader::Reader},
};

/// Identifier table.
pub struct IdTable {
    seen_id: bool,
    ids: Vec<Arc<str>>,
}

impl IdTable {
    /// Create a new `IdTable`.
    pub fn new() -> Self {
        Self {
            seen_id: false,
            ids: vec![],
        }
    }
}

impl Default for IdTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Try from id.
pub trait TryFromId {
    /// Try from id.
    fn try_from_id(id: Option<Arc<str>>) -> Result<Self, Error>
    where
        Self: Sized;
}

pub fn read_id<T: TryFromId>(r: &mut impl Reader, id_table: &mut IdTable) -> Result<T, Error> {
    let id = read_id_or_null(r, id_table)?;

    T::try_from_id(id)
}

fn read_id_or_null(r: &mut impl Reader, id_table: &mut IdTable) -> Result<Option<Arc<str>>, Error> {
    if !id_table.seen_id {
        let version = r.u32()?;

        if version != 3 {
            return Err(error_unknown_version("identifier", version));
        }

        id_table.seen_id = true;
    }

    let index = r.u32()?;

    if index == NULL {
        return Ok(None);
    }

    if index == 0x0000001a {
        // Not sure what this is yet.
        return Ok(Some(Arc::from("")));
    }

    if index == 0x00002713 {
        // Not sure what this is yet.
        return Ok(Some(Arc::from("")));
    }

    if index & ID_MARKER_BIT == 0 {
        return Err(Error::new("expected an identifier"));
    }

    let index = index & 0x37ffffff;

    match index.checked_sub(1) {
        None => {
            let id = Arc::from(r.string()?);
            id_table.ids.push(Arc::clone(&id));

            Ok(Some(id))
        }
        Some(index) => {
            let id = id_table
                .ids
                .get(index as usize)
                .ok_or_else(|| Error::new("no identifier for the given index"))?;

            Ok(Some(Arc::clone(id)))
        }
    }
}

impl TryFromId for Arc<str> {
    fn try_from_id(id: Option<Arc<str>>) -> Result<Self, Error> {
        match id {
            Some(id) => Ok(id),
            None => Err(Error::new("expected a non-null identifier")),
        }
    }
}

impl TryFromId for Option<Arc<str>> {
    fn try_from_id(id: Option<Arc<str>>) -> Result<Self, Error> {
        Ok(id)
    }
}
