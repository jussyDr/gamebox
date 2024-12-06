//! Media clip.

use crate::Class;

/// A media clip.
#[derive(Default)]
pub struct MediaClip;

impl Class for MediaClip {
    const CLASS_ID: u32 = 0x03079000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::MediaTrack,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::MediaClip;

    impl ReadBody for MediaClip {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaClip {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(13, Self::read_chunk_13)].into_iter()
        }
    }

    impl MediaClip {
        fn read_chunk_13(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _tracks = r.list_with_version(|r| r.internal_node_ref::<MediaTrack>())?;
            let _name = r.string()?;
            r.bool()?;

            Ok(())
        }
    }
}
