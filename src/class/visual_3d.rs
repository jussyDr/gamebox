use crate::{Class, class::visual::Visual};

#[derive(Default)]
pub struct Visual3D {
    parent: Visual,
}

impl Class for Visual3D {
    fn class_id(&self) -> u32 {
        0x0902C000
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::{visual::Visual, visual_3d::Visual3D},
        read::{
            BodyChunk, BodyChunks, Error,
            reader::{IdsMut, NodesMut, Reader},
        },
    };

    impl BodyChunks for Visual3D {
        type Parent = Visual;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdsMut, N: NodesMut>()
        -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(0x0902c002, Self::read_chunk_2),
                BodyChunk::new(0x0902c004, Self::read_chunk_4),
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
