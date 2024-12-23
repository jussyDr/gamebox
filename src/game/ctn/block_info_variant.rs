//! Block info variant.

use std::sync::Arc;

use crate::Class;

use super::{block_info_mobil::BlockInfoMobil, block_unit_info::BlockUnitInfo, Direction};

/// A block info variant.
#[derive(Clone, Default)]
pub struct BlockInfoVariant {
    direction: Direction,
    mobils: Vec<Vec<Arc<BlockInfoMobil>>>,
    block_unit_models: Vec<Arc<BlockUnitInfo>>,
}

impl Class for BlockInfoVariant {
    const CLASS_ID: u32 = 0x0315b000;
}

impl BlockInfoVariant {
    /// Direction.
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    /// Mobils.
    pub const fn mobils(&self) -> &Vec<Vec<Arc<BlockInfoMobil>>> {
        &self.mobils
    }

    /// Block unit models.
    pub const fn block_unit_models(&self) -> &Vec<Arc<BlockUnitInfo>> {
        &self.block_unit_models
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::{
            block_info_mobil::BlockInfoMobil, block_unit_info::BlockUnitInfo, BlockInfoClassic,
        },
        plug::{entity_spawner::EntitySpawner, Solid},
        read::{
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error,
        },
    };

    use super::BlockInfoVariant;

    impl BodyChunks for BlockInfoVariant {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::normal(3, Self::read_chunk_3),
                BodyChunk::normal(4, Self::read_chunk_4),
                BodyChunk::normal(5, Self::read_chunk_5),
                BodyChunk::normal(6, Self::read_chunk_6),
                BodyChunk::normal(7, Self::read_chunk_7),
                BodyChunk::normal(8, Self::read_chunk_8),
                BodyChunk::normal(9, Self::read_chunk_9),
                BodyChunk::normal(10, Self::read_chunk_10),
                BodyChunk::normal(11, Self::read_chunk_11),
                BodyChunk::normal(13, Self::read_chunk_13),
            ]
            .into_iter()
        }
    }

    impl BlockInfoVariant {
        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _multi_dir = r.u32()?;

            Ok(())
        }

        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            let _symmetrical_variant_index = r.u32()?;
            self.direction = r.enum_u8()?;
            let _variant_base_type = r.u8()?;
            let _no_pillar_below_index = r.u8()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u16()?;

            Ok(())
        }

        fn read_chunk_5(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            self.mobils = r.list(|r| r.list(|r| r.internal_node_ref::<BlockInfoMobil>()))?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_6(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 11 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            r.internal_node_ref_or_null::<Solid>()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            let _entity_spawners = r.list(|r| r.internal_node_ref::<EntitySpawner>())?;

            Ok(())
        }

        fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _probe = r.u32()?;

            Ok(())
        }

        fn read_chunk_8(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            self.block_unit_models = r.list(|r| r.internal_node_ref::<BlockUnitInfo>())?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.box3d()?;
            let _name = r.string()?;

            Ok(())
        }

        fn read_chunk_9(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            r.list(|r| {
                r.node_ref::<BlockInfoClassic>()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;
            r.list(|r| {
                r.node_ref::<BlockInfoClassic>()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u8()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_10<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_11<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _water_volumes = r.list(|r| {
                r.list(|r| r.box3d())?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.id()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_13<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
