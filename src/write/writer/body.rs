use std::{any::Any, collections::HashSet, io, sync::Arc};

use crate::write::HeaderWriter;

pub trait BodyWriter: HeaderWriter {}

pub struct BodyWriterImpl<W> {
    inner: W,
    string_refs: HashSet<Arc<str>>,
    node_refs: HashSet<Arc<dyn Any>>,
}

impl<W> BodyWriterImpl<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            string_refs: HashSet::new(),
            node_refs: HashSet::new(),
        }
    }

    pub fn num_node_refs(&self) -> usize {
        self.node_refs.len()
    }
}

impl<W: io::Write> io::Write for BodyWriterImpl<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<W: io::Write> HeaderWriter for BodyWriterImpl<W> {}

impl<W: io::Write> BodyWriter for BodyWriterImpl<W> {}
