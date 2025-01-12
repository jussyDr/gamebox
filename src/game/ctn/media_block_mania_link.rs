//! Media block manialink

use ordered_float::OrderedFloat;

use crate::Class;

/// A media block manialink.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockManialink {
    start_time: OrderedFloat<f32>,
    end_time: OrderedFloat<f32>,
    url: String,
}

impl Class for MediaBlockManialink {
    const CLASS_ID: u32 = 0x0312a000;
}

impl MediaBlockManialink {
    /// Start time.
    pub const fn start_time(&self) -> f32 {
        self.start_time.0
    }

    /// End time.
    pub const fn end_time(&self) -> f32 {
        self.end_time.0
    }

    /// URL.
    pub const fn url(&self) -> &String {
        &self.url
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

            self.start_time = OrderedFloat(r.f32()?);
            self.end_time = OrderedFloat(r.f32()?);
            self.url = r.string()?;

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

    use super::MediaBlockManialink;

    impl WriteBody for MediaBlockManialink {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaBlockManialink {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
