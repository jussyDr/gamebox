use std::io::Write;

use lzo::lzo1x;

use crate::{
    common::{
        ClassId, Compression, FileFormat, GAMEBOX_FILE_SIGNATURE, GAMEBOX_VERSION, UNKNOWN_BYTE,
    },
    write::serialize::IdState,
};

use super::{
    serialize::{IdStateRef, NodeState, NodeStateRef, Serializer},
    Result,
};

pub trait Sealed: ClassId + HeaderChunks + WriteBody {}

pub fn write_gbx<T: ClassId + HeaderChunks + WriteBody>(
    node: &T,
    writer: impl Write,
    body_compression: Compression,
) -> Result {
    let header_data = {
        let mut id_state = IdState::new();
        let mut header_chunks = vec![];

        for header_chunk in T::header_chunks() {
            let mut header_chunk_data = vec![];

            let mut s = Serializer::new(&mut header_chunk_data, &mut id_state, ());

            (header_chunk.write_fn)(node, &mut s)?;

            header_chunks.push((header_chunk.chunk_id, header_chunk_data));
        }

        let mut header_data = vec![];

        let mut s = Serializer::new(&mut header_data, (), ());

        s.u32(header_chunks.len() as u32)?;

        for (chunk_id, chunk_data) in &header_chunks {
            s.u32(*chunk_id)?;
            s.u32(chunk_data.len() as u32)?;
        }

        for (_, chunk_data) in header_chunks {
            s.bytes(&chunk_data)?;
        }

        header_data
    };

    let node_state = NodeState::new();
    let mut body = vec![];

    {
        let mut s = Serializer::new(&mut body, IdState::new(), &node_state);
        T::write_body(node, &mut s)?;
    }

    let mut s = Serializer::new(writer, (), ());

    s.byte_array(GAMEBOX_FILE_SIGNATURE)?;
    s.u16(GAMEBOX_VERSION)?;
    FileFormat::Binary.write(&mut s)?;
    Compression::Uncompressed.write(&mut s)?;
    body_compression.write(&mut s)?;
    s.u8(UNKNOWN_BYTE)?;
    s.u32(T::class_id())?;
    s.u32(header_data.len() as u32)?;
    s.bytes(&header_data)?;
    s.u32(node_state.num_nodes())?;

    s.u32(0)?;

    match body_compression {
        Compression::Compressed => {
            let mut buf = vec![0; lzo1x::worst_compress_size(body.len())];
            let compressed_body = lzo1x::compress_1(&body, &mut buf).unwrap();

            s.u32(body.len() as u32)?;
            s.u32(compressed_body.len() as u32)?;
            s.bytes(compressed_body)?;
        }
        Compression::Uncompressed => {
            s.bytes(&body)?;
        }
    }

    Ok(())
}

pub trait HeaderChunks
where
    Self: Sized,
{
    fn header_chunks() -> impl Iterator<Item = HeaderChunk<Self>>;
}

pub struct HeaderChunk<T> {
    pub chunk_id: u32,
    pub write_fn: HeaderChunkWriteFn<T>,
}

pub type HeaderChunkWriteFn<T> = fn(&T, &mut Serializer<&mut Vec<u8>, &mut IdState, ()>) -> Result;

pub trait WriteBody {
    fn write_body<W: Write, I: IdStateRef, N: NodeStateRef>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result;
}
