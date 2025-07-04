use std::ops::{Deref, DerefMut};

use crate::{ClassId, class::plug::visual::Visual};

/// A visual 3D.
#[derive(Default)]
pub struct Visual3D {
    parent: Visual,
}

impl ClassId for Visual3D {
    const CLASS_ID: u32 = 0x0902C000;
}

impl Deref for Visual3D {
    type Target = Visual;

    fn deref(&self) -> &Visual {
        &self.parent
    }
}

impl DerefMut for Visual3D {
    fn deref_mut(&mut self) -> &mut Visual {
        &mut self.parent
    }
}

mod read {
    use crate::{
        class::plug::visual_3d::Visual3D,
        read::{BodyChunk, BodyChunks, Error, reader::BodyReader},
    };

    impl BodyChunks for Visual3D {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(2, Self::read_chunk_2),
                BodyChunk::new(4, Self::read_chunk_4),
            ]
        }
    }

    impl Visual3D {
        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_4(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
