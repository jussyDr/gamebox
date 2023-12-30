use std::io::Write;

use crate::{class::Class, write::serialize::IdState, FILE_SIGNATURE};

use super::{
    serialize::{IdStateMut, NodeState, NodeStateMut, Serializer},
    Result,
};

pub trait Sealed: Class + HeaderChunks + WriteBody {}

pub fn write_gbx<T: Class + HeaderChunks + WriteBody>(
    node: &T,
    writer: impl Write,
    compress_body: bool,
) -> Result<()> {
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

    let mut node_state = NodeState::new();
    let mut body = vec![];

    {
        let mut s = Serializer::new(&mut body, IdState::new(), &mut node_state);
        T::write_body(node, &mut s)?;
    }

    let mut s = Serializer::new(writer, (), ());

    s.byte_array(FILE_SIGNATURE)?;
    s.u16(6)?;
    s.u8(b'B')?;
    s.u8(b'U')?;

    if compress_body {
        s.u8(b'C')?;
    } else {
        s.u8(b'U')?;
    }

    s.u8(b'R')?;
    s.u32(T::class_id())?;
    s.u32(header_data.len() as u32)?;
    s.bytes(&header_data)?;
    s.u32(node_state.num_nodes())?;

    s.u32(0)?;

    if compress_body {
        let mut buf = vec![0; lzo1x_1::worst_compress(body.len())];
        let compressed_body = lzo1x_1::compress_to_slice(&body, &mut buf);

        s.u32(body.len() as u32)?;
        s.u32(compressed_body.len() as u32)?;
        s.bytes(compressed_body)?;
    } else {
        s.bytes(&body)?;
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
    pub write_fn: fn(&T, &mut Serializer<&mut Vec<u8>, &mut IdState, ()>) -> Result<()>,
}

pub trait WriteBody {
    fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result<()>;
}
