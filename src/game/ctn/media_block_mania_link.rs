//! Media block manialink

use crate::Class;

/// A media block manialink.
#[derive(Default)]
pub struct MediaBlockManialink;

impl Class for MediaBlockManialink {
    const CLASS_ID: u32 = 0x0312a000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaBlockManialink;

    impl ReadBody for MediaBlockManialink {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockManialink {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(1, Self::read_chunk_1)].into_iter()
        }
    }

    impl MediaBlockManialink {
        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _start = r.f32()?;
            let _end = r.f32()?;
            let _manialink_url = r.string()?;

            Ok(())
        }
    }
}
