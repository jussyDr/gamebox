//! Media track.

use std::sync::Arc;

use crate::Class;

use super::MediaBlock;

/// Media track.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaTrack {
    name: String,
    blocks: Vec<Arc<MediaBlock>>,
}

impl Class for MediaTrack {
    const CLASS_ID: u32 = 0x03078000;
}

impl MediaTrack {
    /// Name of the media track.
    pub const fn name(&self) -> &String {
        &self.name
    }

    /// Media blocks of the media track.
    pub const fn blocks(&self) -> &Vec<Arc<MediaBlock>> {
        &self.blocks
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::media_block::MediaBlock,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::MediaTrack;

    impl ReadBody for MediaTrack {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaTrack {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::normal(5, Self::read_chunk_5),
            ]
            .into_iter()
        }
    }

    impl MediaTrack {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.name = r.string()?;
            self.blocks =
                r.list_with_version(|r| r.test(|r, class_id| MediaBlock::read(r, class_id)))?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _is_keep_playing = r.bool()?;
            let _is_read_only = r.bool()?;
            let _is_cycling = r.bool()?;
            r.f32()?;
            r.f32()?;

            Ok(())
        }
    }
}
