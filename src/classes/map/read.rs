use std::io::{Read, Seek};

use crate::{
    class::Class,
    read::{
        deserialize::{Deserializer, IdState, IdStateMut, NodeState, NodeStateMut},
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
            BodyChunkEntry {
                id: 0x03043042,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043042(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043043,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043043(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043044,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043044(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043048,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043048(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043049,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03043049(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304304b,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304304b(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304304f,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304304f(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043050,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043050(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043051,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043051(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043052,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043052(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043053,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043053(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043054,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043054(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043055,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043055(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043056,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043056(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043057,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043057(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043059,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043059(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304305a,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304305a(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304305b,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304305b(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304305c,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304305c(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304305d,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304305d(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304305e,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304305e(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304305f,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304305f(n, d)),
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

    fn read_chunk_03043040<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 7
        d.u32()?; // 0
        let size = d.u32()?;

        {
            let mut d = d.take(size as u64, IdState::default(), NodeState::new(0));

            d.u32()?; // 10
            d.list(|d| {
                d.inline_node_no_index::<AnchoredObject>()?;

                Ok(())
            })?;
            d.u32()?; // 0
            d.list(|d| {
                d.u32()?;

                Ok(())
            })?;
            d.list(|d| {
                d.u32()?; // 0xfffffffff

                Ok(())
            })?;
            d.list(|d| {
                d.u32()?;

                Ok(())
            })?;
            d.list(|d| {
                d.u32()?; // 0xfffffffff

                Ok(())
            })?;
            d.list(|d| {
                d.u32()?; // 0xfffffffff

                Ok(())
            })?;

            d.end()?;
        }

        Ok(())
    }

    fn read_chunk_03043042<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.string()?; // "qYw071iWQXu9_jXI7SXEvA"
        d.string()?; // "YannexTM"
        d.string()?; // "World|Europe|Switzerland|Fribourg"
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043043<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        let size = d.u32()?;

        {
            let mut d = d.take(size as u64, IdState::default(), NodeState::new(0));

            d.list(|d| {
                d.inline_node_no_index::<ZoneGenealogy>()?;

                Ok(())
            })?;

            d.end()?;
        }

        Ok(())
    }

    fn read_chunk_03043044<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        let size = d.u32()?;

        {
            let mut d = d.take(size as u64, IdState::default(), NodeState::new(0));

            d.inline_node_no_index::<TraitsMetadata>()?;

            d.end()?;
        }

        Ok(())
    }

    fn read_chunk_03043048<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 6
        d.list(|d| {
            d.id()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043049<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 2
        d.inline_node::<MediaClip>()?;
        d.inline_node::<MediaClip>()?;
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.inline_node::<MediaClip>()?;
        d.u32()?; // 3
        d.u32()?; // 1
        d.u32()?; // 3

        Ok(())
    }

    fn read_chunk_0304304b<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0304304f<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.u8()?; // 0

        Ok(())
    }

    fn read_chunk_03043050<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 3
        d.u32()?; // 1
        d.u32()?; // 3
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043051<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.id()?; // "TMStadium"
        d.string()?; // "date=2023-11-15_11_56 git=126529-e25ec54fd0a GameVersion=3.3.0"

        Ok(())
    }

    fn read_chunk_03043052<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 8

        Ok(())
    }

    fn read_chunk_03043053<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043054<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        let size = d.u32()?;

        {
            let mut d = d.take(size as u64, IdState::default(), ());

            d.list(|d| {
                d.id()?;
                d.u32()?; // 26
                d.id()?;

                Ok(())
            })?;
            let size = d.u32()?;
            d.bytes(size as usize)?;
            d.u32()?; // 0

            d.end()?;
        }

        Ok(())
    }

    fn read_chunk_03043055<R: Read, I, N>(&mut self, _: &mut Deserializer<R, I, N>) -> Result<()> {
        Ok(())
    }

    fn read_chunk_03043056<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?;

        Ok(())
    }

    fn read_chunk_03043057<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 5
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043059<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?;
        d.u32()?;

        Ok(())
    }

    fn read_chunk_0304305a<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0304305b<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 10
        d.list(|d| {
            let size = d.u32()?;
            d.bytes(size as usize)?;
            let size = d.u32()?;
            d.bytes(size as usize)?;
            let size = d.u32()?;
            d.bytes(size as usize)?;

            Ok(())
        })?;
        d.u32()?;
        let size = d.u32()?;
        d.bytes(size as usize)?;

        Ok(())
    }

    fn read_chunk_0304305c<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_0304305d<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.bytes(83860)?;

        Ok(())
    }

    fn read_chunk_0304305e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 8
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0304305f<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

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

#[derive(Default)]
struct AnchoredObject;

impl Class for AnchoredObject {
    const ENGINE: u8 = 0x03;
    const CLASS: u16 = 0x101;
}

impl ReadBody for AnchoredObject {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for AnchoredObject {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03101002,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03101002(n, d)),
            },
            BodyChunkEntry {
                id: 0x03101004,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03101004(n, d)),
            },
            BodyChunkEntry {
                id: 0x03101005,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03101005(n, d)),
            },
        ]
        .into_iter()
    }
}

impl AnchoredObject {
    fn read_chunk_03101002<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 8
        d.id()?; // "Rocks\RPG Rocks\RockB\9\Rocher2.9.4.Item.Gbx"
        d.u32()?; // 26
        d.id()?; // "qYw071iWQXu9_jXI7SXEvA"
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u16()?; // 0
        d.u8()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.inline_node_no_index_or_null::<WaypointSpecialProperty>()?;
        let flags = d.u16()?; // 1
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        if flags & 0x0004 != 0 {
            read_file_ref(d)?;
        }
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }

    fn read_chunk_03101004<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03101005<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 4
        d.u8()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct ZoneGenealogy;

impl Class for ZoneGenealogy {
    const ENGINE: u8 = 0x03;
    const CLASS: u16 = 0x11d;
}

impl ReadBody for ZoneGenealogy {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for ZoneGenealogy {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0311d002,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0311d002(n, d)),
        }]
        .into_iter()
    }
}

impl ZoneGenealogy {
    fn read_chunk_0311d002<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1
        d.id()?; // "VoidToGrass"
        d.u32()?; // 0
        d.u32()?; // 0
        d.id()?; // "Grass"

        Ok(())
    }
}

#[derive(Default)]
struct TraitsMetadata;

impl Class for TraitsMetadata {
    const ENGINE: u8 = 0x11;
    const CLASS: u16 = 0x002;
}

impl ReadBody for TraitsMetadata {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 6
        let n = d.u8()?;
        let types = d.repeat(n as usize, |d| {
            let ty = read_type(d)?;

            Ok(ty)
        })?;
        let n = d.u8()?;
        d.repeat(n as usize, |d| {
            let size = d.u8()?;
            d.bytes(size as usize)?;
            let type_index = d.u8()?;

            read_value(d, &types[type_index as usize])?;

            Ok(())
        })?;

        if d.u32()? != 0xfacade01 {
            todo!()
        }

        Ok(())
    }
}

fn read_type<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Type> {
    let ty = d.u8()?;

    match ty {
        0 => Ok(Type::Void),
        1 => Ok(Type::Boolean),
        2 => Ok(Type::Integer),
        7 => {
            let key_type = read_type(d)?;
            let element_type = read_type(d)?;

            Ok(Type::Array {
                key_type: Box::new(key_type),
                element_type: Box::new(element_type),
            })
        }
        _ => todo!("{ty}"),
    }
}

fn read_value<R: Read, I, N>(d: &mut Deserializer<R, I, N>, ty: &Type) -> Result<()> {
    match ty {
        Type::Void => {}
        Type::Boolean => {
            d.bool8()?;
        }
        Type::Integer => {
            d.i32()?;
        }
        Type::Array {
            key_type,
            element_type,
        } => {
            let len = d.u8()?;
            d.repeat(len as usize, |d| {
                read_value(d, key_type)?;
                read_value(d, element_type)?;

                Ok(())
            })?;
        }
    }

    Ok(())
}

enum Type {
    Void,
    Boolean,
    Integer,
    Array {
        key_type: Box<Type>,
        element_type: Box<Type>,
    },
}

#[derive(Default)]
struct MediaClip;

impl Class for MediaClip {
    const ENGINE: u8 = 0x03;
    const CLASS: u16 = 0x079;
}

impl ReadBody for MediaClip {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaClip {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x0307900d,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0307900d(n, d)),
            },
            BodyChunkEntry {
                id: 0x0307900e,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0307900e(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MediaClip {
    fn read_chunk_0307900d<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 10
        d.list(|d| {
            d.inline_node::<MediaTrack>()?;

            Ok(())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?;
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0307900e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct MediaTrack;

impl Class for MediaTrack {
    const ENGINE: u8 = 0x03;
    const CLASS: u16 = 0x078;
}

impl ReadBody for MediaTrack {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaTrack {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03078001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03078001(n, d)),
            },
            BodyChunkEntry {
                id: 0x03078005,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_5(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MediaTrack {
    fn read_chunk_03078001<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.string()?;
        d.u32()?; // 10
        d.list(|d| {
            d.any_inline_node(|d, class_id| {
                match class_id {
                    0x03080000 => {
                        let mut node = MediaBlockFxColors;
                        MediaBlockFxColors::read_body(&mut node, d)?;
                    }
                    0x030a2000 => {
                        let mut node = MediaBlockCameraCustom;
                        MediaBlockCameraCustom::read_body(&mut node, d)?;
                    }
                    0x030a5000 => {
                        let mut node = MediaBlockImage;
                        MediaBlockImage::read_body(&mut node, d)?;
                    }
                    0x03127000 => {
                        let mut node = MediaBlockToneMapping;
                        MediaBlockToneMapping::read_body(&mut node, d)?;
                    }
                    0x03186000 => {
                        let mut node = MediaBlockColorGrading;
                        MediaBlockColorGrading::read_body(&mut node, d)?;
                    }
                    0x03199000 => {
                        let mut node = MediaBlockFog;
                        MediaBlockFog::read_body(&mut node, d)?;
                    }
                    _ => todo!(),
                }

                Ok(())
            })?;

            Ok(())
        })?;
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_5<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // -1
        d.f32()?; // -1

        Ok(())
    }
}

struct MediaBlockFxColors;

impl ReadBody for MediaBlockFxColors {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockFxColors {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03080003,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_3(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockFxColors {
    fn read_chunk_3<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
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
}

struct MediaBlockCameraCustom;

impl ReadBody for MediaBlockCameraCustom {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockCameraCustom {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030a2006,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_6(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockCameraCustom {
    fn read_chunk_6<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
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
}

struct MediaBlockImage;

impl ReadBody for MediaBlockImage {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockImage {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030a5000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockImage {
    fn read_chunk_0<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.inline_node::<EffectSimi>()?;
        read_file_ref(d)?;

        Ok(())
    }
}

struct MediaBlockToneMapping;

impl ReadBody for MediaBlockToneMapping {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockToneMapping {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03127004,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_4(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockToneMapping {
    fn read_chunk_4<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

struct MediaBlockColorGrading;

impl ReadBody for MediaBlockColorGrading {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockColorGrading {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03186000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
            },
            BodyChunkEntry {
                id: 0x03186001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_1(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MediaBlockColorGrading {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_file_ref(d)?;

        Ok(())
    }

    fn read_chunk_1<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

struct MediaBlockFog;

impl ReadBody for MediaBlockFog {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockFog {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03199000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockFog {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
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
}

#[derive(Default)]
struct EffectSimi;

impl Class for EffectSimi {
    const ENGINE: u8 = 0x07;
    const CLASS: u16 = 0x010;
}

impl ReadBody for EffectSimi {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for EffectSimi {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x07010005,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_5(n, d)),
        }]
        .into_iter()
    }
}

impl EffectSimi {
    fn read_chunk_5<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }
}
