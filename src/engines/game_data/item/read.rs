use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeRef, NodeStateMut},
    engines::{
        game_data::collector::Collector,
        plug::{static_object_model::StaticObjectModel, surface::Surface},
    },
    read::{
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
};

use super::{
    BlockItem, Crystal, Item, ItemEntityModel, ItemEntityModelEdition, ItemModel, ItemPlacement,
    ItemPlacementParam,
};

impl Readable for Item {}

impl Sealed for Item {
    fn read(
        reader: impl Read,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for Item {
    #[allow(clippy::redundant_closure)]
    fn header_chunks<R: Read>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>> {
        [
            HeaderChunkEntry {
                id: 0x2e001003,
                read_fn: |n: &mut Self, d| Collector::read_chunk_2e001003(&mut n.parent, d),
            },
            HeaderChunkEntry {
                id: 0x2e001004,
                read_fn: |n: &mut Self, d| Collector::read_chunk_2e001004(&mut n.parent, d),
            },
            HeaderChunkEntry {
                id: 0x2e001006,
                read_fn: |n: &mut Self, d| Collector::read_chunk_2e001006(&mut n.parent, d),
            },
            HeaderChunkEntry {
                id: 0x2e002000,
                read_fn: |n, d| Self::read_chunk_2e002000(n, d),
            },
            HeaderChunkEntry {
                id: 0x2e002001,
                read_fn: |n, d| Self::read_chunk_2e002001(n, d),
            },
        ]
        .into_iter()
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Item {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Item {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x2e001009,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e001009(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100b,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e00100b(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100c,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e00100c(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100d,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e00100d(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100e,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e00100e(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e001010,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e001010(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e001011,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e001011(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e001012,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e001012(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e002008,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e002008(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002009,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e002009(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e00200c,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e00200c(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002012,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e002012(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002013,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e002013(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002015,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e002015(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002019,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e002019(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e00201a,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e00201a(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e00201c,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e00201c(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e00201e,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e00201e(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e00201f,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e00201f(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002020,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e002020(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002021,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e002021(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002023,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e002023(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002024,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e002024(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002025,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e002025(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002026,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e002026(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e002027,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e002027(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Item {
    fn read_chunk_2e002000<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_2e002001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e002008<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 7
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e002009<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 10
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e00200c<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e002012<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // -1.0
        d.f32()?; // 0.15

        Ok(())
    }

    fn read_chunk_2e002013<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e002015<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_2e002019<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?;

        if !matches!(version, 12 | 13 | 15) {
            return Err("".into());
        }
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        let model_edition = d.any_unique_internal_node_ref_or_null(|d, class_id| {
            let model = match class_id {
                0x2e025000 => {
                    let mut node = BlockItem::default();
                    BlockItem::read_body(&mut node, d)?;
                    ItemModel::Block(node)
                }
                0x2e026000 => {
                    let mut node = ItemEntityModelEdition::default();
                    ItemEntityModelEdition::read_body(&mut node, d)?;
                    ItemModel::EntityEdition(node)
                }
                _ => return Err("".into()),
            };

            Ok(model)
        })?;
        if version >= 13 {
            let model = d.any_unique_node_ref_or_null(|d, class_id| {
                let model = match class_id {
                    0x2e027000 => {
                        let mut node = ItemEntityModel::default();
                        read_body_chunks(&mut node, d)?;

                        ItemModel::Entity(node)
                    }
                    0x2f0bc000 => {
                        d.u32()?; // 1
                        d.list(|d| {
                            d.list(|d| {
                                d.string()?; // "Type" | "Size" | "Placement" | "MatModifier"
                                d.string()?; // "SpringTree" | "Medium" | "Wild" | "Grass"

                                Ok(())
                            })?;
                            d.u32()?; // 2
                            d.u32()?; // 1

                            Ok(())
                        })?;

                        ItemModel::VariantList
                    }
                    _ => return Err("unknown entity model type".into()),
                };

                Ok(model)
            })?;

            match model_edition {
                None => {
                    d.u32()?; // 0xffffffff
                }
                Some(model) => self.model = model,
            }

            if version >= 15 {
                d.u32()?; // 0xffffffff
            }

            if let Some(NodeRef::Internal { node }) = model {
                self.model = node;
            }
        }

        Ok(())
    }

    fn read_chunk_2e00201a<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e00201c<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 5
        d.unique_node_ref::<ItemPlacementParam>()?;

        Ok(())
    }

    fn read_chunk_2e00201e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let version = d.u32()?; // 6 | 7
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0

        if version >= 7 {
            d.u32()?; // 0xffffffff
        }

        Ok(())
    }

    fn read_chunk_2e00201f<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?;

        if !matches!(version, 8 | 10..=12) {
            return Err("".into());
        }

        d.u32()?; // 3
        d.u32()?; // 0

        if version >= 10 {
            d.u32()?; // 0xffffffff
        }

        if version >= 11 {
            d.u8()?; // 0
        }

        if version >= 12 {
            d.unique_internal_node_ref_or_null::<MediaClipList>()?;
            d.u32()?; // 0xffffffff
        }

        Ok(())
    }

    fn read_chunk_2e002020<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.string()?; // "" | "\Items\IconFall.dds"
        d.u8()?; // 0 | 1

        Ok(())
    }

    fn read_chunk_2e002021<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e002023<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u8()?; // 0

        Ok(())
    }

    fn read_chunk_2e002024<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e002025<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e002026<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e002027<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for ItemPlacementParam {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for ItemPlacementParam {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x2e020000,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e020000(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e020001,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e020001(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e020003,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e020003(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e020004,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e020004(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e020005,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e020005(n, d)),
            },
        ]
        .into_iter()
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for BlockItem {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for BlockItem {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x2e025000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e025001,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_1(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e025002,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e025003,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_3(n, d)),
            },
        ]
        .into_iter()
    }
}

impl BlockItem {
    fn read_chunk_0<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        let _archetype = d.id()?; // "GateFinish"
        d.u32()?; // 26
        self.variants = d.list(|d| {
            d.u32()?; // 0
            let crystal = d.internal_node_ref_or_null::<Crystal>()?;

            Ok(crystal)
        })?;

        Ok(())
    }

    fn read_chunk_1<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.read_to_end()?;

        Ok(())
    }

    fn read_chunk_2<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_3<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        for _ in 0..self.variants.len() {
            d.u8()?;
        }

        Ok(())
    }
}

impl ItemPlacementParam {
    fn read_chunk_2e020000<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u16()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.f32()?; // -1.0

        Ok(())
    }

    fn read_chunk_2e020001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e020003<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        let version = d.u32()?;

        if !matches!(version, 5 | 6 | 8 | 10) {
            return Err("".into());
        }

        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0
        if version >= 8 {
            d.u32()?; // 0
            d.u32()?; // 0
            d.f32()?; // 1.0
            d.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_2e020004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_2e020005<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.unique_node_ref::<ItemPlacement>()?;

        Ok(())
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for ItemEntityModel {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x2e027000,
            read_fn: BodyChunkReadFn::Normal(Self::read_chunk_2e027000),
        }]
        .into_iter()
    }
}

impl ItemEntityModel {
    fn read_chunk_2e027000<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?;

        if !matches!(version, 4 | 5) {
            return Err("".into());
        }

        self.static_object_model = d.unique_internal_node_ref::<StaticObjectModel>()?;
        d.internal_node_ref_or_null::<Surface>()?;
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        if version >= 5 {
            d.u8()?; // 0
        }

        Ok(())
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for ItemEntityModelEdition {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for ItemEntityModelEdition {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x2e026000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e026000(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e026001,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e026001(n, d)),
            },
        ]
        .into_iter()
    }
}

impl ItemEntityModelEdition {
    fn read_chunk_2e026000<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?;

        if !matches!(version, 5 | 7 | 8) {
            return Err("".into());
        }

        d.u32()?; // 1
        self.crystal = d.unique_internal_node_ref::<Crystal>()?;
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0x3e8
        if version == 7 {
            d.u32()?; // 0xffffffff
        }

        Ok(())
    }

    fn read_chunk_2e026001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct MediaClipList;

impl Class for MediaClipList {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 393);
}

impl<R: Read, I, N> ReadBody<R, I, N> for MediaClipList {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaClipList {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x09189000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09189000(n, d)),
        }]
        .into_iter()
    }
}

impl MediaClipList {
    fn read_chunk_09189000<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 6

        Ok(())
    }
}

impl<R: Read, I: IdStateMut, N> ReadBody<R, I, N> for ItemPlacement {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 10
        d.id_or_null()?; // "1x1"
        d.list(|d| {
            d.id()?;

            Ok(())
        })?;
        d.u32()?; // 0 | 1
        d.u32()?; // 0 | 1
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.list(|d| {
                d.id()?;

                Ok(())
            })?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}
