//! Writing GameBox files.

mod error;
pub(crate) mod writable;
pub(crate) mod writer;

pub use error::Error;
pub(crate) use writable::{BodyChunk, BodyChunks};
pub(crate) use writer::Writer;
use writer::{write_to_buf, IdState, NodeState};

use std::{
    fs::File,
    io::{BufWriter, Seek, Write},
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
        let header_data = write_to_buf(
            |w| {
                let header_chunks = T::header_chunks();

                let mut chunk_bytes = vec![];

                let mut id_state = IdState::new();

                w.u32(header_chunks.len() as u32)?;

                for header_chunk in header_chunks {
                    let bytes =
                        write_to_buf(|w| (header_chunk.write_fn)(node, w), &mut id_state, ())?;

                    let mut len = bytes.len() as u32;

                    if header_chunk.is_heavy {
                        len |= HEAVY_CHUNK_MARKER_BIT;
                    }

                    w.u32(T::CLASS_ID | header_chunk.num as u32)?;
                    w.u32(len)?;

                    chunk_bytes.push(bytes);
                }

                for bytes in chunk_bytes {
                    w.bytes(&bytes)?;
                }

                Ok(())
            },
            (),
            (),
        )?;
        w.byte_buf(&header_data)?;

        let mut node_state = NodeState::new();
        let body = write_to_buf(|w| node.write_body(w), IdState::new(), &mut node_state)?;

        w.u32(node_state.num_nodes() as u32 + 1)?;
        w.u32(0)?;

        match self.body_compression {
            Compression::None => {
                w.bytes(&body)?;
            }
            Compression::Compress { level } => {
                let compressed_body = lzo1x::compress(&body, level);

                w.u32(body.len() as u32)?;
                w.byte_buf(&compressed_body)?;
            }
        }

        Ok(())
    }

    /// Write the given `node` to a file at the given `path`.
    pub fn write_file<T: Writable>(&self, node: &T, path: impl AsRef<Path>) -> Result<(), Error> {
        let file = File::create(path).map_err(Error::io)?;
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
