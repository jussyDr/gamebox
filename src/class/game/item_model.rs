//! Item model.

use std::ops::{Deref, DerefMut};

use crate::{ClassId, SubExtensions, class::game::ctn::collector::Collector};

/// Item model.
#[derive(Default)]
pub struct ItemModel {
    parent: Collector,
}

impl ClassId for ItemModel {
    const CLASS_ID: u32 = 0x2e002000;
}

impl Deref for ItemModel {
    type Target = Collector;

    fn deref(&self) -> &Collector {
        &self.parent
    }
}

impl DerefMut for ItemModel {
    fn deref_mut(&mut self) -> &mut Collector {
        &mut self.parent
    }
}

impl SubExtensions for ItemModel {
    const SUB_EXTENSIONS: &[&str] = &["Item"];
}

mod read {
    use std::sync::Arc;

    use crate::{
        NodeRef,
        class::{
            game::{item_model::ItemModel, item_placement_param::ItemPlacementParam},
            plug::{
                game_skin_and_folder::GameSkinAndFolder, item_variant_list::ItemVariantList,
                media_clip_list::MediaClipList,
            },
        },
        read::{
            BodyChunk, BodyChunks, BodyReader, Error, HeaderChunk, HeaderChunks, HeaderReader,
            ReadBody, Readable, error_unknown_chunk_version, read_body_chunks,
        },
    };

    impl Readable for ItemModel {}

    impl HeaderChunks for ItemModel {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            []
        }
    }

    impl ReadBody for ItemModel {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for ItemModel {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(8, Self::read_chunk_8),
                BodyChunk::new(9, Self::read_chunk_9),
                BodyChunk::new(12, Self::read_chunk_12),
                BodyChunk::new(18, Self::read_chunk_18),
                BodyChunk::new(21, Self::read_chunk_21),
                BodyChunk::new(25, Self::read_chunk_25),
                BodyChunk::new(26, Self::read_chunk_26),
                BodyChunk::new(28, Self::read_chunk_28),
                BodyChunk::new(30, Self::read_chunk_30),
                BodyChunk::new(31, Self::read_chunk_31),
                BodyChunk::new(32, Self::read_chunk_32),
                BodyChunk::skippable(37, Self::read_chunk_37),
                BodyChunk::skippable(38, Self::read_chunk_38),
                BodyChunk::skippable(39, Self::read_chunk_39),
            ]
        }
    }

    impl ItemModel {
        fn read_chunk_8(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _nadeo_skin_fids = r.list(|r| r.u32())?;

            Ok(())
        }

        fn read_chunk_9(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _cameras = r.list_with_version(|r| r.u32())?;

            Ok(())
        }

        fn read_chunk_12(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _race_interface_fid = r.u32()?;

            Ok(())
        }

        fn read_chunk_18(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _ground_point = r.vec3()?;
            let _painter_ground_margin = r.f32()?;
            let _orbital_center_height_from_ground = r.f32()?;
            let _orbital_radius_base = r.f32()?;
            let _orbital_preview_angle = r.f32()?;

            Ok(())
        }

        fn read_chunk_21(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _item_type = r.u32()?;

            Ok(())
        }

        fn read_chunk_25(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 15 {
                return Err(error_unknown_chunk_version(version));
            }

            let _phy_model_custom = r.u32()?;
            let _vis_model_custom = r.u32()?;
            let _default_weapon_name: Option<Arc<str>> = r.id()?;
            let _actions: Vec<()> = r.list(|r| todo!())?;
            let _default_cam = r.u32()?;
            let _entity_model_edition = r.u32()?;
            let _entity_model: NodeRef<ItemVariantList> = r.node_ref()?;
            r.u32()?;
            let _: Option<NodeRef<GameSkinAndFolder>> = r.node_ref()?;

            Ok(())
        }

        fn read_chunk_26(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_28(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(error_unknown_chunk_version(version));
            }

            let _default_placement: NodeRef<ItemPlacementParam> = r.node_ref()?;

            Ok(())
        }

        fn read_chunk_30(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 7 {
                return Err(error_unknown_chunk_version(version));
            }

            let archetype_ref = r.string()?;

            if archetype_ref.is_empty() {
                let _archetype_fid = r.u32()?;
            }

            r.string()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_31(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 12 {
                return Err(error_unknown_chunk_version(version));
            }

            let _waypoint_type = r.u32()?;
            let _disable_lightmap = r.bool32()?;
            r.u32()?;
            r.u8()?;
            let _: Option<Arc<MediaClipList>> = r.node_ref()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_32(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            let _icon_fid = r.string()?;
            r.bool8()?;

            Ok(())
        }

        fn read_chunk_37(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            r.bool32()?;

            Ok(())
        }

        fn read_chunk_38(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            r.list(|r| r.vec3())?;

            Ok(())
        }

        fn read_chunk_39(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            r.f32()?;

            Ok(())
        }
    }
}
