use crate::read::{BodyReader, ReadNode, Result, read_body_chunks};

pub struct CollectorList {
    chunk_0: Chunk0,
}

struct Chunk0;

impl ReadNode for CollectorList {
    const CLASS_ID: u32 = 0x0301b000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_0: r.chunk(0x0301b000, |r| {
                    let _collector_stock = r.list(|r| {
                        let _block_model_id = r.string_ref()?;
                        let _block_model_collection = r.string_ref()?;
                        let _block_model_author = r.string_ref()?;
                        let _count = r.u32()?;

                        Ok(())
                    })?;

                    Ok(Chunk0)
                })?,
            })
        })
    }
}
