//! Block info mobil.

use crate::{plug::Prefab, Class, ExternalNodeRef};

/// A block info mobil.
#[derive(Default)]
pub struct BlockInfoMobil {
    prefab: Option<ExternalNodeRef<Prefab>>,
}

impl Class for BlockInfoMobil {
    const CLASS_ID: u32 = 0x03122000;
}

impl BlockInfoMobil {
    /// Prefab of the mobil.
    pub const fn prefab(&self) -> Option<&ExternalNodeRef<Prefab>> {
        self.prefab.as_ref()
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::placement_patch::PlacementPatch,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::BlockInfoMobil;

    impl ReadBody for BlockInfoMobil {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockInfoMobil {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::normal(3, Self::read_chunk_3),
                BodyChunk::normal(4, Self::read_chunk_4),
            ]
            .into_iter()
        }
    }

    impl BlockInfoMobil {
        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _solid_decals = r.list_with_version(|_| Ok(()))?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_3(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 23 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            if r.bool8()? {
                let _geom_translation = r.vec3::<f32>()?;
                let _geom_rotation = r.pitch_yaw_roll()?;
            }

            r.u32()?;
            r.u32()?;
            self.prefab = r.external_node_ref_or_null()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u8()?;
            r.f32()?;
            r.u32()?;
            r.list(|r| r.node_ref::<PlacementPatch>())?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _dyna_links = r.list_with_version(|_| Ok(()))?;

            Ok(())
        }
    }
}
