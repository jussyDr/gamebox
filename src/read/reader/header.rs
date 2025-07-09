use std::io::Read;

use crate::read::{
    Error, IdTable,
    id::{TryFromId, read_id},
    reader::Reader,
};

/// Header reader.
pub trait HeaderReader: Reader {
    /// Read an identifier.
    fn id<T: TryFromId>(&mut self) -> Result<T, Error>
    where
        Self: Sized;
}

/// Header reader.
pub struct HeaderReaderImpl<R> {
    /// Reader.
    pub reader: R,
    /// Id table.
    pub id_table: IdTable,
}

impl<R: Read> Read for HeaderReaderImpl<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read> HeaderReader for HeaderReaderImpl<R> {
    /// Read an identifier.
    fn id<T: TryFromId>(&mut self) -> Result<T, Error> {
        read_id(&mut self.reader, &mut self.id_table)
    }
}
