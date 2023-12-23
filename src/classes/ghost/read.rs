use std::io::{Read, Seek};

use crate::{
    read::{
        deserialize::{Deserializer, IdStateMut, NodeStateMut},
        read_body_chunks,
        readable::{BodyChunkEntry, BodyChunkReadFn, BodyChunks},
        ReadBody, Result,
    },
    read_file_ref,
};

use super::{EntRecordData, Ghost, Ghost2};

impl ReadBody for Ghost {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for Ghost {
    fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x0303f006,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Ghost, d| {
                    Ghost2::read_chunk_6(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0303f007,
                read_fn: BodyChunkReadFn::Skippable(|n: &mut Ghost, d| {
                    Ghost2::read_chunk_7(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x03092000,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092005,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_5(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092008,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_8(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309200a,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_10(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309200b,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_11(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309200c,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_12(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309200e,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_14(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309200f,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_15(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092010,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_16(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092013,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_19(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092014,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_20(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309201a,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_26(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309201b,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_27(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309201c,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_28(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309201d,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_29(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092022,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_34(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092023,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_35(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092024,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_36(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092025,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_37(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092026,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_38(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092027,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_39(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092028,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_40(n, d)),
            },
            BodyChunkEntry {
                id: 0x03092029,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_41(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309202a,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_42(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Ghost {
    fn read_chunk_0<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.id()?; // "CarSport"
        d.u32()?;
        d.id()?; // "Nadeo"
        d.u32()?;
        d.u32()?;
        d.u32()?; // 0
        d.list(|d| {
            read_file_ref(d)?;

            Ok(())
        })?;
        d.u32()?; // 0
        d.string()?; // "htimh"
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.inline_node::<EntRecordData>()?;
        d.u32()?; // 1
        d.u32()?; // 1
        d.string()?; // "TIM"
        d.string()?; // "World|Europe|United Kingdom"

        Ok(())
    }

    fn read_chunk_5<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;

        Ok(())
    }

    fn read_chunk_8<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_10<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_11<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_12<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_14<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;

        Ok(())
    }

    fn read_chunk_15<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.string()?; // "FE_OBpuQSvmlsJFIvMBWbw"

        Ok(())
    }

    fn read_chunk_16<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.id()?; // "pGjLOGmlwbkrlImXcjfyqtE7j6i"

        Ok(())
    }

    fn read_chunk_19<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_20<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 8

        Ok(())
    }

    fn read_chunk_26<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 9

        Ok(())
    }

    fn read_chunk_27<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;
        d.u32()?;
        d.u16()?;
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_28<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.bytes(32)?;

        Ok(())
    }

    fn read_chunk_29<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 4
        d.u32()?; // 1
        d.u32()?; // 12
        d.u32()?;
        d.u32()?;
        d.u32()?;
        let size = d.u32()?;
        d.bytes(size as usize)?;

        Ok(())
    }

    fn read_chunk_34<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 4
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_35<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 0
        d.u32()?; // 5
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 0
        d.u16()?; // 0
        d.u8()?; // 0

        Ok(())
    }

    fn read_chunk_36<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_37<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.string()?; // "Trackmania date=2020-10-09_10_58 git=102950-7526936b722 GameVersion=3.3.0"
        d.u32()?;
        d.u32()?; // 14
        d.u32()?; // 5
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_38<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }

    fn read_chunk_39<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 4
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_40<R: Read, I, N>(&mut self, _: &mut Deserializer<R, I, N>) -> Result<()> {
        Ok(())
    }

    fn read_chunk_41<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_42<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?; // 7

        Ok(())
    }
}

impl Ghost2 {
    fn read_chunk_6<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 4
        let size = d.u32()?; // 12
        d.bytes(size as usize)?;

        Ok(())
    }

    fn read_chunk_7<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }
}

impl ReadBody for EntRecordData {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for EntRecordData {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0911f000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl EntRecordData {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 10
        d.u32()?;
        let size = d.u32()?;
        d.bytes(size as usize)?;

        Ok(())
    }
}
