//! Spawn model.

use crate::{Class, Iso4};

/// Spawn model.
#[derive(Default, Debug)]
pub struct SpawnModel {
    placement: Iso4,
}

impl Class for SpawnModel {
    const CLASS_ID: u32 = 0x0917a000;
}

impl SpawnModel {
    /// Placement.
    pub const fn placement(&self) -> Iso4 {
        self.placement
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::SpawnModel;

    impl ReadBody for SpawnModel {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for SpawnModel {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl SpawnModel {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            self.placement = r.iso4()?;
            let _torque_x = r.f32()?;
            let _torque_duration = r.u32()?;
            let _default_gravity_spawn = r.vec3()?;
            r.u32()?;

            Ok(())
        }
    }
}
