//! Item model.

use std::sync::Arc;

use crate::Class;

use super::{
    common_item_entity_model_edition::CommonItemEntityModelEdition, ctn::collector::Collector,
};

/// An item model.
#[derive(PartialEq, Default, Debug)]
pub struct ItemModel {
    parent: Collector,
    entity_model_edition: Arc<CommonItemEntityModelEdition>,
}

impl Class for ItemModel {
    const CLASS_ID: u32 = 0x2e002000;
}

impl ItemModel {
    pub const fn entity_model_edition(&self) -> &Arc<CommonItemEntityModelEdition> {
        &self.entity_model_edition
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::{
            common_item_entity_model_edition::CommonItemEntityModelEdition,
            item_placement_param::ItemPlacementParam,
        },
        read::{
            read_body_chunks,
            readable::Sealed,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody, Readable,
        },
    };

    use super::ItemModel;

    impl Readable for ItemModel {}

    impl Sealed for ItemModel {}

    impl ReadBody for ItemModel {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for ItemModel {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(8, Self::read_chunk_8),
                BodyChunk::normal(9, Self::read_chunk_9),
                BodyChunk::normal(12, Self::read_chunk_12),
                BodyChunk::normal(18, Self::read_chunk_18),
                BodyChunk::normal(21, Self::read_chunk_21),
                BodyChunk::normal(25, Self::read_chunk_25),
                BodyChunk::normal(28, Self::read_chunk_28),
                BodyChunk::normal(30, Self::read_chunk_30),
                BodyChunk::normal(31, Self::read_chunk_31),
                BodyChunk::normal(32, Self::read_chunk_32),
                BodyChunk::normal(33, Self::read_chunk_33),
                BodyChunk::normal(35, Self::read_chunk_35),
                BodyChunk::skippable(36, Self::read_chunk_36),
                BodyChunk::skippable(37, Self::read_chunk_37),
                BodyChunk::skippable(38, Self::read_chunk_38),
                BodyChunk::skippable(39, Self::read_chunk_39),
            ]
            .into_iter()
        }
    }

    impl ItemModel {
        fn read_chunk_8<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _nadeo_skin_fids = r.list(|r| r.u32())?;

            Ok(())
        }

        fn read_chunk_9<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _cameras = r.list_with_version(|r| r.u32())?;

            Ok(())
        }

        fn read_chunk_12<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _race_interface_fid = r.u32()?;

            Ok(())
        }

        fn read_chunk_18<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _ground_point = r.vec3::<f32>()?;
            let _painter_ground_margin = r.f32()?;
            let _orbital_center_height_from_ground = r.f32()?;
            let _orbital_radius_base = r.f32()?;
            let _orbital_preview_angle = r.f32()?;

            Ok(())
        }

        fn read_chunk_21<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _item_type = r.u32()?;

            Ok(())
        }

        fn read_chunk_25(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 13 {
                return Err(Error::chunk_version(version));
            }

            let _default_weapon_name = r.id_or_null()?;
            let _phy_model_custom = r.u32()?;
            let _vis_custom_model = r.u32()?;
            let _actions = r.list(|r| r.u32())?;
            let _default_cam = r.u32()?;
            self.entity_model_edition = r.internal_node_ref::<CommonItemEntityModelEdition>()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_28(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(Error::chunk_version(version));
            }

            let _default_placement = r.internal_node_ref::<ItemPlacementParam>()?;

            Ok(())
        }

        fn read_chunk_30<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(Error::chunk_version(version));
            }

            let archetype_ref = r.string()?;

            if archetype_ref.is_empty() {
                r.u32()?;
            }

            r.string()?;

            Ok(())
        }

        fn read_chunk_31<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 10 {
                return Err(Error::chunk_version(version));
            }

            let _waypoint_type = r.u32()?;
            let _disable_lightmap = r.bool()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_32<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            let _icon_fid = r.string()?;
            r.u8()?;

            Ok(())
        }

        fn read_chunk_33<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_35<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.u8()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_36<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.list(|r| {
                r.f32()?;
                r.f32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_37<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.bool()?;

            Ok(())
        }

        fn read_chunk_38<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.list(|r| r.vec3::<f32>())?;

            Ok(())
        }

        fn read_chunk_39<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.f32()?;

            Ok(())
        }
    }
}

mod write {
    use crate::write::{writable, Writable};

    use self::writable::BodyChunks;

    use super::ItemModel;

    impl Writable for ItemModel {}

    impl writable::Sealed for ItemModel {}

    impl BodyChunks for ItemModel {
        fn body_chunks<W, I, N>() -> impl Iterator<Item = writable::BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
