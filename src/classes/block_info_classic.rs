use std::{
    io::{BufRead, Read},
    ops::{Deref, DerefMut},
};

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut},
    read::{
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
};

use super::collector::Collector;

#[derive(Default)]
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

impl<R: Read, I: IdStateMut, N> ReadBody<R, I, N> for BlockInfoClassic {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N> BodyChunks<R, I, N> for BlockInfoClassic {
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
        ]
        .into_iter()
    }
}

#[derive(Default)]
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

    fn read_chunk_35<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(&mut BlockInfoVariant, d)?;

        Ok(())
    }
}

struct BlockInfoVariant;

impl<R: Read, I, N> BodyChunks<R, I, N> for BlockInfoVariant {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x0315b002,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b003,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_3(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b004,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_4(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b005,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_5(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b006,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_6(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b007,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_7(n, d)),
            },
            BodyChunkEntry {
                id: 0x0315b008,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_8(n, d)),
            },
        ]
        .into_iter()
    }
}

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

    fn read_chunk_5<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 1
        d.u32()?; // 0
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

    fn read_chunk_8<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
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
        d.u32()?; // 0
        d.string()?; // "Variant Ground"

        Ok(())
    }

    fn read_chunk_9<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        println!("{:02X?}", d.bytes(144)?);

        Ok(())
    }
}
