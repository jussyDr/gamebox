use crate::Class;

#[derive(Default)]
pub struct AutoTerrain;

impl Class for AutoTerrain {
    const CLASS_ID: u32 = 0x03120000;
}

mod read {
    use std::io::Read;

    use crate::{
        engines::game::zone_genealogy::ZoneGenealogy,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::AutoTerrain;

    impl ReadBody for AutoTerrain {
        fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for AutoTerrain {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(1, Self::read_chunk_1)].into_iter()
        }
    }

    impl AutoTerrain {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _offset = r.u32()?;
            let _offset = r.u32()?;
            let _offset = r.u32()?;
            let _genealogy = r.internal_node_ref::<ZoneGenealogy>()?;

            Ok(())
        }
    }
}
