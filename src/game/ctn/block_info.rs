use std::sync::Arc;

use crate::{game::ctn::collector::Collector, Class};

use super::{
    block_info_variant_air::BlockInfoVariantAir, block_info_variant_ground::BlockInfoVariantGround,
};

/// A block info.
#[derive(Default)]
pub struct BlockInfo {
    parent: Collector,
    variant_base_ground: BlockInfoVariantGround,
    variant_base_air: BlockInfoVariantAir,
    additional_variants_ground: Vec<Arc<BlockInfoVariantGround>>,
    additional_variants_air: Vec<Arc<BlockInfoVariantAir>>,
}

impl Class for BlockInfo {
    const CLASS_ID: u32 = 0x0304e000;
}

impl BlockInfo {
    pub const fn variant_base_ground(&self) -> &BlockInfoVariantGround {
        &self.variant_base_ground
    }

    pub const fn variant_base_air(&self) -> &BlockInfoVariantAir {
        &self.variant_base_air
    }

    pub fn additional_variants_ground(&self) -> &[Arc<BlockInfoVariantGround>] {
        &self.additional_variants_ground
    }

    pub fn additional_variants_air(&self) -> &[Arc<BlockInfoVariantAir>] {
        &self.additional_variants_air
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::{
            block_info_variant_air::BlockInfoVariantAir,
            block_info_variant_ground::BlockInfoVariantGround, Direction,
        },
        read::{
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::BlockInfo;

    impl BodyChunks for BlockInfo {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(15, Self::read_chunk_15),
                BodyChunk::new(19, Self::read_chunk_19),
                BodyChunk::new(23, Self::read_chunk_23),
                BodyChunk::new(32, Self::read_chunk_32),
                BodyChunk::new(35, Self::read_chunk_35),
                BodyChunk::new(38, Self::read_chunk_38),
                BodyChunk::new(39, Self::read_chunk_39),
                BodyChunk::new(40, Self::read_chunk_40),
                BodyChunk::new(41, Self::read_chunk_41),
                BodyChunk::new(42, Self::read_chunk_42),
                BodyChunk::new(43, Self::read_chunk_43),
                BodyChunk::new(44, Self::read_chunk_44),
                BodyChunk::new(47, Self::read_chunk_47),
                BodyChunk::new(49, Self::read_chunk_49),
            ]
            .into_iter()
        }
    }

    impl BlockInfo {
        fn read_chunk_15<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _no_respawn = r.bool()?;

            Ok(())
        }

        fn read_chunk_19<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _icon_use_auto_ground = r.bool()?;

            Ok(())
        }

        fn read_chunk_23<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.bool()?;

            Ok(())
        }

        fn read_chunk_32<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 8 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            r.u32()?;
            r.u32()?;
            let _char_phy_special_property_customizable = r.bool()?;

            if r.bool()? {
                r.string()?;
                r.string()?;
            }

            Ok(())
        }

        fn read_chunk_35(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.variant_base_ground = BlockInfoVariantGround::read_from_body(r)?;
            self.variant_base_air = BlockInfoVariantAir::read_from_body(r)?;

            Ok(())
        }

        fn read_chunk_38<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _waypoint_type = r.u32()?;

            Ok(())
        }

        fn read_chunk_39(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.additional_variants_ground =
                r.list_with_version(|r| r.internal_node_ref::<BlockInfoVariantGround>())?;

            Ok(())
        }

        fn read_chunk_40<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _symmetrical_block_info_id = r.id_or_null()?;
            let _dir = r.enum_u32::<Direction>()?;

            Ok(())
        }

        fn read_chunk_41(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _fog_volume_box = r.external_node_ref_or_null::<()>()?;

            Ok(())
        }

        fn read_chunk_42(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            let _sound_1 = r.external_node_ref_or_null::<()>()?;
            let _sound_2 = r.external_node_ref_or_null::<()>()?;

            Ok(())
        }

        fn read_chunk_43<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_44(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.additional_variants_air =
                r.list_with_version(|r| r.internal_node_ref::<BlockInfoVariantAir>())?;

            Ok(())
        }

        fn read_chunk_47<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _is_pillar = r.bool8()?;
            let _pillar_shape_multi_dir = r.u8()?;
            r.u8()?;

            Ok(())
        }

        fn read_chunk_49<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
