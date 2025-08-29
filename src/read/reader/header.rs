use std::{io, sync::Arc};

use crate::read::{Error, Result, reader::Reader};

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

impl<R> HeaderReaderImpl<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            string_refs: vec![],
            seen_string_ref: false,
        }
    }
}

impl<R: io::Read> io::Read for HeaderReaderImpl<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::Read> HeaderReader for HeaderReaderImpl<R> {
    fn string_ref<T: ReadStringRef>(&mut self) -> Result<T> {
        if !self.seen_string_ref {
            if self.u32()? != 3 {
                return Err(Error::Internal("unknown string reference version".into()));
            }

            self.seen_string_ref = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return T::from_option(None);
        }

        if index == 0x0000001a {
            // TODO: what is this?
            return T::from_option(Some(Arc::default()));
        }

        if index == 0x00002713 {
            // TODO: what is this?
            return T::from_option(Some(Arc::default()));
        }

        if index & 0xc0000000 != 0x40000000 {
            return Err(Error::Internal("expected a string reference".into()));
        }

        let index = index & 0x3fffffff;

        match index.checked_sub(1) {
            None => {
                let string_ref = Arc::from(self.string()?);
                self.string_refs.push(Arc::clone(&string_ref));

                T::from_option(Some(string_ref))
            }
            Some(index) => {
                let string_ref = self.string_refs.get(index as usize).ok_or_else(|| {
                    Error::Internal("string reference index out of bounds".into())
                })?;

                T::from_option(Some(Arc::clone(string_ref)))
            }
        }
    }
}
