//! Writing GameBox files.

pub mod writer;

pub use writer::Writer;

pub(crate) mod writable;

pub(crate) use writable::{write_body, BodyChunk, BodyChunks};

use std::{
    fs::File,
    io::{BufWriter, Cursor, Error, Seek, Write},
    path::Path,
};

use lzo1x::CompressLevel;

use crate::FILE_SIGNATURE;

/// A writable class.
pub trait Writable: writable::Sealed {}

pub enum Compression {
    None,
    Compress { level: CompressLevel },
}

/// Write the given `node` to the given `writer`.
pub fn write<T: Writable>(node: &T, writer: impl Write + Seek) -> Result<(), Error> {
    let compression = Compression::Compress {
        level: CompressLevel::default(),
    };

    let mut w = Writer::new(writer, (), ());

    w.bytes(&FILE_SIGNATURE)?;
    w.u16(6)?;
    w.u8(b'B')?;
    w.u8(b'U')?;

    match compression {
        Compression::None => w.u8(b'U')?,
        Compression::Compress { .. } => w.u8(b'C')?,
    }

    w.u8(b'R')?;
    w.u32(T::CLASS_ID)?;
    w.u32(0)?;
    w.u32(1)?;
    w.u32(0)?;

    match compression {
        Compression::None => {
            write_body(&mut w, node)?;
        }
        Compression::Compress { level } => {
            let body = {
                let mut body = vec![];
                let mut w = Writer::new(Cursor::new(&mut body), (), ());

                write_body(&mut w, node)?;

                body
            };

            let compressed_body = lzo1x::compress(&body, level);

            w.u32(body.len() as u32)?;
            w.u32(compressed_body.len() as u32)?;
            w.bytes(&compressed_body)?;
        }
    }

    Ok(())
}

/// Write the given `node` to a file at the given `path`.
pub fn write_file<T: Writable>(node: &T, path: impl AsRef<Path>) -> Result<(), Error> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    write(node, writer)
}
