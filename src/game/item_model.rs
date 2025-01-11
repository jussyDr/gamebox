//! Item model.

use std::{ops::Deref, sync::Arc};

use crate::{
    plug::{item_variant_list::ItemVariantList, Prefab},
    Class, NodeRef,
};

use super::{
    ctn::collector::Collector, BlockItem, CommonItemEntityModel, CommonItemEntityModelEdition,
};

/// Item model.
#[derive(Default)]
pub struct ItemModel {
    parent: Collector,
    ty: ItemModelType,
}

impl Class for ItemModel {
    const CLASS_ID: u32 = 0x2e002000;
}

impl Deref for ItemModel {
    type Target = Collector;

    fn deref(&self) -> &Collector {
        &self.parent
    }
}

impl ItemModel {
    /// Type.
    pub const fn ty(&self) -> &ItemModelType {
        &self.ty
    }
}

/// Item model type.
#[derive(Debug)]
pub enum ItemModelType {
    /// Block item.
    BlockItem(Arc<BlockItem>),
    /// Common item entity model.
    CommonItemEntityModel(Arc<CommonItemEntityModel>),
    /// Common item entity model edition.
    CommonItemEntityModelEdition(Arc<CommonItemEntityModelEdition>),
    /// Item variant list.
    ItemVariantList(Arc<ItemVariantList>),
    /// Prefab.
    Prefab(NodeRef<Prefab>),
}

impl Default for ItemModelType {
    fn default() -> Self {
        Self::CommonItemEntityModel(Default::default())
    }
}

mod read {
    use std::{
        io::{Read, Seek},
        marker::PhantomData,
        sync::Arc,
    };

    use crate::{
        game::{
            common_item_entity_model_edition::CommonItemEntityModelEdition,
            item_placement_param::ItemPlacementParam, BlockItem, CommonItemEntityModel,
        },
        plug::{item_variant_list::ItemVariantList, GameSkinAndFolder, MediaClipList, Prefab},
        read::{
            read_body_chunks,
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ErrorKind, ReadBody, Readable,
        },
        ExternalNodeRef, NodeRef,
    };

    use super::{ItemModel, ItemModelType};

    impl Readable for ItemModel {}

    impl Sealed for ItemModel {}

    impl HeaderChunks for ItemModel {
        fn parent(&mut self) -> Option<&mut impl HeaderChunks> {
            Some(&mut self.parent)
        }

        fn header_chunks<R: Read, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [
                HeaderChunk::new(0, Self::read_chunk_0),
                HeaderChunk::new(1, Self::read_chunk_1),
            ]
            .into_iter()
        }
    }

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
                BodyChunk::normal(26, Self::read_chunk_26),
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
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _item_type = r.u32()?;

            Ok(())
        }

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

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
            let _ground_point = r.vec3()?;
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

            if !matches!(version, 13 | 15) {
                return Err(Error::chunk_version(version));
            }

            let _default_weapon_name = r.id_or_null()?;
            let _phy_model_custom = r.u32()?;
            let _vis_custom_model = r.u32()?;
            let _actions = r.list(|r| r.u32())?;
            let _default_cam = r.u32()?;
            let model_edition = r.test_or_null(|r, class_id| match class_id {
                0x2e025000 => {
                    let mut block_item = BlockItem::default();
                    block_item.read_body(r)?;

                    self.ty = ItemModelType::BlockItem(Arc::new(block_item));

                    Ok(())
                }
                0x2e026000 => {
                    let mut entity_model_edition = CommonItemEntityModelEdition::default();
                    entity_model_edition.read_body(r)?;

                    self.ty =
                        ItemModelType::CommonItemEntityModelEdition(Arc::new(entity_model_edition));

                    Ok(())
                }
                _ => Err(Error::new(ErrorKind::Unsupported("".into()))),
            })?;

            if model_edition.is_none() {
                let ext = r.test_or_ext(|r, class_id| {
                    match class_id {
                        0x09145000 => {
                            let mut model = Prefab::default();
                            model.read_body(r)?;

                            self.ty = ItemModelType::Prefab(NodeRef::Internal(Arc::new(model)))
                        }
                        0x2e027000 => {
                            let mut model = CommonItemEntityModel::default();
                            model.read_body(r)?;

                            self.ty = ItemModelType::CommonItemEntityModel(Arc::new(model))
                        }
                        0x2f0bc000 => {
                            let mut model = ItemVariantList::default();
                            model.read_body(r)?;

                            self.ty = ItemModelType::ItemVariantList(Arc::new(model))
                        }
                        _ => return Err(Error::new(ErrorKind::Unsupported("".into()))),
                    }

                    Ok(())
                })?;

                if let Some(ext) = ext {
                    self.ty = ItemModelType::Prefab(NodeRef::External(ExternalNodeRef {
                        ancestor_level: ext.ancestor_level,
                        use_file: ext.use_file,
                        path: ext.path,
                        phantom: PhantomData,
                    }));
                }
            }

            r.u32()?;

            if version >= 15 {
                r.node_ref_or_null::<GameSkinAndFolder>()?;
            }

            Ok(())
        }

        fn read_chunk_26<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
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

            let _default_placement = r.node_ref::<ItemPlacementParam>()?;

            Ok(())
        }

        fn read_chunk_30<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 6 | 7) {
                return Err(Error::chunk_version(version));
            }

            let archetype_ref = r.string()?;

            if archetype_ref.is_empty() {
                r.u32()?;
            }

            r.string()?;

            if version >= 7 {
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_31(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 10..=12) {
                return Err(Error::chunk_version(version));
            }

            let _waypoint_type = r.u32()?;
            let _disable_lightmap = r.bool()?;
            r.u32()?;

            if version >= 11 {
                r.u8()?;
            }

            if version >= 12 {
                r.internal_node_ref_or_null::<MediaClipList>()?;
                r.u32()?;
            }

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

            r.list(|r| r.vec3())?;

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
    use std::io::Write;

    use crate::write::{
        writable,
        writer::{IdStateMut, NodeStateMut},
        Error, Writable, Writer,
    };

    use self::writable::{write_body_chunks, BodyChunks, HeaderChunk, HeaderChunks, WriteBody};

    use super::ItemModel;

    impl Writable for ItemModel {}

    impl writable::Sealed for ItemModel {}

    impl HeaderChunks for ItemModel {
        fn header_chunks<W, I, N>() -> impl ExactSizeIterator<Item = HeaderChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }

    impl WriteBody for ItemModel {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for ItemModel {
        fn body_chunks<W, I, N>() -> impl Iterator<Item = writable::BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
