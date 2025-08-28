use std::{any::Any, io, sync::Arc};

use crate::read::{
    Error, Result,
    reader::{HeaderReader, Reader, header::ReadStringRef},
};

pub trait BodyReader: HeaderReader {
    fn node<T: ReadNode>(&mut self) -> Result<T>;

    fn node_with_id<T: ReadNode>(&mut self, class_id: u32) -> Result<T>;

    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<T>;
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

impl<R: io::Read> BodyReader for BodyReaderImpl<R> {
    fn node<T: ReadNode>(&mut self) -> Result<T> {
        let class_id = self.u32()?;

        self.node_with_id(class_id)
    }

    fn node_with_id<T: ReadNode>(&mut self, class_id: u32) -> Result<T> {
        if class_id != T::CLASS_ID {
            return Err(Error::Internal("class id mismatch".into()));
        }

        T::read_node(self)
    }

    fn node_ref<T: ReadNodeRef>(&mut self) -> Result<T> {
        let index = self.u32()?;

        if index == 0xffffffff {
            return T::from_option_any(None);
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
                let class_id = self.u32()?;

                T::read_node_ref(self, class_id)?
            }
            Some(node_ref) => Arc::clone(node_ref),
        };

        let node_ref = T::from_option_any(Some(node_ref))?;

        Ok(node_ref)
    }
}

pub trait ReadNodeRef: Sized {
    fn read_node_ref(r: &mut impl BodyReader, class_id: u32) -> Result<Arc<dyn Any + Send + Sync>>;

    fn from_option_any(node_ref: Option<Arc<dyn Any + Send + Sync>>) -> Result<Self>;
}

impl<T: 'static + Send + Sync + ReadNode> ReadNodeRef for Arc<T> {
    fn read_node_ref(r: &mut impl BodyReader, class_id: u32) -> Result<Arc<dyn Any + Send + Sync>> {
        let node = r.node_with_id::<T>(class_id)?;

        Ok(Arc::new(node))
    }

    fn from_option_any(node_ref: Option<Arc<dyn Any + Send + Sync>>) -> Result<Self> {
        match node_ref {
            None => todo!(),
            Some(node_ref) => node_ref
                .downcast()
                .map_err(|_| Error::Internal("failed to downcast node reference".into())),
        }
    }
}

impl<T: 'static + Send + Sync + ReadNode> ReadNodeRef for Option<Arc<T>> {
    fn read_node_ref(r: &mut impl BodyReader, class_id: u32) -> Result<Arc<dyn Any + Send + Sync>> {
        let node = r.node_with_id::<T>(class_id)?;

        Ok(Arc::new(node))
    }

    fn from_option_any(node_ref: Option<Arc<dyn Any + Send + Sync>>) -> Result<Self> {
        match node_ref {
            None => Ok(None),
            Some(node_ref) => {
                let node_ref = node_ref
                    .downcast()
                    .map_err(|_| Error::Internal("failed to downcast node reference".into()))?;

                Ok(Some(node_ref))
            }
        }
    }
}

pub trait ReadNode: Sized {
    const CLASS_ID: u32;

    fn read_node(r: &mut impl BodyReader) -> Result<Self>;
}
