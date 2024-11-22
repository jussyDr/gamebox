use crate::Class;

#[derive(Default)]
pub struct BlockInfoMobil;

impl Class for BlockInfoMobil {
    const CLASS_ID: u32 = 0x03122000;
}

mod read {
    use std::io::Read;

    use crate::{
        engines::plug::placement_patch::PlacementPatch,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::BlockInfoMobil;

    impl ReadBody for BlockInfoMobil {
        fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockInfoMobil {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(2, Self::read_chunk_2),
                BodyChunk::new(3, Self::read_chunk_3),
                BodyChunk::new(4, Self::read_chunk_4),
            ]
            .into_iter()
        }
    }

    impl BlockInfoMobil {
        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _solid_decals = r.list_with_version(|r| {
                todo!();

                Ok(())
            })?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_3(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 23 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            if r.bool8()? {
                todo!()
            }

            r.u32()?;
            r.u32()?;
            let _prefab_fid = r.external_node_ref::<()>()?;
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

            let _dyna_links = r.list_with_version(|r| {
                todo!();

                Ok(())
            })?;

            Ok(())
        }
    }
}
