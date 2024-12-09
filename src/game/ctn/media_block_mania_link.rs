//! Media block manialink

use crate::Class;

/// A media block manialink.
#[derive(Default)]
pub struct MediaBlockManialink {
    start_time: f32,
    end_time: f32,
    url: String,
}

impl Class for MediaBlockManialink {
    const CLASS_ID: u32 = 0x0312a000;
}

impl MediaBlockManialink {
    /// Start time.
    pub const fn start_time(&self) -> f32 {
        self.start_time
    }

    /// End time.
    pub const fn end_time(&self) -> f32 {
        self.end_time
    }

    /// URL.
    pub const fn url(&self) -> &String {
        &self.url
    }
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

            self.start_time = r.f32()?;
            self.end_time = r.f32()?;
            self.url = r.string()?;

            Ok(())
        }
    }
}
