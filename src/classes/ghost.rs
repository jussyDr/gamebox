//! Types used for reading [Ghost] nodes.

use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
    FileRef, RcStr,
};

use super::ent_record_data::EntRecordData;

/// Node type corresponding to GameBox files with the extension `Ghost.Gbx`.
#[derive(Default)]
pub struct Ghost {
    parent: Ghost2,
    car_model_id: RcStr,
    player_name: String,
    player_trigram: String,
    player_region: String,
    player_id: RcStr,
}

impl Class for Ghost {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 146);
}

#[derive(Default)]
struct Ghost2;

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Ghost {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Ghost {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
            BodyChunkEntry {
                id: 0x0309202b,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_43(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309202c,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_44(n, d)),
            },
            BodyChunkEntry {
                id: 0x0309202d,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_45(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Ghost {
    fn read_chunk_0<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        self.car_model_id = d.id()?.into();
        d.u32()?;
        let _car_model_author = d.id()?;
        d.u32()?;
        d.u32()?;
        d.u32()?; // 0
        d.list(|d| {
            FileRef::read(d)?;

            Ok(())
        })?;
        d.u32()?; // 0
        self.player_name = d.string()?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.internal_node_ref::<EntRecordData>()?;
        d.u32()?; // 1
        d.u32()?; // 1
        self.player_trigram = d.string()?;
        self.player_region = d.string()?;

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
        let _player_id = d.string()?;

        Ok(())
    }

    fn read_chunk_16<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let _map_id = d.id()?;

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

    fn read_chunk_40<R, I, N>(&mut self, _: &mut Deserializer<R, I, N>) -> Result<()> {
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

    fn read_chunk_43<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?;
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_44<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?;
        d.u32()?;

        Ok(())
    }

    fn read_chunk_45<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.string()?; // "Trackmania date=2022-05-19_15_03 git=113018-25c17c3a2da GameVersion=3.3.0"
        d.u32()?;
        d.u32()?; // 14
        d.u32()?; // 6
        d.u32()?;
        d.u32()?;
        d.string()?; // "Trackmania"
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?; // 0
        d.u32()?;
        d.u32()?; // 16
        d.u32()?; // 0

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
