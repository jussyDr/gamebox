//! Writing GameBox files.

pub(crate) mod writable;
pub(crate) mod writer;

pub(crate) use writable::{BodyChunk, BodyChunks};
pub(crate) use writer::Writer;
use writer::{IdState, NodeState};

use std::{
    fs::File,
    io::{BufWriter, Cursor, Error, Seek, Write},
    path::Path,
};

use lzo1x::CompressLevel;

use crate::{FILE_SIGNATURE, HEAVY_CHUNK_MARKER_BIT};

/// A writable class.
pub trait Writable: writable::Sealed {}

/// Compression.
pub enum Compression {
    /// No compression.
    None,
    /// Compression.
    Compress {
        /// Compression level.
        level: CompressLevel,
    },
}

/// Write settings.
pub struct Settings {
    body_compression: Compression,
}

impl Settings {
    /// New.
    pub const fn new() -> Self {
        Self {
            body_compression: Compression::Compress {
                level: CompressLevel::new(3),
            },
        }
    }

    /// Body compression.
    pub const fn body_compression(self, body_compression: Compression) -> Self {
        Self { body_compression }
    }

    /// Write the given `node` to the given `writer`.
    pub fn write<T: Writable>(&self, node: &T, writer: impl Write + Seek) -> Result<(), Error> {
        let mut w = Writer::new(writer, (), ());

        w.bytes(&FILE_SIGNATURE)?;
        w.u16(6)?;
        w.u8(b'B')?;
        w.u8(b'U')?;

        match self.body_compression {
            Compression::None => w.u8(b'U')?,
            Compression::Compress { .. } => w.u8(b'C')?,
        }

        w.u8(b'R')?;
        w.u32(T::CLASS_ID)?;
        w.byte_buf_inline(|w| {
            let header_chunks = T::header_chunks();

            let mut hh = vec![];

            let mut id_state = IdState::new();

            for header_chunk in header_chunks {
                let mut w2 = Writer::new(vec![], &mut id_state, ());
                (header_chunk.write_fn)(node, &mut w2)?;
                let buf = w2.into_inner();

                let mut len = buf.len() as u32;

                if header_chunk.heavy {
                    len |= HEAVY_CHUNK_MARKER_BIT;
                }

                w.u32(T::CLASS_ID | header_chunk.num as u32)?;
                w.u32(len)?;

                hh.push(buf);
            }

            for h in hh {
                w.bytes(&h)?;
            }

            Ok(())
        })?;
        w.u32(1)?;
        w.u32(0)?;

        match self.body_compression {
            Compression::None => {
                let mut w = Writer::new(w.into_inner(), IdState::new(), NodeState::new());

                node.write_body(&mut w)?;
            }
            Compression::Compress { level } => {
                let body = {
                    let mut body = vec![];
                    let mut w =
                        Writer::new(Cursor::new(&mut body), IdState::new(), NodeState::new());

                    node.write_body(&mut w)?;

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
    pub fn write_file<T: Writable>(&self, node: &T, path: impl AsRef<Path>) -> Result<(), Error> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        self.write(node, writer)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

/// Write the given `node` to the given `writer`.
pub fn write<T: Writable>(node: &T, writer: impl Write + Seek) -> Result<(), Error> {
    Settings::default().write(node, writer)
}

/// Write the given `node` to a file at the given `path`.
pub fn write_file<T: Writable>(node: &T, path: impl AsRef<Path>) -> Result<(), Error> {
    Settings::default().write_file(node, path)
}
