//! Media block DOF.

use crate::Class;

/// DOF media block.
#[derive(Default)]
pub struct MediaBlockDof;

impl Class for MediaBlockDof {
    const CLASS_ID: u32 = 0x03126000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaBlockDof;

    impl ReadBody for MediaBlockDof {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockDof {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(2, Self::read_chunk_2)].into_iter()
        }
    }

    impl MediaBlockDof {
        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _z_focus = r.f32()?;
                let _lens_size = r.f32()?;
                let _target = r.u32()?;
                let _target_position = r.vec3::<f32>()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
