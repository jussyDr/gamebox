use std::io::Read;

use crate::{
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{self, BodyChunkEntry, BodyChunkReadFn, HeaderChunkEntry, ReadBody},
        Readable, Result,
    },
};

use super::Item;

impl Readable for Item {}

impl readable::Sealed for Item {
    #[allow(clippy::redundant_closure)]
    fn header_chunk_table<'a, R: Read>() -> &'a [HeaderChunkEntry<Self, R>] {
        &[
            HeaderChunkEntry {
                id: 0x2e001003,
                read_fn: |_, d| Collector::read_chunk_2e001003(&mut Collector, d),
            },
            HeaderChunkEntry {
                id: 0x2e001006,
                read_fn: |_, d| Collector::read_chunk_2e001006(&mut Collector, d),
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
    }
}

impl ReadBody for Item {
    fn default() -> Self {
        Self
    }

    #[allow(clippy::redundant_closure)]
    fn body_chunk_table<'a, R: Read>() -> &'a [BodyChunkEntry<Self, R>] {
        &[
            BodyChunkEntry {
                id: 0x2e001009,
                read_fn: BodyChunkReadFn::Normal(|_, d| {
                    Collector::read_chunk_2e001009(&mut Collector, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100b,
                read_fn: BodyChunkReadFn::Normal(|_, d| {
                    Collector::read_chunk_2e00100b(&mut Collector, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100c,
                read_fn: BodyChunkReadFn::Normal(|_, d| {
                    Collector::read_chunk_2e00100c(&mut Collector, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100d,
                read_fn: BodyChunkReadFn::Normal(|_, d| {
                    Collector::read_chunk_2e00100d(&mut Collector, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e001010,
                read_fn: BodyChunkReadFn::Normal(|_, d| {
                    Collector::read_chunk_2e001010(&mut Collector, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e001011,
                read_fn: BodyChunkReadFn::Normal(|_, d| {
                    Collector::read_chunk_2e001011(&mut Collector, d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e001012,
                read_fn: BodyChunkReadFn::Normal(|_, d| {
                    Collector::read_chunk_2e001012(&mut Collector, d)
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
        d.u32()?; // 15
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.node(0x2e026000, |d| {
            loop {
                let chunk_id = d.u32()?;

                match chunk_id {
                    0x2e026000 => ItemEntityModelEdition::read_chunk_2e026000(d)?,
                    0x2e026001 => ItemEntityModelEdition::read_chunk_2e026001(d)?,
                    0xfacade01 => break,
                    _ => todo!(),
                }
            }

            Ok(())
        })?;
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e00201a<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e00201c<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 5
        d.node(0x2e020000, |d| {
            loop {
                let chunk_id = d.u32()?;

                match chunk_id {
                    0x2e020000 => ItemPlacementParam::read_chunk_2e020000(d)?,
                    0x2e020001 => ItemPlacementParam::read_chunk_2e020001(d)?,
                    0x2e020004 => ItemPlacementParam::read_chunk_2e020004(d)?,
                    0x2e020005 => ItemPlacementParam::read_chunk_2e020005(d)?,
                    0xfacade01 => break,
                    _ => todo!(),
                }
            }

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_2e00201e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 7
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e00201f<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 12
        d.u32()?; // 3
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u8()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e002020<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 0
        d.u8()?; // 0

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

struct Collector;

impl Collector {
    fn read_chunk_2e001003<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.id_or_null()?; // null
        d.u32()?; // 26
        d.id()?; // "r-brwiQCRnOZ2PIHcM0Q8A"
        d.u32()?; // 8
        d.string()?; // "Items"
        d.u32()?; // 0xffffffff
        d.u32()?; // 8
        d.u16()?; // 1
        d.string()?; // "New Item"
        d.u8()?; // 3

        Ok(())
    }

    fn read_chunk_2e001006<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e001009<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.string()?; // "Items"
        d.u32()?; // 0
        d.id_or_null()?; // null

        Ok(())
    }

    fn read_chunk_2e00100b<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0xffffffff
        d.u32()?; // 26
        d.id()?; // "r-brwiQCRnOZ2PIHcM0Q8A"

        Ok(())
    }

    fn read_chunk_2e00100c<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.string()?; // "New Item"

        Ok(())
    }

    fn read_chunk_2e00100d<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.string()?; // "No Description"

        Ok(())
    }

    fn read_chunk_2e001010<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 4
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_2e001011<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 1
        d.u8()?; // 3

        Ok(())
    }

    fn read_chunk_2e001012<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

struct ItemPlacementParam;

impl ItemPlacementParam {
    fn read_chunk_2e020000<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // "piks"
        d.u32()?; // 50
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

    fn read_chunk_2e020001<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // "piks"
        d.u32()?; // 8
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e020004<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // "piks"
        d.u32()?; // 8
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e020005<R: Read, I, N: NodeStateMut>(
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // "piks"
        d.u32()?; // 52
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

struct ItemEntityModelEdition;

impl ItemEntityModelEdition {
    fn read_chunk_2e026000<R: Read, I: IdStateMut, N: NodeStateMut>(
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 7
        d.u32()?; // 1
        d.node(0x09003000, |d| {
            loop {
                let chunk_id = d.u32()?;

                match chunk_id {
                    0x09003003 => Crystal::read_chunk_09003003(d)?,
                    0x09003004 => Crystal::read_chunk_09003004(d)?,
                    0x09003005 => Crystal::read_chunk_09003005(d)?,
                    0x09003006 => Crystal::read_chunk_09003006(d)?,
                    0x09003007 => Crystal::read_chunk_09003007(d)?,
                    0xfacade01 => break,
                    _ => todo!(),
                }
            }

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

    fn read_chunk_2e026001<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // "piks"
        d.u32()?; // 8
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

struct Crystal;

impl Crystal {
    fn read_chunk_09003003<R: Read, I, N: NodeStateMut>(
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 2
        d.list(|d| {
            d.u32()?; // 0
            d.node(0x090fd000, |d| {
                loop {
                    let chunk_id = d.u32()?;

                    match chunk_id {
                        0x090fd000 => MaterialUserInst::read_chunk_090fd000(d)?,
                        0x090fd001 => MaterialUserInst::read_chunk_090fd001(d)?,
                        0x090fd002 => MaterialUserInst::read_chunk_090fd002(d)?,
                        0xfacade01 => break,
                        _ => todo!(),
                    }
                }

                Ok(())
            })?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_09003004<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // "piks"
        d.u32()?; // 12
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_09003005<R: Read, I: IdStateMut, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
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

    fn read_chunk_09003006<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
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

    fn read_chunk_09003007<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
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

struct MaterialUserInst;

impl MaterialUserInst {
    fn read_chunk_090fd000<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 11
        d.u8()?; // 1
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u16()?; // 4 | 22
        d.string()?; // "Stadium\Media\Material\TechnicsTrims" | "Stadium\Media\Material\TrackWallClips"
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_090fd001<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 5
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_090fd002<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

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
