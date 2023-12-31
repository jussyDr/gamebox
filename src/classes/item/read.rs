use std::{
    io::{BufRead, Read, Seek},
    rc::Rc,
};

use crate::{
    classes::{
        collector::Collector,
        static_object_model::{MaterialUserInst, StaticObjectModel},
    },
    common::{read_compact_index, Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateRef, NodeStateRef},
    read::{
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
};

use super::{Item, ItemEntityModel};

impl Readable for Item {}

impl Sealed for Item {
    fn read(
        reader: impl BufRead + Seek,
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

impl ReadBody for Item {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for Item {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

    fn read_chunk_2e002015<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_2e002019<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?; // 13 | 15
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.node_ref_or_null::<ItemEntityModelEdition>()?;
        d.any_node_ref_or_null(|d, class_id| match class_id {
            0x2e027000 => {
                let mut node = ItemEntityModel::default();
                read_body_chunks(&mut node, d)?;

                Ok(Rc::new(node))
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

                Ok(Rc::new(()))
            }
            _ => Err("unknown entity model type".into()),
        })?;
        d.u32()?; // 0xffffffff

        if version >= 15 {
            d.u32()?; // 0xffffffff
        }

        Ok(())
    }

    fn read_chunk_2e00201a<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e00201c<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 5
        d.node_ref::<ItemPlacementParam>()?;

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

    fn read_chunk_2e00201f<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?; // 11 | 12
        d.u32()?; // 3
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u8()?; // 0

        if version >= 12 {
            d.internal_node_ref_or_null::<MediaClipList>()?;
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

struct ItemPlacementParam;

impl Class for ItemPlacementParam {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 32);
}

impl Default for ItemPlacementParam {
    fn default() -> Self {
        Self
    }
}

impl ReadBody for ItemPlacementParam {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for ItemPlacementParam {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
        d.u32()?; // 6
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0

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

    fn read_chunk_2e020005<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.node_ref::<ItemPlacement>()?;

        Ok(())
    }
}

impl BodyChunks for ItemEntityModel {
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x2e027000,
            read_fn: BodyChunkReadFn::Normal(Self::read_chunk_2e027000),
        }]
        .into_iter()
    }
}

impl ItemEntityModel {
    fn read_chunk_2e027000<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 4
        d.internal_node_ref::<StaticObjectModel>()?;

        Ok(())
    }
}

struct ItemEntityModelEdition;

impl Class for ItemEntityModelEdition {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 38);
}

impl Default for ItemEntityModelEdition {
    fn default() -> Self {
        Self
    }
}

impl ReadBody for ItemEntityModelEdition {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for ItemEntityModelEdition {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
    fn read_chunk_2e026000<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 7
        d.u32()?; // 1
        d.internal_node_ref::<Crystal>()?;
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
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e026001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

struct Crystal;

impl Class for Crystal {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 3);
}

impl Default for Crystal {
    fn default() -> Self {
        Self
    }
}

impl ReadBody for Crystal {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for Crystal {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x09003003,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09003003(n, d)),
            },
            BodyChunkEntry {
                id: 0x09003004,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_09003004(n, d)),
            },
            BodyChunkEntry {
                id: 0x09003005,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09003005(n, d)),
            },
            BodyChunkEntry {
                id: 0x09003006,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09003006(n, d)),
            },
            BodyChunkEntry {
                id: 0x09003007,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09003007(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Crystal {
    fn read_chunk_09003003<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 2
        d.list(|d| {
            d.u32()?; // 0
            d.internal_node_ref::<MaterialUserInst>()?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_09003004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_09003005<R: Read, I: IdStateRef, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 2
        d.u32()?; // 0
        d.id()?; // "Layer0"
        d.string()?; // "Geometry"
        d.u32()?; // 1
        d.u32()?; // 1
        d.u32()?; // 37
        d.u32()?; // 4
        d.u32()?; // 3
        d.u32()?; // 4
        d.f32()?; // 64.0
        d.u32()?; // 2
        d.f32()?; // 128.0
        d.u32()?; // 1
        d.f32()?; // 192.0
        d.u32()?; // 0
        let num_groups = d.u32()?;
        d.repeat(num_groups as usize, |d| {
            d.u32()?; // 0
            d.u8()?; // 1
            d.u32()?; // 0xffffffff
            d.string()?; // "" | "part"
            d.u32()?; // 0xffffffff
            d.list(|d| {
                d.u32()?;

                Ok(())
            })?;

            Ok(())
        })?;
        d.u8()?; // 1
        d.u32()?; // 1
        d.u32()?; // 35
        let num_vertices = d.u32()?;
        d.repeat(num_vertices as usize, |d| {
            d.f32()?;
            d.f32()?;
            d.f32()?;

            Ok(())
        })?;
        d.u32()?; // 0x330
        d.u32()?; // 0
        let num_faces = d.u32()?; // 0x144
        d.list(|d| {
            d.f32()?;
            d.f32()?;

            Ok(())
        })?;
        let num_face_indices = d.u32()?;
        d.repeat(num_face_indices as usize, |d| {
            read_compact_index(d, num_face_indices)?;

            Ok(())
        })?;
        d.repeat(num_faces as usize, |d| {
            let index_count = d.u8()? + 3;
            d.repeat(index_count as usize, |d| {
                read_compact_index(d, num_vertices)?;

                Ok(())
            })?;
            read_compact_index(d, 64)?;
            read_compact_index(d, num_groups)?;

            Ok(())
        })?;
        d.u32()?; // 0
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;
        d.u32()?; // 1
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_09003006<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;
        let len = d.u32()?;
        d.repeat(len as usize, |d| {
            read_compact_index(d, len)?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_09003007<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 3
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.f32()?; // 2.0
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Default)]
struct MediaClipList;

impl Class for MediaClipList {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 393);
}

impl ReadBody for MediaClipList {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaClipList {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

#[derive(Default)]
struct ItemPlacement;

impl Class for ItemPlacement {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 391);
}

impl ReadBody for ItemPlacement {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
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
