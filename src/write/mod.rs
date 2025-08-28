//! Writing GameBox files.

mod writer;
pub(crate) use writer::{BodyWriter, HeaderWriter};
use writer::{BodyWriterImpl, Writer};

use lzo1x::CompressLevel;

use std::{
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

use crate::{FILE_SIGNATURE, write::writer::HeaderWriterImpl};

pub fn write_file<T: Write>(node: &T, path: impl AsRef<Path>) -> io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    write(node, writer)
}

pub fn write<T: Write>(node: &T, writer: impl io::Write) -> io::Result<()> {
    let mut w = writer;

    w.write_all(&FILE_SIGNATURE)?;
    w.u16(6)?;
    w.u8(b'B')?;
    w.u8(b'U')?;
    w.u8(b'C')?;
    w.u8(b'R')?;
    w.u32(T::CLASS_ID)?;

    let mut header_data = vec![];
    let mut header_writer = HeaderWriterImpl::new(&mut header_data);
    node.write_header(&mut header_writer)?;

    w.list_u8(&header_data)?;

    let mut body = vec![];
    let mut body_writer = BodyWriterImpl::new(&mut body);
    node.write_body(&mut body_writer)?;

    w.u32(body_writer.num_node_refs() as u32 + 1)?;
    w.u32(0)?;

    let compressed_body = lzo1x::compress(&body, CompressLevel::default());

    w.u32(body.len() as u32)?;
    w.list_u8(&compressed_body)?;

    Ok(())
}

pub trait Write: sealed::Write {}

pub(crate) mod sealed {
    use std::io;

    use crate::write::{BodyWriter, HeaderWriter};

    pub trait Write {
        const CLASS_ID: u32;

        fn write_header(&self, w: &mut impl HeaderWriter) -> io::Result<()>;

        fn write_body(&self, w: &mut impl BodyWriter) -> io::Result<()>;
    }
}
