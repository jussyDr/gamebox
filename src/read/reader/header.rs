use std::{io::Read, sync::Arc};

use crate::read::{
    Error, error_unknown_version,
    reader::{IdTable, Reader},
};

/// Try from id.
pub trait TryFromId {
    /// Try from id.
    fn try_from_id(id: Option<Arc<str>>) -> Result<Self, Error>
    where
        Self: Sized;
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

/// Header reader.
pub trait HeaderReader: Reader {
    /// Id table.
    fn id_table(&mut self) -> &mut IdTable;

    /// Read an identifier.
    fn id<T: TryFromId>(&mut self) -> Result<T, Error>
    where
        Self: Sized,
    {
        let id = id_or_null(self)?;

        T::try_from_id(id)
    }
}

fn id_or_null(r: &mut impl HeaderReader) -> Result<Option<Arc<str>>, Error> {
    if !r.id_table().seen_id {
        let version = r.u32()?;

        if version != 3 {
            return Err(error_unknown_version("identifier", version));
        }

        r.id_table().seen_id = true;
    }

    let index = r.u32()?;

    if index == 0xffffffff {
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

    if index & 0x40000000 == 0 {
        return Err(Error::new("expected an identifier"));
    }

    let index = index & 0x37ffffff;

    match index.checked_sub(1) {
        None => {
            let id = Arc::from(r.string()?);
            r.id_table().ids.push(Arc::clone(&id));

            Ok(Some(id))
        }
        Some(index) => {
            let id = r
                .id_table()
                .ids
                .get(index as usize)
                .ok_or_else(|| Error::new(""))?;

            Ok(Some(Arc::clone(id)))
        }
    }
}

/// Header reader.
pub struct HeaderReaderImpl<R, I> {
    /// Reader.
    pub reader: R,
    /// Id table.
    pub id_table: I,
}

impl<R: Read, I> Read for HeaderReaderImpl<R, I> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read, I: AsMut<IdTable>> HeaderReader for HeaderReaderImpl<R, I> {
    fn id_table(&mut self) -> &mut IdTable {
        self.id_table.as_mut()
    }
}
