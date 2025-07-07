//! Block info variant.

use std::sync::Arc;

use crate::{ClassId, class::game::ctn::block_info_mobil::BlockInfoMobil};

/// Block info variant.
#[derive(Default)]
pub struct BlockInfoVariant {
    mobils: Vec<Vec<Arc<BlockInfoMobil>>>,
}

impl BlockInfoVariant {
    /// Mobils.
    pub fn mobils(&self) -> &Vec<Vec<Arc<BlockInfoMobil>>> {
        &self.mobils
    }
}

impl ClassId for BlockInfoVariant {
    const CLASS_ID: u32 = 0x0315b000;
}

mod read {
    use crate::{
        Delme,
        class::game::ctn::{
            block_info_classic::BlockInfoClassic, block_info_variant::BlockInfoVariant,
            block_unit_info::BlockUnitInfo,
        },
        read::{BodyChunk, BodyChunks, Error, error_unknown_chunk_version, reader::BodyReader},
    };

    impl BodyChunks for BlockInfoVariant {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(2, Self::read_chunk_2),
                BodyChunk::new(3, Self::read_chunk_3),
                BodyChunk::new(4, Self::read_chunk_4),
                BodyChunk::new(5, Self::read_chunk_5),
                BodyChunk::new(6, Self::read_chunk_6),
                BodyChunk::new(7, Self::read_chunk_7),
                BodyChunk::new(8, Self::read_chunk_8),
                BodyChunk::new(9, Self::read_chunk_9),
                BodyChunk::new(10, Self::read_chunk_10),
                BodyChunk::new(11, Self::read_chunk_11),
                BodyChunk::new(13, Self::read_chunk_13),
            ]
        }
    }

    impl BlockInfoVariant {
        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _multi_dir = r.u32()?;

            Ok(())
        }

        fn read_chunk_3(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_chunk_version(version));
            }

            let _symmetrical_variant_index = r.u32()?;
            let _cardinal_dir = r.u8()?;
            let _variant_base_type = r.u8()?;
            let _no_pillar_below_index = r.u8()?;

            Ok(())
        }

        fn read_chunk_4(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u16()?;

            Ok(())
        }

        fn read_chunk_5(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            self.mobils = r.list(|r| r.list(|r| r.internal_node_ref()))?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_6(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 11 {
                return Err(error_unknown_chunk_version(version));
            }

            let _screen_interaction_trigger_solid = r.external_node_ref_or_null::<Delme>()?;
            let _waypoint_trigger_solid = r.external_node_ref_or_null::<Delme>()?;
            let _gate = r.external_node_ref_or_null::<Delme>()?;
            let _teleporter = r.u32()?;
            r.u32()?;
            let _turbine = r.external_node_ref_or_null::<Delme>()?;
            let flock_model = r.external_node_ref_or_null::<Delme>()?;

            if flock_model.is_some() {
                todo!()
            }

            let _spawn_model = r.external_node_ref_or_null::<Delme>()?;
            r.u32()?;
            r.u32()?;
            let _entity_spawners: Vec<()> = r.list(|r| todo!())?;

            Ok(())
        }

        fn read_chunk_7(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let _probe = r.external_node_ref_or_null::<Delme>()?;

            Ok(())
        }

        fn read_chunk_8(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_chunk_version(version));
            }

            let _block_unit_models = r.list(|r| r.internal_node_ref::<BlockUnitInfo>())?;
            r.u32()?;
            let _has_manual_symmetry_h = r.bool32()?;
            let _has_manual_symmetry_v = r.bool32()?;
            let _has_manual_symmetry_d1 = r.bool32()?;
            let _has_manual_symmetry_d2 = r.bool32()?;
            r.box3d()?;
            let _name = r.string()?;

            Ok(())
        }

        fn read_chunk_9(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            r.list(|r| {
                r.external_node_ref_or_null::<BlockInfoClassic>()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;
            r.list(|r| {
                r.external_node_ref_or_null::<Delme>()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u8()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_10(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            let _compound_model = r.u32()?;

            Ok(())
        }

        fn read_chunk_11(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            let _water_volumes: Vec<()> = r.list(|r| todo!())?;

            Ok(())
        }

        fn read_chunk_13(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
