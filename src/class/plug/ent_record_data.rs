use crate::read::{BodyReader, Error, ReadNode, Result, read_body_chunks};

pub struct EntRecordData {
    chunk_0: Chunk0,
}

struct Chunk0;

impl ReadNode for EntRecordData {
    const CLASS_ID: u32 = 0x0911f000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            let chunk_0 = r.chunk(0x0911f000, |r| {
                if r.u32()? != 10 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                let _data_size = r.u32()?;
                let _compressed_data = r.list_u8()?;

                Ok(Chunk0)
            })?;

            Ok(Self { chunk_0 })
        })
    }
}
