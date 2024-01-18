//! Types used for reading and writing [Macroblock] nodes.

use std::io::Read;

use crate::{
    classes::{
        traits_metadata::TraitsMetadata, waypoint_special_property::WaypointSpecialProperty,
        zone_genealogy::ZoneGenealogy,
    },
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
};

use super::collector::Collector;

/// Node type corresponding to GameBox files with the extension `Macroblock.Gbx`.
#[derive(Default)]
pub struct Macroblock {
    parent: Collector,
}

impl Class for Macroblock {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 269);
}

impl Readable for Macroblock {}

impl Sealed for Macroblock {
    fn read(
        reader: impl Read,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for Macroblock {
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
        ]
        .into_iter()
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Macroblock {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Macroblock {
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
                id: 0x0310d000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_1(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d002,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d006,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_6(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d008,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_8(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d00b,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_11(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d00c,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_12(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d00e,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_14(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d00f,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_15(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d010,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_16(n, d)),
            },
            BodyChunkEntry {
                id: 0x0310d011,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_17(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Macroblock {
    fn read_chunk_0<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.list(|d| {
            let version = d.u32()?; // 8

            if !matches!(version, 5 | 8) {
                return Err("".into());
            }

            d.id()?; // "TechnicsScreen2x3Straight"
            d.null_id()?;
            d.id()?; // "Nadeo"
            d.u32()?;
            d.u32()?;
            d.internal_node_ref_or_null::<WaypointSpecialProperty>()?;
            if version == 5 {
                d.u32()?; // 0xffffffff
            }
            if version == 8 {
                d.u16()?; // 0
            }

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_1<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_6<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_8<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 10
        d.list(|d| {
            d.internal_node_ref::<AutoTerrain>()?;

            Ok(())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0 | 2

        Ok(())
    }

    fn read_chunk_11<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 14
        d.node::<TraitsMetadata>()?;

        Ok(())
    }

    fn read_chunk_12<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2

        Ok(())
    }

    fn read_chunk_14<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 2
        d.list(|d| {
            let version = d.u32()?; // 10

            if !matches!(version, 10 | 12) {
                return Err("".into());
            }

            d.id()?; // "TrackBarrier4m"
            d.null_id()?;
            d.id()?; // "Nadeo"
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0xffffffff
            d.f32()?;
            d.f32()?;
            d.f32()?;
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u16()?; // 0
            d.u32()?; // 0xffffffff
            d.f32()?;
            d.u32()?; // 0xffffffff
            d.u32()?; // 0xffffffff
            d.u32()?; // 0xffffffff
            if version >= 12 {
                d.u16()?; // 0
                d.u8()?; // 0
            }

            Ok(())
        })?;
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_15<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 3
        d.u32()?; // 1
        d.u32()?; // 3
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_16<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_17<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 32
        d.u32()?; // 30
        d.u32()?; // 10
        d.u32()?; // 15
        d.u32()?; // 3
        d.u32()?; // 1
        d.u32()?; // 3
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }
}

#[derive(Default)]
struct AutoTerrain;

impl Class for AutoTerrain {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 288);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for AutoTerrain {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for AutoTerrain {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03120001,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_1(n, d)),
        }]
        .into_iter()
    }
}

impl AutoTerrain {
    fn read_chunk_1<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.internal_node_ref::<ZoneGenealogy>()?;

        Ok(())
    }
}
