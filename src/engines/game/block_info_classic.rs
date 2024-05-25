use std::{
    io::{BufRead, Read},
    ops::{Deref, DerefMut},
};

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    engines::game_data::collector::Collector,
    read::{
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
};

/// Classic block info.
#[derive(Default, Debug)]
pub struct BlockInfoClassic {
    parent: BlockInfo,
}

impl Deref for BlockInfoClassic {
    type Target = BlockInfo;

    fn deref(&self) -> &BlockInfo {
        &self.parent
    }
}

impl DerefMut for BlockInfoClassic {
    fn deref_mut(&mut self) -> &mut BlockInfo {
        &mut self.parent
    }
}

impl Class for BlockInfoClassic {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 81);
}

impl Readable for BlockInfoClassic {}

impl Sealed for BlockInfoClassic {
    fn read(
        reader: impl Read,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for BlockInfoClassic {
    fn header_chunks<R: BufRead>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>> {
        [].into_iter()
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for BlockInfoClassic {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for BlockInfoClassic {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x2e001009,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e001009(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100b,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e00100b(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100c,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e00100c(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e00100d,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e00100d(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e001010,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e001010(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e001011,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e001011(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x2e001012,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Collector::read_chunk_2e001012(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e00f,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_15(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e013,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_19(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e017,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_23(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e020,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_32(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e023,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_35(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e026,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_38(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e027,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_39(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e028,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_40(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e029,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_41(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e02a,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_42(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e02b,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_43(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e02c,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_44(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e02f,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_47(n.deref_mut(), d)
                }),
            },
            BodyChunkEntry {
                id: 0x0304e031,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfo::read_chunk_49(n.deref_mut(), d)
                }),
            },
        ]
        .into_iter()
    }
}

/// Block info.
#[derive(Default, Debug)]
pub struct BlockInfo {
    parent: Collector,
}

impl Deref for BlockInfo {
    type Target = Collector;

    fn deref(&self) -> &Collector {
        &self.parent
    }
}

impl DerefMut for BlockInfo {
    fn deref_mut(&mut self) -> &mut Collector {
        &mut self.parent
    }
}

impl BlockInfo {
    fn read_chunk_15<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_19<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_23<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_32<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 8
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_35<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(&mut BlockInfoVariantGround::default(), d)?;
        read_body_chunks(&mut BlockInfoVariantGround::default(), d)?;

        Ok(())
    }

    fn read_chunk_38<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 3

        Ok(())
    }

    fn read_chunk_39<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 10
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_40<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0xffffffff
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_41<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_42<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_43<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_44<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 10
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_47<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1
        d.u8()?; // 0
        d.u16()?; // 2

        Ok(())
    }

    fn read_chunk_49<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }
}

#[derive(Default)]
struct BlockInfoVariant;

impl BlockInfoVariant {
    fn read_chunk_2<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_3<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.u32()?; // 0xffffffff
        d.u16()?; // 0
        d.u8()?; // 0xff

        Ok(())
    }

    fn read_chunk_4<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u16()?; // 0

        Ok(())
    }

    fn read_chunk_5<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 1
        if d.bool32()? {
            d.internal_node_ref::<BlockInfoMobil>()?;
        }
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_6<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 11
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_7<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_8<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 2
        if d.bool32()? {
            d.internal_node_ref::<BlockUnitInfo>()?;
        }
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.string()?; // "Variant Ground"

        Ok(())
    }

    fn read_chunk_9<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_10<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_11<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_13<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0xffffffff

        Ok(())
    }
}

#[derive(Default)]
struct BlockInfoVariantGround {
    parent: BlockInfoVariant,
}

impl Deref for BlockInfoVariantGround {
    type Target = BlockInfoVariant;

    fn deref(&self) -> &BlockInfoVariant {
        &self.parent
    }
}

impl DerefMut for BlockInfoVariantGround {
    fn deref_mut(&mut self) -> &mut BlockInfoVariant {
        &mut self.parent
    }
}

impl<R: Read, I, N: NodeStateMut> BodyChunks<R, I, N> for BlockInfoVariantGround {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x0315b002,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    BlockInfoVariant::read_chunk_2(n, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0315b003,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_3(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b004,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_4(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b005,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_5(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b006,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_6(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b007,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_7(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b008,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_8(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b009,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_9(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b00a,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_10(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b00b,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_11(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b00d,
                read_fn: BodyChunkReadFn::Normal(|n, d| BlockInfoVariant::read_chunk_13(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315c001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_1(n, d)),
            },
        ]
        .into_iter()
    }
}

impl BlockInfoVariantGround {
    fn read_chunk_1<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.u32()?; // 10
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct BlockInfoMobil;

impl Class for BlockInfoMobil {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 290);
}

impl<R: Read, I, N> ReadBody<R, I, N> for BlockInfoMobil {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for BlockInfoMobil {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03122002,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2(n, d)),
            },
            BodyChunkEntry {
                id: 0x03122003,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_3(n, d)),
            },
            BodyChunkEntry {
                id: 0x03122004,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_4(n, d)),
            },
        ]
        .into_iter()
    }
}

impl BlockInfoMobil {
    fn read_chunk_2<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 10
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_3<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 23
        d.u32()?; // 1
        d.u8()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 2
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 3
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u8()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_4<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 10
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct BlockUnitInfo;

impl Class for BlockUnitInfo {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 54);
}

impl<R: Read, I, N> ReadBody<R, I, N> for BlockUnitInfo {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for BlockUnitInfo {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03036000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
            },
            BodyChunkEntry {
                id: 0x03036001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_1(n, d)),
            },
            BodyChunkEntry {
                id: 0x03036002,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2(n, d)),
            },
            BodyChunkEntry {
                id: 0x03036004,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_4(n, d)),
            },
            BodyChunkEntry {
                id: 0x03036005,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_5(n, d)),
            },
            BodyChunkEntry {
                id: 0x03036007,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_7(n, d)),
            },
            BodyChunkEntry {
                id: 0x0303600c,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_12(n, d)),
            },
            BodyChunkEntry {
                id: 0x0303600d,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_13(n, d)),
            },
        ]
        .into_iter()
    }
}

impl BlockUnitInfo {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_1<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff
        d.u32()?; // 5
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_4<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_5<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_7<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_12<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.u32()?; // 585
        d.u32()?; // 4
        d.u32()?; // 4
        d.u32()?; // 4
        d.u32()?; // 4
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_13<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}
