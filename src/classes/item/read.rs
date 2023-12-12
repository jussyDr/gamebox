use std::io::Read;

use crate::{
    class::Class,
    classes::collector::Collector,
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        read_body,
        readable::{
            BodyChunkEntry, BodyChunkReadFn, HeaderChunkEntry, ReadBody, ReadHeader, Sealed,
        },
        Readable, Result,
    },
};

use super::{
    IndexBuffer, Indices, Item, ItemEntityModel, Material, MaterialUserInst, Mesh, Solid2Model,
    VertexStream, Visual, Visual3D, VisualIndexed, VisualIndexedTriangles,
};

impl Readable for Item {}

impl Sealed for Item {}

impl ReadHeader for Item {
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
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
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

    fn read_chunk_2e002019<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 13 | 15
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.node_or_null(0x2e026000, |d| {
            let mut node = ItemEntityModelEdition::default();
            read_body(&mut node, d)?;

            Ok(())
        })?;
        let entity_model = d
            .node_or_null(0x2e027000, |d| {
                let mut node = ItemEntityModel::default();
                read_body(&mut node, d)?;

                Ok(node)
            })?
            .cloned();
        d.u32()?; // 0xffffffff

        if let Some(entity_model) = entity_model {
            self.layers = entity_model.solid_to_model.layers.clone();
            self.materials = entity_model.solid_to_model.materials.clone();
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
        d.node(0x2e020000, |d| {
            let mut node = ItemPlacementParam::default();
            read_body(&mut node, d)?;

            Ok(())
        })?;

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

    fn read_chunk_2e00201f<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let version = d.u32()?; // 11 | 12
        d.u32()?; // 3
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u8()?; // 0

        if version >= 12 {
            d.u32()?; // 0xffffffff
            d.u32()?; // 0xffffffff
        }

        Ok(())
    }

    fn read_chunk_2e002020<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 0
        d.u8()?; // 0

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
    const CLASS_ID: u32 = 0x2e020000;
}

impl Default for ItemPlacementParam {
    fn default() -> Self {
        Self
    }
}

impl ReadBody for ItemPlacementParam {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
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
        d.u32()?; // 0
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
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e020005<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.node(0x09187000, |d| {
            d.u32()?; // 10
            d.u32()?; // 0xffffffff
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 1
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.f32()?; // 1.0
            d.u32()?; // 0
            d.u32()?; // 0

            Ok(())
        })?;

        Ok(())
    }
}

impl ReadBody for ItemEntityModel {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
        d.u32()?; // 4
        self.solid_to_model = d
            .node(0x09159000, |d| {
                d.u32()?; // 3
                let solid_to_model = d
                    .node(0x090bb000, |d| {
                        let mut node = Solid2Model::default();
                        read_body(&mut node, d)?;

                        Ok(node)
                    })?
                    .clone();
                d.u8()?; // 1
                d.u32()?; // 0xffffffff
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

                Ok(solid_to_model)
            })?
            .clone();

        Ok(())
    }
}

struct ItemEntityModelEdition;

impl Class for ItemEntityModelEdition {
    const CLASS_ID: u32 = 0x2e026000;
}

impl Default for ItemEntityModelEdition {
    fn default() -> Self {
        Self
    }
}

impl ReadBody for ItemEntityModelEdition {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
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
    fn read_chunk_2e026000<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 7
        d.u32()?; // 1
        d.node(0x09003000, |d| {
            let mut node = Crystal::default();
            read_body(&mut node, d)?;

            Ok(())
        })?;
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
    const CLASS_ID: u32 = 0x09003000;
}

impl Default for Crystal {
    fn default() -> Self {
        Self
    }
}

impl ReadBody for Crystal {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
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
    fn read_chunk_09003003<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 2
        d.list(|d| {
            d.u32()?; // 0
            d.node(0x090fd000, |d| {
                let mut node = MaterialUserInst::default();
                read_body(&mut node, d)?;

                Ok(())
            })?;

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

    fn read_chunk_09003005<R: Read, I: IdStateMut, N>(
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

impl ReadBody for MaterialUserInst {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x090fd000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_090fd000(n, d)),
            },
            BodyChunkEntry {
                id: 0x090fd001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_090fd001(n, d)),
            },
            BodyChunkEntry {
                id: 0x090fd002,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_090fd002(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MaterialUserInst {
    fn read_chunk_090fd000<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 11
        let uses_game_material = d.bool8()?;
        d.id_or_null()?; // "TM_wiuehrfsd"
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u16()?; // 4 | 22
        if uses_game_material {
            d.string()?; // "Stadium\Media\Material\TechnicsTrims" | "Stadium\Media\Material\TrackWallClips"
        } else {
            d.id()?; // "CustomConcrete"
        }
        d.list(|d| {
            d.id()?; // "TargetColor"
            d.id()?; // "Real"
            d.u32()?; // 3

            Ok(())
        })?;
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_090fd001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 5
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_090fd002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

impl ReadBody for Solid2Model {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x090bb000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_090bb000(n, d)),
            },
            BodyChunkEntry {
                id: 0x090bb002,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_090bb002(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Solid2Model {
    fn read_chunk_090bb000<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 30
        d.u32()?; // 0xffffffff
        d.u32()?; // 2
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 1
        d.u32()?; // 1
        d.u32()?; // 1
        d.u32()?; // 0xffffffff
        d.u32()?; // 1
        d.u32()?; // 10
        self.layers = d.list(|d| {
            let visual_indexed_triangles = d.node(0x0901e000, |d| {
                let mut node = VisualIndexedTriangles::default();
                read_body(&mut node, d)?;

                Ok(node)
            })?;

            Ok(Mesh {
                positions: visual_indexed_triangles.vertices.positions.clone(),
                indices: visual_indexed_triangles.indices.clone(),
            })
        })?;
        d.u32()?; // 0
        let num_materials = d.u32()?; // 2
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 1
        d.u32()?; // 1
        d.u32()?; // 1
        d.f32()?; // 73.47571
        d.u32()?; // 1
        d.f32()?; // 0.011813663
        d.f32()?; // 0.12343697
        d.f32()?; // 0.99153054
        d.f32()?; // 0.98973596
        d.u32()?; // 0xffff7f7f
        d.u32()?; // 0xffff7f7f
        d.u32()?; // 0xffff7f7f
        d.u32()?; // 0xffff7f7f
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xd05ebb50
        d.u32()?; // 0x01d74f56
        d.u32()?; // 0
        d.string()?; // "Stadium\Media\Material\"
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 1
        d.string()?; // "NadeoImporter Item Items/palm_trees/big_palm_trees/big_palm_tree_low.Item.xml"
        d.u32()?; // 1
        d.u32()?; // 0
        self.materials = d.repeat(num_materials as usize, |d| {
            let material = d
                .node(0x090fd000, |d| {
                    let mut node = MaterialUserInst::default();
                    read_body(&mut node, d)?;

                    Ok(node)
                })?
                .clone();
            d.u32()?; // 0

            Ok(material.material.clone())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.f32()?; // 1.0
        d.f32()?; // 1.0
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_090bb002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

impl ReadBody for VisualIndexedTriangles {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x09006001,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_09006001(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x09006005,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_09006005(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x09006009,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_09006009(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0900600b,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_0900600b(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0900600f,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_0900600f(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x09006010,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_09006010(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0902c002,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual3D::read_chunk_0902c002(&mut n.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0902c004,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual3D::read_chunk_0902c004(&mut n.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0906a001,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    VisualIndexed::read_chunk_0906a001(&mut n.parent, d)
                }),
            },
        ]
        .into_iter()
    }
}

impl Visual {
    fn read_chunk_09006001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_09006005<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09006009<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0900600b<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0900600f<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 6
        d.u32()?; // 56
        d.u32()?; // 0
        d.u32()?; // 180
        d.u32()?; // 1
        self.vertices = d
            .node(0x09056000, |d| {
                let mut node = VertexStream::default();
                read_body(&mut node, d)?;

                Ok(node)
            })?
            .clone();
        d.u32()?; // 0
        d.f32()?; // 12.703503
        d.f32()?; // 15.202776
        d.f32()?; // 1.7036213
        d.f32()?; // 14.653503
        d.f32()?; // 17.410307
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09006010<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

impl Visual3D {
    fn read_chunk_0902c002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0902c004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

impl VisualIndexed {
    fn read_chunk_0906a001<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1

        let mut node = IndexBuffer::default();
        read_body(&mut node, d)?;

        self.indices = node.indices;

        Ok(())
    }
}

impl ReadBody for IndexBuffer {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x09057001,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09057001(n, d)),
        }]
        .into_iter()
    }
}

impl IndexBuffer {
    fn read_chunk_09057001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        let num_indices = d.u32()?;
        let mut current_index = 0;
        self.indices = Indices::U16(d.repeat(num_indices as usize, |d| {
            let offset = d.i16()?;
            current_index = (current_index as i32 + offset as i32) as u16;

            Ok(current_index)
        })?);

        Ok(())
    }
}

impl ReadBody for VertexStream {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x09056000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09056000(n, d)),
        }]
        .into_iter()
    }
}

impl VertexStream {
    fn read_chunk_09056000<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        let vertex_count = d.u32()?; // 180
        d.u32()?; // 3
        d.u32()?; // 0xffffffff
        let vertex_attributes = d.list(|d| {
            let flags = d.u32()?;
            let offset = d.u32()?; // 0 | 0x30 | 0x40

            if offset != 0 {
                d.u32()?;
            }

            Ok(VertexAttribute {
                kind: (flags & 0x1FF) as u16,
                format: ((flags >> 9) & 0x1FF) as u16,
            })
        })?;
        d.u32()?; // 1
        for vertex_attribute in vertex_attributes {
            match vertex_attribute {
                VertexAttribute { kind: 0, format: 2 } => {
                    self.positions = d.repeat(vertex_count as usize, |d| {
                        let x = d.f32()?;
                        let y = d.f32()?;
                        let z = d.f32()?;

                        Ok([x, y, z])
                    })?;
                }
                VertexAttribute {
                    kind: 10,
                    format: 1,
                } => {
                    self.texcoords = d.repeat(vertex_count as usize, |d| {
                        d.f32()?;
                        d.f32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute {
                    kind: 11,
                    format: 1,
                } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.f32()?;
                        d.f32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute {
                    kind: 5,
                    format: 14,
                } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.u32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute {
                    kind: 18,
                    format: 14,
                } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.u32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute {
                    kind: 20,
                    format: 14,
                } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.u32()?;

                        Ok(())
                    })?;
                }
                _ => todo!(),
            }
        }

        Ok(())
    }
}

fn read_compact_index<R: Read, I, N>(d: &mut Deserializer<R, I, N>, num_items: u32) -> Result<u32> {
    if num_items < u8::MAX as u32 {
        let index = d.u8()?;
        Ok(index as u32)
    } else if num_items < u16::MAX as u32 {
        let index = d.u16()?;
        Ok(index as u32)
    } else {
        d.u32()
    }
}

struct VertexAttribute {
    kind: u16,
    format: u16,
}
