use std::{any::Any, io, sync::Arc};

use crate::read::{
    Error, Result,
    reader::{HeaderReader, Reader},
};

pub trait BodyReader: HeaderReader {
    fn node<T: ReadNode>(&mut self) -> Result<T>;

    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<Option<T>>;
}

pub struct BodyReaderImpl<R> {
    inner: R,
    string_refs: Vec<Arc<str>>,
    seen_string_ref: bool,
    node_refs: Box<[Option<Arc<dyn Any + Send + Sync>>]>,
}

impl<R> BodyReaderImpl<R> {
    pub fn new(inner: R, node_refs: Box<[Option<Arc<dyn Any + Send + Sync>>]>) -> Self {
        Self {
            inner,
            string_refs: vec![],
            seen_string_ref: false,
            node_refs,
        }
    }
}

impl<R: io::Read> io::Read for BodyReaderImpl<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::Read> HeaderReader for BodyReaderImpl<R> {
    fn string_ref(&mut self) -> Result<Option<Arc<str>>> {
        if !self.seen_string_ref {
            if self.u32()? != 3 {
                return Err(Error::Internal("unknown string reference version".into()));
            }

            self.seen_string_ref = true;
        }

        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        if index == 0x0000001a {
            // TODO: what is this?
            return Ok(Some(Arc::default()));
        }

        if index & 0xc0000000 != 0x40000000 {
            return Err(Error::Internal("expected a string reference".into()));
        }

        let index = index & 0x3fffffff;

        match index.checked_sub(1) {
            None => {
                let string_ref = Arc::from(self.string()?);
                self.string_refs.push(Arc::clone(&string_ref));

                Ok(Some(string_ref))
            }
            Some(index) => {
                let string_ref = self.string_refs.get(index as usize).ok_or_else(|| {
                    Error::Internal("string reference index out of bounds".into())
                })?;

                Ok(Some(Arc::clone(string_ref)))
            }
        }
    }
}

impl<R: io::Read> BodyReader for BodyReaderImpl<R> {
    fn node<T: ReadNode>(&mut self) -> Result<T> {
        todo!()
    }

    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<Option<T>> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return Ok(None);
        }

        let index = index
            .checked_sub(1)
            .ok_or_else(|| Error::Internal("node reference index is zero".into()))?;

        let entry = self
            .node_refs
            .get(index as usize)
            .ok_or_else(|| Error::Internal("node reference index out of bounds".into()))?;

        let node_ref = match entry {
            None => {
                if self.u32()? != T::CLASS_ID {
                    return Err(Error::Internal("class id mismatch".into()));
                }

                T::read_node_ref(self)?
            }
            Some(node_ref) => Arc::clone(node_ref),
        };

        let node_ref = T::from_any(node_ref)?;

        Ok(Some(node_ref))
    }
}

pub trait ReadNodeRef: Sized {
    const CLASS_ID: u32;

    fn read_node_ref(r: &mut impl BodyReader) -> Result<Arc<dyn Any + Send + Sync>>;

    fn from_any(node_ref: Arc<dyn Any + Send + Sync>) -> Result<Self>;
}

impl<T: 'static + Send + Sync + ReadNode> ReadNodeRef for Arc<T> {
    const CLASS_ID: u32 = T::CLASS_ID;

    fn read_node_ref(r: &mut impl BodyReader) -> Result<Arc<dyn Any + Send + Sync>> {
        Ok(Arc::new(T::read_node(r)?))
    }

    fn from_any(node_ref: Arc<dyn Any + Send + Sync>) -> Result<Self> {
        node_ref
            .downcast()
            .map_err(|_| Error::Internal("failed to downcast node reference".into()))
    }
}

pub trait ReadNode: Sized {
    const CLASS_ID: u32;

    fn read_node(r: &mut impl BodyReader) -> Result<Self>;
}
