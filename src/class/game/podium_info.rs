//! Podium info.

use crate::ClassId;

/// Podium info.
#[derive(Default)]
pub struct PodiumInfo;

impl ClassId for PodiumInfo {
    const CLASS_ID: u32 = 0x03168000;
}

mod read {
    use crate::{
        class::game::podium_info::PodiumInfo,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for PodiumInfo {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for PodiumInfo {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            []
        }
    }
}
