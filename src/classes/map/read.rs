use std::io::{Read, Seek};

use crate::{
    class::Class,
    read::{
        deserialize::{Deserializer, IdStateMut, NodeStateMut},
        read_body_chunks, read_gbx,
        readable::{
            BodyChunkEntry, BodyChunkReadFn, BodyChunks, HeaderChunkEntry, HeaderChunks, Sealed,
        },
        BodyOptions, HeaderOptions, ReadBody, Result,
    },
};

use super::Map;

impl Sealed for Map {
    fn read(
        reader: impl Read + Seek,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for Map {
    fn header_chunks<R: Read>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>> {
        [
            HeaderChunkEntry {
                id: 0x03043002,
                read_fn: |n, d| Self::read_chunk_03043002(n, d),
            },
            HeaderChunkEntry {
                id: 0x03043003,
                read_fn: |n, d| Self::read_chunk_03043003(n, d),
            },
            HeaderChunkEntry {
                id: 0x03043004,
                read_fn: |n, d| Self::read_chunk_03043004(n, d),
            },
            HeaderChunkEntry {
                id: 0x03043005,
                read_fn: |n, d| Self::read_chunk_03043005(n, d),
            },
            HeaderChunkEntry {
                id: 0x03043007,
                read_fn: |n, d| Self::read_chunk_03043007(n, d),
            },
            HeaderChunkEntry {
                id: 0x03043008,
                read_fn: |n, d| Self::read_chunk_03043008(n, d),
            },
        ]
        .into_iter()
    }
}

impl ReadBody for Map {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for Map {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x0304300d,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0304300d(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043011,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03043011(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043018,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043018(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043019,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043019(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304301f,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0304301f(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043022,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03043022(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043024,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03043024(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043025,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03043025(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043026,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03043026(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043028,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03043028(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043029,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043029(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304302a,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0304302a(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043034,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043034(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043036,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043036(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043038,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043038(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304303e,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304303e(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043040,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043040(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Map {
    fn read_chunk_03043002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 13
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u8()?; // 0
        d.u32()?; // 2
        d.u32()?; // 0
        d.u32()?; // 38
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_03043003<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u8()?; // 11
        d.id()?; // "d1I0RQQLjvUJLOmy9kiZDGX5E4e"
        d.u32()?; // 26
        d.id()?; // "qYw071iWQXu9_jXI7SXEvA"
        d.string()?; // "$s$i$o$F90M$FA0i$FB0n$FD0d$FE0o$FF0r"
        d.u8()?; // 8
        d.u32()?; // 0
        d.u32()?; // 0
        d.id()?; // "NoStadium48x48Sunrise"
        d.u32()?; // 26
        d.id()?; // "Nadeo"
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.string()?; // "TrackMania\TM_Race"
        d.u32()?; // 0
        d.u32()?;
        d.u32()?;
        d.u8()?; // 8
        d.id()?; // "TMStadium"

        Ok(())
    }

    fn read_chunk_03043004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 6

        Ok(())
    }

    fn read_chunk_03043005<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.string()?;

        Ok(())
    }

    fn read_chunk_03043007<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        let n = d.u32()?;
        d.bytes(15)?;
        d.bytes(n as usize)?;
        d.bytes(16)?;
        d.bytes(10)?;
        d.u32()?; // 0
        d.bytes(11)?;

        Ok(())
    }

    fn read_chunk_03043008<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.string()?; // "qYw071iWQXu9_jXI7SXEvA"
        d.string()?; // "YannexTM"
        d.string()?; // "World|Europe|Switzerland|Fribourg"
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0304300d<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.null_id()?;
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_03043011<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.inline_node::<CollectorList>()?;
        d.inline_node::<ChallengeParameters>()?;
        d.u32()?; // 6

        Ok(())
    }

    fn read_chunk_03043018<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 3

        Ok(())
    }

    fn read_chunk_03043019<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_file_ref(d)?;

        Ok(())
    }

    fn read_chunk_0304301f<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.id()?; // "d1I0RQQLjvUJLOmy9kiZDGX5E4e"
        d.u32()?; // 26
        d.id()?; // "qYw071iWQXu9_jXI7SXEvA"
        d.string()?; // "$s$i$o$F90M$FA0i$FB0n$FD0d$FE0o$FF0r"
        d.id()?; // "NoStadium48x48Sunrise"
        d.u32()?; // 26
        d.id()?; // "Nadeo"
        d.u32()?; // 120
        d.u32()?; // 120
        d.u32()?; // 120
        d.u32()?; // 0
        d.u32()?; // 6
        d.list(|d| {
            d.id()?;
            d.u32()?;
            let flags = d.u32()?;

            if flags & 0x00008000 != 0 {
                d.id()?; // "Nadeo"
                d.inline_node::<BlockSkin>()?;
            }

            if flags & 0x00100000 != 0 {
                d.inline_node::<WaypointSpecialProperty>()?;
            }

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_03043022<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_03043024<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_file_ref(d)?;

        Ok(())
    }

    fn read_chunk_03043025<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043026<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_03043028<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043029<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?;

        Ok(())
    }

    fn read_chunk_0304302a<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043034<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043036<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?; // 0
        d.f32()?; // -1.0
        d.f32()?; // -1.0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043038<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0304303e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 10
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043040<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        println!("{:02X?}", d.bytes(144)?);

        Ok(())
    }
}

#[derive(Default)]
struct CollectorList;

impl Class for CollectorList {
    const ENGINE: u8 = 0x03;
    const CLASS: u16 = 0x01b;
}

impl ReadBody for CollectorList {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for CollectorList {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0301b000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0301b000(n, d)),
        }]
        .into_iter()
    }
}

impl CollectorList {
    fn read_chunk_0301b000<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct ChallengeParameters;

impl Class for ChallengeParameters {
    const ENGINE: u8 = 0x03;
    const CLASS: u16 = 0x05b;
}

impl ReadBody for ChallengeParameters {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for ChallengeParameters {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x0305b001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0305b001(n, d)),
            },
            BodyChunkEntry {
                id: 0x0305b004,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0305b004(n, d)),
            },
            BodyChunkEntry {
                id: 0x0305b008,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0305b008(n, d)),
            },
            BodyChunkEntry {
                id: 0x0305b00a,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0305b00a(n, d)),
            },
            BodyChunkEntry {
                id: 0x0305b00d,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0305b00d(n, d)),
            },
            BodyChunkEntry {
                id: 0x0305b00e,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0305b00e(n, d)),
            },
        ]
        .into_iter()
    }
}

impl ChallengeParameters {
    fn read_chunk_0305b001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0305b004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0305b008<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0305b00a<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0305b00d<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0305b00e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.string()?; // "TrackMania\TM_Race"
        d.u32()?; // 0
        d.u32()?; // 1

        Ok(())
    }
}

fn read_file_ref<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<()> {
    if d.u8()? != 3 {
        todo!()
    }

    d.bytes(32)?;
    d.string()?;
    d.string()?;

    Ok(())
}

#[derive(Default)]
struct WaypointSpecialProperty;

impl Class for WaypointSpecialProperty {
    const ENGINE: u8 = 0x2e;
    const CLASS: u16 = 0x009;
}

impl ReadBody for WaypointSpecialProperty {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for WaypointSpecialProperty {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x2e009000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e009000(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e009001,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e009001(n, d)),
            },
        ]
        .into_iter()
    }
}

impl WaypointSpecialProperty {
    fn read_chunk_2e009000<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.string()?; // "Goal"
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_2e009001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct BlockSkin;

impl Class for BlockSkin {
    const ENGINE: u8 = 0x03;
    const CLASS: u16 = 0x059;
}

impl ReadBody for BlockSkin {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for BlockSkin {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03059002,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03059002(n, d)),
            },
            BodyChunkEntry {
                id: 0x03059003,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03059003(n, d)),
            },
        ]
        .into_iter()
    }
}

impl BlockSkin {
    fn read_chunk_03059002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.string()?; // "!4"
        read_file_ref(d)?;
        read_file_ref(d)?;

        Ok(())
    }

    fn read_chunk_03059003<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        read_file_ref(d)?;

        Ok(())
    }
}
