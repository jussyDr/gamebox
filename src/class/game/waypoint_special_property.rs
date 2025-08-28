use crate::{
    game::read_encapsulation,
    read::{BodyReader, Error, ReadNode, Result, read_body_chunks},
    script::TraitsMetadata,
};

pub struct WaypointSpecialProperty {
    chunk_0: Chunk0,
    chunk_1: Chunk1,
}

struct Chunk0;

struct Chunk1;

impl ReadNode for WaypointSpecialProperty {
    const CLASS_ID: u32 = 0x2e009000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            let chunk_0 = r.chunk(0x2e009000, |r| {
                if r.u32()? != 2 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                let _tag = r.string()?;
                let _order = r.u32()?;

                Ok(Chunk0)
            })?;
            let chunk_1 = r.chunk_skippable(0x2e009001, |r| {
                if r.u32()? != 0 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                if r.bool32()? {
                    read_encapsulation(r, |r| {
                        let _script_metadata = r.node::<TraitsMetadata>()?;

                        Ok(())
                    })?;
                }

                Ok(Chunk1)
            })?;

            Ok(Self { chunk_0, chunk_1 })
        })
    }
}
