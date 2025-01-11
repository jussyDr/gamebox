//! Media block interface.

use ordered_float::OrderedFloat;

use crate::Class;

/// Interface media block.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockInterface {
    start_time: OrderedFloat<f32>,
    end_time: OrderedFloat<f32>,
    show: bool,
    manialink: String,
}

impl Class for MediaBlockInterface {
    const CLASS_ID: u32 = 0x03195000;
}

impl MediaBlockInterface {
    /// Start time.
    pub const fn start_time(&self) -> f32 {
        self.start_time.0
    }

    /// End time.
    pub const fn end_time(&self) -> f32 {
        self.end_time.0
    }

    /// Show.
    pub const fn show(&self) -> bool {
        self.show
    }

    /// Manialink.
    pub const fn manialink(&self) -> &String {
        &self.manialink
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

    use super::MediaBlockInterface;

    impl ReadBody for MediaBlockInterface {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockInterface {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl MediaBlockInterface {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            self.start_time = OrderedFloat(r.f32()?);
            self.end_time = OrderedFloat(r.f32()?);
            self.show = r.bool()?;
            self.manialink = r.string()?;

            Ok(())
        }
    }
}
