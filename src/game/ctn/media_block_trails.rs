//! Media block trails.

use crate::Class;

/// Media block trails.
#[derive(Default)]
pub struct MediaBlockTrails {
    start_time: f32,
    end_time: f32,
}

impl Class for MediaBlockTrails {
    const CLASS_ID: u32 = 0x030a9000;
}

impl MediaBlockTrails {
    /// Start time.
    pub const fn start_time(&self) -> f32 {
        self.start_time
    }

    /// End time.
    pub const fn end_time(&self) -> f32 {
        self.end_time
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaBlockTrails;

    impl ReadBody for MediaBlockTrails {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockTrails {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl MediaBlockTrails {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.start_time = r.f32()?;
            self.end_time = r.f32()?;

            Ok(())
        }
    }
}
