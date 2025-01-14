//! Game skin.

use crate::Class;

/// Game skin.
#[derive(Default)]
pub struct GameSkin;

impl Class for GameSkin {
    const CLASS_ID: u32 = 0x090f4000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::GameSkin;

    impl ReadBody for GameSkin {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for GameSkin {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            #![allow(clippy::redundant_closure)]
            [
                BodyChunk::normal(3, Self::read_chunk_3),
                BodyChunk::skippable(5, |s, r| Self::read_chunk_5(s, r)),
            ]
            .into_iter()
        }
    }

    impl GameSkin {
        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.string()?;
            r.string()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 8 {
                return Err(Error::chunk_version(version as u32));
            }

            let _relative_skin_directory = r.string()?;
            let _parent_pack_desc = r.string()?;
            r.string()?;
            let num_fids = r.u8()?;
            let _fids = r.repeat(num_fids as usize, |r| {
                let _class_id = r.u32()?;
                let _name = r.string()?;
                let _directory = r.string()?;
                r.bool()?;

                Ok(())
            })?;
            r.string()?;
            r.bool()?;
            r.string()?;
            r.u32()?;
            r.u8()?;

            Ok(())
        }
    }
}
