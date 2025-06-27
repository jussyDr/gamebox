use std::ops::{Deref, DerefMut};

use crate::{Class, class::visual::Visual};

/// A visual 3D.
#[derive(Default)]
pub struct Visual3D {
    parent: Visual,
}

impl Class for Visual3D {
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
    use std::io::Read;

    use crate::{
        class::visual_3d::Visual3D,
        read::{
            BodyChunk, BodyChunks, Error,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl BodyChunks for Visual3D {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(0x0902c002, Self::read_chunk_2),
                BodyChunk::new(0x0902c004, Self::read_chunk_4),
            ]
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
