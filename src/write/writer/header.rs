use std::{collections::HashSet, io, sync::Arc};

use super::Writer;

pub trait HeaderWriter: Writer {}

pub struct HeaderWriterImpl<W> {
    inner: W,
    string_refs: HashSet<Arc<str>>,
}

impl<W> HeaderWriterImpl<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            string_refs: HashSet::new(),
        }
    }
}

impl<W: io::Write> io::Write for HeaderWriterImpl<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<W: io::Write> HeaderWriter for HeaderWriterImpl<W> {}
