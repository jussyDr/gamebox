use crate::Class;

#[derive(Default)]
pub struct TreeGenerator;

impl Class for TreeGenerator {
    const CLASS_ID: u32 = 0x09051000;
}

mod read {
    use std::io::Read;

    use crate::read::{reader::Reader, BodyChunk, BodyChunks, Error};

    use super::TreeGenerator;

    impl BodyChunks for TreeGenerator {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl TreeGenerator {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }
    }
}
