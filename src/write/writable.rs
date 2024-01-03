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
    BodyCompression, FastBodyCompression, Result, SlowBodyCompression,
};

pub trait Sealed: ClassId + HeaderChunks + WriteBody {}

pub fn write_gbx<T: ClassId + HeaderChunks + WriteBody>(
    node: &T,
    writer: impl Write,
    body_compression: BodyCompression,
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
    let compression = match body_compression {
        BodyCompression::None => Compression::Uncompressed,
        _ => Compression::Compressed,
    };
    compression.write(&mut s)?;
    s.u8(UNKNOWN_BYTE)?;
    s.u32(T::class_id())?;
    s.u32(header_data.len() as u32)?;
    s.bytes(&header_data)?;
    s.u32(node_state.num_nodes())?;

    s.u32(0)?;

    let compress_fn: CompressFn = match body_compression {
        BodyCompression::None => None,
        BodyCompression::Fast(level) => {
            let func = match level {
                FastBodyCompression::Level1 => lzo1x::compress_1_11,
                FastBodyCompression::Level2 => lzo1x::compress_1_12,
                FastBodyCompression::Level3 => lzo1x::compress_1,
                FastBodyCompression::Level4 => lzo1x::compress_1_15,
            };

            Some(Box::new(func))
        }
        BodyCompression::Slow(level) => {
            let level = match level {
                SlowBodyCompression::Level1 => 1,
                SlowBodyCompression::Level2 => 2,
                SlowBodyCompression::Level3 => 3,
                SlowBodyCompression::Level4 => 4,
                SlowBodyCompression::Level5 => 5,
                SlowBodyCompression::Level6 => 6,
                SlowBodyCompression::Level7 => 7,
                SlowBodyCompression::Level8 => 8,
                SlowBodyCompression::Level9 => 9,
            };

            Some(Box::new(move |src, dst| {
                lzo1x::compress_level(src, dst, &mut [], level)
            }))
        }
    };

    match compress_fn {
        None => s.bytes(&body)?,
        Some(compress_fn) => {
            let mut buf = vec![0; lzo1x::worst_compress_size(body.len())];
            let compressed_body = compress_fn(&body, &mut buf).unwrap();

            s.u32(body.len() as u32)?;
            s.u32(compressed_body.len() as u32)?;
            s.bytes(compressed_body)?;
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

type CompressFn = Option<
    Box<dyn for<'a> Fn(&[u8], &'a mut [u8]) -> std::result::Result<&'a mut [u8], lzo::Error>>,
>;
