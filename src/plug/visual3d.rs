//! Visual 3D.

use std::ops::Deref;

use crate::Class;

use super::visual::Visual;

/// A visual 3D.
#[derive(Default)]
pub struct Visual3D {
    parent: Visual,
}

impl Class for Visual3D {
    const CLASS_ID: u32 = 0x0902c000;
}

impl Deref for Visual3D {
    type Target = Visual;

    fn deref(&self) -> &Visual {
        &self.parent
    }
}

mod read {

    use std::io::Read;

    use crate::read::{reader::Reader, BodyChunk, BodyChunks, Error};

    use super::Visual3D;

    impl BodyChunks for Visual3D {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::normal(4, Self::read_chunk_4),
            ]
            .into_iter()
        }
    }

    impl Visual3D {
        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
