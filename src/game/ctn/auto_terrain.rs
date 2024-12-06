//! Auto terrain.

use std::sync::Arc;

use crate::{Class, Vec3};

use super::ZoneGenealogy;

/// An auto terrain.
#[derive(Default)]
pub struct AutoTerrain {
    offset: Vec3<u32>,
    genealogy: Arc<ZoneGenealogy>,
}

impl Class for AutoTerrain {
    const CLASS_ID: u32 = 0x03120000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::zone_genealogy::ZoneGenealogy,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::AutoTerrain;

    impl ReadBody for AutoTerrain {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for AutoTerrain {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(1, Self::read_chunk_1)].into_iter()
        }
    }

    impl AutoTerrain {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.offset = r.vec3::<u32>()?;
            self.genealogy = r.internal_node_ref::<ZoneGenealogy>()?;

            Ok(())
        }
    }
}
