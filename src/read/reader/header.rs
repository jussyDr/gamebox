use std::{io, sync::Arc};

use crate::read::{Result, reader::Reader};

pub trait HeaderReader: Reader {
    fn string_ref<T: ReadStringRef>(&mut self) -> Result<T>;
}

pub trait ReadStringRef: Sized {
    fn from_option(string_ref: Option<Arc<str>>) -> Result<Self>;
}

impl ReadStringRef for Arc<str> {
    fn from_option(string_ref: Option<Arc<str>>) -> Result<Self> {
        match string_ref {
            None => todo!(),
            Some(string_ref) => Ok(string_ref),
        }
    }
}

impl ReadStringRef for Option<Arc<str>> {
    fn from_option(string_ref: Option<Arc<str>>) -> Result<Self> {
        Ok(string_ref)
    }
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
    fn string_ref<T: ReadStringRef>(&mut self) -> Result<T> {
        todo!()
    }
}
