use std::{io, sync::Arc};

use crate::read::{Result, reader::Reader};

pub trait HeaderReader: Reader {
    fn string_ref(&mut self) -> Result<Option<Arc<str>>>;
}

pub struct HeaderReaderImpl<R> {
    inner: R,
    string_refs: Vec<Arc<str>>,
    seen_string_ref: bool,
}

impl<R: io::Read> io::Read for HeaderReaderImpl<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::Read> HeaderReader for HeaderReaderImpl<R> {
    fn string_ref(&mut self) -> Result<Option<Arc<str>>> {
        todo!()
    }
}
