use std::io::Write;

use lzo1x::CompressLevel;

use crate::{
    common::{
        Class, Compression, FileFormat, GAMEBOX_FILE_SIGNATURE, GAMEBOX_FILE_VERSION,
        HEAVY_CHUNK_MARKER_BIT, UNKNOWN_BYTE,
    },
    serialize::{IdState, NodeState, Serializer},
};

use super::Result;

pub trait Sealed:
    Class + HeaderChunks + for<'a> WriteBody<&'a mut Vec<u8>, IdState, &'a mut NodeState>
{
}

pub fn write_gbx<
    T: Class + HeaderChunks + for<'a> WriteBody<&'a mut Vec<u8>, IdState, &'a mut NodeState>,
>(
    node: &T,
    writer: impl Write,
    body_compression: Option<CompressLevel>,
) -> Result {
    let header_data = {
        let mut id_state = IdState::new();
        let mut header_chunks = vec![];

        for header_chunk in T::header_chunks() {
            let mut header_chunk_data = vec![];

            let mut s = Serializer::new(&mut header_chunk_data, &mut id_state, ());

            (header_chunk.write_fn)(node, &mut s)?;

            header_chunks.push((
                header_chunk.chunk_id,
                header_chunk.is_heavy,
                header_chunk_data,
            ));
        }

        let mut header_data = vec![];

        let mut s = Serializer::new(&mut header_data, (), ());

        s.u32(header_chunks.len() as u32)?;

        for (chunk_id, is_heavy, chunk_data) in &header_chunks {
            s.u32(*chunk_id)?;

            if *is_heavy {
                s.u32(chunk_data.len() as u32 | HEAVY_CHUNK_MARKER_BIT)?;
            } else {
                s.u32(chunk_data.len() as u32)?;
            }
        }

        for (_, _, chunk_data) in header_chunks {
            s.bytes(&chunk_data)?;
        }

        header_data
    };

    let mut node_state = NodeState::new();
    let mut body = vec![];

    {
        let mut s = Serializer::new(&mut body, IdState::new(), &mut node_state);
        T::write_body(node, &mut s)?;
    }

    let mut s = Serializer::new(writer, (), ());

    s.byte_array(GAMEBOX_FILE_SIGNATURE)?;
    s.u16(GAMEBOX_FILE_VERSION)?;
    FileFormat::Binary.write(&mut s)?;
    Compression::Uncompressed.write(&mut s)?;
    let compression = match body_compression {
        None => Compression::Uncompressed,
        _ => Compression::Compressed,
    };
    compression.write(&mut s)?;
    s.u8(UNKNOWN_BYTE)?;
    s.u32(T::CLASS_ID.as_u32())?;
    s.u32(header_data.len() as u32)?;
    s.bytes(&header_data)?;
    s.u32(node_state.num_nodes())?;

    s.u32(0)?;

    match body_compression {
        None => s.bytes(&body)?,
        Some(level) => {
            let compressed_body = lzo1x::compress(&body, level);

            s.u32(body.len() as u32)?;
            s.u32(compressed_body.len() as u32)?;
            s.bytes(&compressed_body)?;
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
    pub is_heavy: bool,
    pub write_fn: HeaderChunkWriteFn<T>,
}

pub type HeaderChunkWriteFn<T> = fn(&T, &mut Serializer<&mut Vec<u8>, &mut IdState, ()>) -> Result;

pub trait WriteBody<W, I, N> {
    fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result;
}
