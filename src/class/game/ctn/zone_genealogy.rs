use crate::{
    game::ctn::Direction,
    read::{BodyReader, ReadNode, Result, read_body_chunks},
};

pub struct ZoneGenealogy {
    chunk_2: Chunk2,
}

struct Chunk2;

impl ReadNode for ZoneGenealogy {
    const CLASS_ID: u32 = 0x0311d000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_2: r.chunk(0x0311d002, |r| {
                    let _zone_ids = r.list(|r| r.string_ref())?;
                    let _current_index = r.u32()?;
                    let _direction = r.enum32::<Direction>()?;
                    let _current_zone = r.string_ref()?;

                    Ok(Chunk2)
                })?,
            })
        })
    }
}
