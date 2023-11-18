//! Types for reading GameBox nodes.

use std::io::{self, Cursor, Read, Seek};

use crate::deserializer::{Deserializer, IdState, NodeState};

/// Error while reading a GameBox node.
#[derive(Debug)]
pub enum Error {
    /// An I/O error.
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(io_err: io::Error) -> Self {
        Self::Io(io_err)
    }
}

/// Result type used when reading GameBox nodes.
pub type Result<T> = std::result::Result<T, Error>;

/// Trait which indicates that a certain class is readable.
pub trait Readable: private::Readable {}

/// Read a node of class `T` from the given `reader`.
pub fn read<T: Readable>(reader: impl Read + Seek) -> Result<T> {
    let mut node = T::default();

    let mut d = Deserializer::new(reader, (), ());

    if d.byte_array()? != [b'G', b'B', b'X'] {
        todo!()
    }

    if d.u16()? != 6 {
        todo!()
    }

    if d.u8()? != b'B' {
        todo!()
    }

    if d.u8()? != b'U' {
        todo!()
    }

    if d.u8()? != b'C' {
        todo!()
    }

    if d.u8()? != b'R' {
        todo!()
    }

    let class_id = d.u32()?;

    if class_id != T::CLASS_ID {
        todo!()
    }

    let user_data_size = d.u32()?;

    {
        let mut d = d.take(user_data_size as u64, (), ());

        let header_chunks = d.list(|d| {
            let chunk_id = d.u32()?;
            let chunk_size = d.u32()?;

            Ok((chunk_id, chunk_size))
        })?;

        let mut id_state = IdState::default();

        let mut header_chunk_entries = T::header_chunk_table().into_iter();

        for (chunk_id, chunk_size) in header_chunks {
            let mut d = d.take(chunk_size as u64, &mut id_state, ());

            let header_chunk_entry = header_chunk_entries
                .find(|header_chunk_entry| header_chunk_entry.id == chunk_id)
                .unwrap();

            (header_chunk_entry.read_fn)(&mut node, &mut d)?;

            d.end()?;
        }

        d.end()?;
    }

    let num_nodes = d.u32()?;

    let num_node_refs = d.u32()?;

    if num_node_refs > 0 {
        todo!()
    }

    let body_size = d.u32()?;
    let compressed_body_size = d.u32()?;
    let compressed_body = d.bytes(compressed_body_size as usize)?;
    let mut buf = vec![0; body_size as usize];

    let body = lzo1x_1::decompress_to_slice(&compressed_body, &mut buf).unwrap();
    let reader = Cursor::new(body);

    let mut d = Deserializer::new(reader, IdState::default(), NodeState::new(num_nodes));

    let mut body_chunk_entries = T::body_chunk_table().into_iter();

    loop {
        let chunk_id = d.u32()?;

        if chunk_id == 0xfacade01 {
            break;
        }

        let body_chunk_entry = body_chunk_entries
            .find(|body_chunk_entry| body_chunk_entry.id == chunk_id)
            .unwrap();

        (body_chunk_entry.read_fn)(&mut node, &mut d)?;
    }

    d.end()?;

    Ok(node)
}

pub(crate) mod private {
    use std::io::{Read, Take};

    use crate::deserializer::{Deserializer, IdState, NodeState};

    use super::Result;

    pub struct HeaderChunkEntry<T, R> {
        pub id: u32,
        pub read_fn: HeaderChunkReadFn<T, R>,
    }
    type HeaderChunkReadFn<T, R> =
        fn(n: &mut T, d: &mut Deserializer<Take<&mut R>, &mut IdState, ()>) -> Result<()>;

    pub struct BodyChunkEntry<T, R> {
        pub id: u32,
        pub read_fn: BodyChunkReadFn<T, R>,
    }

    type BodyChunkReadFn<T, R> =
        fn(n: &mut T, d: &mut Deserializer<R, IdState, NodeState>) -> Result<()>;

    pub trait Readable {
        const CLASS_ID: u32;

        fn default() -> Self;

        fn header_chunk_table<R: Read>() -> Vec<HeaderChunkEntry<Self, R>>
        where
            Self: Sized;

        fn body_chunk_table<R: Read>() -> Vec<BodyChunkEntry<Self, R>>
        where
            Self: Sized;
    }
}

impl<T: private::Readable> Readable for T {}
