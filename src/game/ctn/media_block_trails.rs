//! Media block trails.

use ordered_float::OrderedFloat;

use crate::Class;

/// Media block trails.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockTrails {
    start_time: OrderedFloat<f32>,
    end_time: OrderedFloat<f32>,
}

impl Class for MediaBlockTrails {
    const CLASS_ID: u32 = 0x030a9000;
}

impl MediaBlockTrails {
    /// Start time.
    pub const fn start_time(&self) -> f32 {
        self.start_time.0
    }

    /// End time.
    pub const fn end_time(&self) -> f32 {
        self.end_time.0
    }
}

mod read {
    use std::io::{Read, Seek};

    use ordered_float::OrderedFloat;

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
            self.start_time = OrderedFloat(r.f32()?);
            self.end_time = OrderedFloat(r.f32()?);

            Ok(())
        }
    }
}

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::MediaBlockTrails;

    impl WriteBody for MediaBlockTrails {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaBlockTrails {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
