//! Media clip group.

use crate::Class;

/// A media clip group.
#[derive(Default)]
pub struct MediaClipGroup;

impl Class for MediaClipGroup {
    const CLASS_ID: u32 = 0x0307a000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::MediaClip,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::MediaClipGroup;

    impl ReadBody for MediaClipGroup {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaClipGroup {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(3, Self::read_chunk_3)].into_iter()
        }
    }

    impl MediaClipGroup {
        fn read_chunk_3(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _clips = r.list_with_version(|r| r.internal_node_ref::<MediaClip>())?;
            let _triggers = r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                let _condition = r.u32()?;
                let _condition_value = r.f32()?;
                let _coords = r.list(|r| r.vec3::<u32>())?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
