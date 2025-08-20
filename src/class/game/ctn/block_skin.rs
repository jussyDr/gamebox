use crate::{
    game::ctn::FileRef,
    read::{BodyReader, Error, ReadNode, Result, read_body_chunks},
};

pub struct BlockSkin {
    chunk_2: Chunk2,
    chunk_3: Chunk3,
}

struct Chunk2;

struct Chunk3;

impl ReadNode for BlockSkin {
    const CLASS_ID: u32 = 0x03059000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_2: r.chunk(0x03059002, |r| {
                    let _text = r.string()?;
                    let _file_ref = FileRef::read(r)?;
                    let _parent_file_ref = FileRef::read(r)?;

                    Ok(Chunk2)
                })?,
                chunk_3: r.chunk(0x03059003, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _foreground_file_ref = FileRef::read(r)?;

                    Ok(Chunk3)
                })?,
            })
        })
    }
}
