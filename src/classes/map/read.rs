use std::io::{BufRead, Read, Seek};

use crate::{
    class::ClassId,
    classes::ghost::Ghost,
    read::{
        deserialize::{Deserializer, IdState, IdStateMut, NodeState, NodeStateMut},
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
    read_file_ref, EngineId,
};

use super::{
    media::{MediaClip, MediaClipGroup},
    Block, BlockKind, Color, Coord, Direction, EmbeddedObjects, FreeBlock, Item, LightmapQuality,
    Map, MedalTimes, NormalBlock, PhaseOffset, Position, Rotation,
};

impl Readable for Map {}

impl Sealed for Map {
    fn read(
        reader: impl BufRead + Seek,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for Map {
    fn header_chunks<R: BufRead>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>> {
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
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for Map {
    fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
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
                id: 0x03043058,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043058(n, d)),
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
            BodyChunkEntry {
                id: 0x03043060,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043060(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043061,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043061(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043062,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043062(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043063,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043063(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043064,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043064(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043065,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043065(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043067,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043067(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043068,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043068(n, d)),
            },
            BodyChunkEntry {
                id: 0x03043069,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_03043069(n, d)),
            },
            BodyChunkEntry {
                id: 0x0304306b,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0304306b(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Map {
    fn read_chunk_03043002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u8()?; // 13
        d.u32()?;
        let bronze = d.u32()?;
        let silver = d.u32()?;
        let gold = d.u32()?;
        let author = d.u32()?;
        self.cost = d.u32()?;
        let _is_multilap = d.bool32()?;
        let _play_mode = d.u32()?; // 0
        d.u32()?; // 0
        let _author_score = d.u32()?; // 0
        let _editor_mode = d.u32()?; // 2
        d.u32()?; // 0
        let _num_cps = d.u32()?; // 38
        let _num_laps = d.u32()?; // 1

        if bronze != 0xffffffff
            && silver != 0xffffffff
            && gold != 0xffffffff
            && author != 0xffffffff
        {
            self.medal_times = Some(MedalTimes {
                bronze,
                silver,
                gold,
                author,
            });
        }

        Ok(())
    }

    fn read_chunk_03043003<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u8()?; // 11
        self.id = d.id()?.into();
        d.u32()?; // 26
        self.author_id = d.id()?.into();
        self.name = d.string()?;
        let _map_kind = d.u8()?; // 8
        d.u32()?; // 0
        let _password = d.u32()?; // 0
        let _deco_id = d.id()?; // "NoStadium48x48Sunrise"
        d.u32()?; // 26
        let _deco_author_id = d.id()?; // "Nadeo"
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        self.ty = d.string()?;
        self.style = d.string()?;
        let _lightmap_cache_id = d.u64()?;
        let _lightmap_version = d.u8()?; // 8
        let _title_id = d.id()?; // "TMStadium"

        Ok(())
    }

    fn read_chunk_03043004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 6

        Ok(())
    }

    fn read_chunk_03043005<R: BufRead, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let len = d.u32()?;

        let mut xml_reader = xml::Deserializer::new(d.get_mut(), len as usize);

        xml_reader.with_inner_content(
            b"header",
            |attributes| {
                if attributes.get(b"type").unwrap() != b"map" {
                    todo!()
                }

                if attributes.get(b"exever").unwrap() != b"3.3.0" {
                    todo!()
                }

                attributes.get(b"exebuild").unwrap();
                attributes.get(b"title").unwrap();
                attributes.get(b"lightmap").unwrap();
            },
            |xml_reader| {
                xml_reader.with_empty(b"ident", |attributes| {
                    self.id = attributes.get_str(b"uid").unwrap().into();
                    self.name = attributes.get_str(b"name").unwrap().into();
                    self.author_id = attributes.get_str(b"author").unwrap().into();
                    self.author_region = attributes.get_str(b"authorzone").unwrap().into();
                });

                xml_reader.with_empty(b"desc", |attributes| {
                    if attributes.get(b"envir").unwrap() != b"Stadium" {
                        todo!()
                    }

                    attributes.get(b"mood").unwrap();

                    if attributes.get(b"type").unwrap() != b"Race" {
                        todo!()
                    }

                    self.ty = attributes.get_str(b"maptype").unwrap().into();
                    self.style = attributes.get_str(b"mapstyle").unwrap().to_owned();
                    attributes.get(b"validated").unwrap();
                    attributes.get(b"nblaps").unwrap();
                    self.cost = attributes.get_u32(b"displaycost").unwrap();
                    attributes.get(b"mod").unwrap();
                    attributes.get(b"hasghostblocks").unwrap();
                });

                xml_reader.with_empty(b"playermodel", |attributes| {
                    attributes.get(b"id").unwrap();
                });

                xml_reader.with_empty(b"times", |attributes| {
                    let bronze = attributes.get_u32_or_null(b"bronze").unwrap();
                    let silver = attributes.get_u32_or_null(b"silver").unwrap();
                    let gold = attributes.get_u32_or_null(b"gold").unwrap();
                    let author = attributes.get_u32_or_null(b"authortime").unwrap();
                    attributes.get(b"authorscore").unwrap();

                    if bronze != 0xffffffff
                        && silver != 0xffffffff
                        && gold != 0xffffffff
                        && author != 0xffffffff
                    {
                        self.medal_times = Some(MedalTimes {
                            bronze,
                            silver,
                            gold,
                            author,
                        });
                    }
                });

                xml_reader.until_end(b"deps");
            },
        );

        xml_reader.eof();

        Ok(())
    }

    fn read_chunk_03043007<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        let thumbnail_size = d.u32()?;
        d.bytes(15)?;
        let _thumbnail = d.bytes(thumbnail_size as usize)?;
        d.bytes(16)?;
        d.bytes(10)?;
        let _comments = d.u32()?; // 0
        d.bytes(11)?;

        Ok(())
    }

    fn read_chunk_03043008<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        self.author_id = d.string()?.into(); // "qYw071iWQXu9_jXI7SXEvA"
        self.author_name = d.string()?; // "YannexTM"
        self.author_region = d.string()?; // "World|Europe|Switzerland|Fribourg"
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

    fn read_chunk_03043011<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.internal_node_ref::<CollectorList>()?;
        let params = d.internal_node_ref::<ChallengeParameters>()?;
        let _map_kind = d.u32()?; // 6

        self.medal_times = params.medal_times.clone();
        self.ty = params.ty.clone();
        self.style = params.style.clone();

        Ok(())
    }

    fn read_chunk_03043018<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let _is_multilap = d.bool32()?; // 0
        let _num_laps = d.u32()?; // 3

        Ok(())
    }

    fn read_chunk_03043019<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let _mod = read_file_ref(d)?;

        Ok(())
    }

    fn read_chunk_0304301f<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        self.id = d.id()?.into(); // "d1I0RQQLjvUJLOmy9kiZDGX5E4e"
        d.u32()?; // 26
        self.author_id = d.id()?.into(); // "qYw071iWQXu9_jXI7SXEvA"
        self.name = d.string()?; // "$s$i$o$F90M$FA0i$FB0n$FD0d$FE0o$FF0r"
        let _deco_id = d.id()?; // "NoStadium48x48Sunrise"
        d.u32()?; // 26
        let _deco_author = d.id()?; // "Nadeo"
        let _size_x = d.u32()?; // 120
        let _size_y = d.u32()?; // 120
        let _size_z = d.u32()?; // 120
        d.u32()?; // 0
        d.u32()?; // 6
        let num_blocks = d.u32()?;
        self.blocks = Vec::with_capacity(num_blocks as usize);
        while d.peek_u32()? & 0xffffc000 == 0x40000000 {
            let id = d.id()?;
            let direction = d.u8()?;
            let x = d.u8()?;
            let y = d.u8()?;
            let z = d.u8()?;
            let flags = d.u32()?;

            if flags == 0xffffffff {
                continue;
            }

            if flags & 0x00008000 != 0 {
                d.id()?; // "Nadeo"
                d.internal_node_ref_or_null::<BlockSkin>()?;
            }

            if flags & 0x00100000 != 0 {
                d.internal_node_ref::<WaypointSpecialProperty>()?;
            }

            let is_free = flags & 0x20000000 != 0;

            if is_free {
                self.blocks.push(Block {
                    id,
                    kind: BlockKind::Free(FreeBlock::default()),
                    color: Color::default(),
                    lightmap_quality: LightmapQuality::default(),
                });
            } else {
                let direction = Direction::try_from_u8(direction)?;

                let coord = Coord { x, y, z };

                let is_ghost = flags & 0x10000000 != 0;

                self.blocks.push(Block {
                    id,
                    kind: BlockKind::Normal(NormalBlock {
                        direction,
                        coord,
                        is_ghost,
                    }),
                    color: Color::default(),
                    lightmap_quality: LightmapQuality::default(),
                });
            }
        }

        Ok(())
    }

    fn read_chunk_03043022<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_03043024<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let _music = read_file_ref(d)?;

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
        let _comments = d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043029<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let _password = d.bytes(16)?;
        let _crc = d.u32()?;

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

    fn read_chunk_03043040<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?; // 5 | 7
        d.u32()?; // 0
        let size = d.u32()?;
        {
            let mut d = d.take_with(size as u64, IdState::new(), NodeState::new(0));

            d.u32()?; // 10
            self.items = d.list(|d| {
                let item = d.node::<AnchoredObject>()?;

                Ok(item)
            })?;
            if version != 5 {
                d.list(|d| {
                    d.u32()?;
                    d.u32()?;

                    Ok(())
                })?;
            }
            d.list(|d| {
                d.u32()?;

                Ok(())
            })?;
            if version < 7 {
                d.list(|d| {
                    d.u32()?;

                    Ok(())
                })?;
            }
            if version >= 6 {
                d.list(|d| {
                    d.u32()?;

                    Ok(())
                })?;
            }
            if version >= 7 {
                d.list(|d| {
                    d.u32()?;

                    Ok(())
                })?;
            }
            if version != 6 {
                d.list(|d| {
                    d.u32()?;

                    Ok(())
                })?;
            }
            d.list(|d| {
                d.u32()?;

                Ok(())
            })?;

            d.end()?;
        }

        Ok(())
    }

    fn read_chunk_03043042<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        self.author_id = d.string()?.into(); // "qYw071iWQXu9_jXI7SXEvA"
        self.author_name = d.string()?; // "YannexTM"
        self.author_region = d.string()?; // "World|Europe|Switzerland|Fribourg"
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043043<R: Read + Seek, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        let size = d.u32()?;

        {
            let mut d = d.take_with(size as u64, IdState::new(), NodeState::new(0));

            d.list(|d| {
                d.node::<ZoneGenealogy>()?;

                Ok(())
            })?;

            d.end()?;
        }

        Ok(())
    }

    fn read_chunk_03043044<R: Read + Seek, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        let size = d.u32()?;

        {
            let mut d = d.take_with(size as u64, IdState::new(), NodeState::new(0));

            d.node::<TraitsMetadata>()?;

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
        self.baked_blocks = d.list(|d| {
            let id = d.id()?;
            let direction = d.u8()?;
            let x = d.u8()?;
            let y = d.u8()?;
            let z = d.u8()?;
            let flags = d.u32()?;

            let is_free = flags & 0x20000000 != 0;

            if is_free {
                Ok(Block {
                    id,
                    kind: BlockKind::Free(FreeBlock::default()),
                    color: Color::default(),
                    lightmap_quality: LightmapQuality::default(),
                })
            } else {
                let direction = Direction::try_from_u8(direction)?;

                let coord = Coord { x, y, z };

                let is_ghost = flags & 0x10000000 != 0;

                Ok(Block {
                    id,
                    kind: BlockKind::Normal(NormalBlock {
                        direction,
                        coord,
                        is_ghost,
                    }),
                    color: Color::default(),
                    lightmap_quality: LightmapQuality::default(),
                })
            }
        })?;
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043049<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 2
        self.intro_media = d.internal_node_ref_or_null::<MediaClip>()?;
        self.podium_media = d.internal_node_ref_or_null::<MediaClip>()?;
        self.in_game_media = d.internal_node_ref_or_null::<MediaClipGroup>()?;
        self.end_race_media = d.internal_node_ref_or_null::<MediaClipGroup>()?;
        self.ambiance_media = d.internal_node_ref_or_null::<MediaClip>()?;
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
            let mut d = d.take_with(size as u64, IdState::new(), ());

            let object_ids = d.list(|d| {
                let id = d.id()?.into();
                d.u32()?; // 26
                d.id_or_null()?;

                Ok(id)
            })?;
            let size = d.u32()?;
            let data = d.bytes(size as usize)?;

            self.embedded_objects = Some(EmbeddedObjects { object_ids, data });

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

    fn read_chunk_03043058<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
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
        let list = d.list(|d| {
            let size = d.u32()?;
            d.bytes(size as usize)?;
            let size = d.u32()?;
            d.bytes(size as usize)?;
            let size = d.u32()?;
            d.bytes(size as usize)?;

            Ok(())
        })?;
        if !list.is_empty() {
            d.u32()?;
            let size = d.u32()?;
            d.bytes(size as usize)?;
        }

        Ok(())
    }

    fn read_chunk_0304305c<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        if !d.bool32()? {
            d.u32()?; // 0
            d.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_0304305d<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        if d.u32()? != 0 {
            d.bytes(83852)?;
        }

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
        for block in self.blocks.iter_mut().chain(self.baked_blocks.iter_mut()) {
            if let BlockKind::Free(ref mut free_block) = block.kind {
                let x = d.f32()?;
                let y = d.f32()?;
                let z = d.f32()?;

                free_block.position = Position { x, y, z };

                let yaw = d.f32()?;
                let pitch = d.f32()?;
                let roll = d.f32()?;

                free_block.rotation = Rotation { yaw, pitch, roll };
            }
        }

        Ok(())
    }

    fn read_chunk_03043060<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043061<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        if d.u32()? != 0 {
            let n = d.u32()?;
            d.bytes(n as usize * 4)?;
            let n = d.u32()?;
            d.bytes(n as usize)?;
            d.u32()?;
        } else {
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_03043062<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        for block in self.blocks.iter_mut().chain(self.baked_blocks.iter_mut()) {
            block.color = Color::read(d)?;
        }
        for item in &mut self.items {
            item.color = Color::read(d)?;
        }

        Ok(())
    }

    fn read_chunk_03043063<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        for item in &mut self.items {
            item.animation_offset = PhaseOffset::read(d)?;
        }

        Ok(())
    }

    fn read_chunk_03043064<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 4
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043065<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        for _ in &self.items {
            if d.bool8()? {
                read_file_ref(d)?;
            }
        }

        Ok(())
    }

    fn read_chunk_03043067<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 4
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_03043068<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        for _ in &self.blocks {
            d.u8()?;
        }
        for _ in &self.baked_blocks {
            d.u8()?;
        }
        for _ in &self.items {
            d.u8()?;
        }

        Ok(())
    }

    fn read_chunk_03043069<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        for _ in &self.blocks {
            d.u32()?;
        }
        for _ in &self.items {
            d.u32()?;
        }
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_0304306b<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?;

        Ok(())
    }
}

#[derive(Default)]
struct CollectorList;

impl ClassId for CollectorList {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x01b;
}

impl ReadBody for CollectorList {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
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
struct ChallengeParameters {
    medal_times: Option<MedalTimes>,
    ty: String,
    style: String,
}

impl ClassId for ChallengeParameters {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x05b;
}

impl ReadBody for ChallengeParameters {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for ChallengeParameters {
    fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
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
        let bronze = d.u32()?;
        let silver = d.u32()?;
        let gold = d.u32()?;
        let author = d.u32()?;
        d.u32()?;
        d.u32()?; // 0

        if bronze != 0xffffffff
            && silver != 0xffffffff
            && gold != 0xffffffff
            && author != 0xffffffff
        {
            self.medal_times = Some(MedalTimes {
                bronze,
                silver,
                gold,
                author,
            });
        }

        Ok(())
    }

    fn read_chunk_0305b00d<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.internal_node_ref_or_null::<Ghost>()?;

        Ok(())
    }

    fn read_chunk_0305b00e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        self.ty = d.string()?;
        self.style = d.string()?;
        d.u32()?; // 1

        Ok(())
    }
}

#[derive(Default)]
struct WaypointSpecialProperty;

impl ClassId for WaypointSpecialProperty {
    const ENGINE: u8 = 0x2e;
    const CLASS: u16 = 0x009;
}

impl ReadBody for WaypointSpecialProperty {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
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

impl ClassId for BlockSkin {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x059;
}

impl ReadBody for BlockSkin {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
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

type AnchoredObject = Item;

impl ClassId for AnchoredObject {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x101;
}

impl ReadBody for AnchoredObject {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for AnchoredObject {
    fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
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
    fn read_chunk_03101002<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 8
        self.id = d.id()?.into(); // "Rocks\RPG Rocks\RockB\9\Rocher2.9.4.Item.Gbx"
        d.u32()?; // 26
        d.id_or_null()?; // "qYw071iWQXu9_jXI7SXEvA"
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u16()?; // 0
        d.u8()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.node_or_null::<WaypointSpecialProperty>()?;
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

impl ClassId for ZoneGenealogy {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x11d;
}

impl ReadBody for ZoneGenealogy {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
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

impl ClassId for TraitsMetadata {
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

impl Direction {
    fn try_from_u8(x: u8) -> Result<Self> {
        let direction = match x {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => todo!(),
        };

        Ok(direction)
    }
}

impl Color {
    fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Self> {
        let color = match d.u8()? {
            0 => Self::Default,
            1 => Self::White,
            2 => Self::Green,
            3 => Self::Blue,
            4 => Self::Red,
            5 => Self::Black,
            _ => todo!(),
        };

        Ok(color)
    }
}

impl PhaseOffset {
    fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Self> {
        let phase_offset = match d.u8()? {
            0 => Self::None,
            1 => Self::One8th,
            2 => Self::Two8th,
            3 => Self::Three8th,
            4 => Self::Four8th,
            5 => Self::Five8th,
            6 => Self::Six8th,
            7 => Self::Seven8th,
            _ => todo!(),
        };

        Ok(phase_offset)
    }
}

mod xml {
    use std::{borrow::Cow, collections::HashMap, io::BufRead, str};

    use quick_xml::{events::Event, Reader};

    pub struct Deserializer<R> {
        reader: Reader<R>,
        buf: Vec<u8>,
    }

    impl<R> Deserializer<R> {
        pub fn new(reader: R, len: usize) -> Self {
            Self {
                reader: Reader::from_reader(reader),
                buf: Vec::with_capacity(len),
            }
        }
    }

    impl<R: BufRead> Deserializer<R> {
        pub fn with_inner_content(
            &mut self,
            name: &[u8],
            mut attr_read_fn: impl FnMut(Attributes),
            mut inner_read_fn: impl FnMut(&mut Self),
        ) {
            let tag = match self.reader.read_event_into(&mut self.buf).unwrap() {
                Event::Start(tag) if tag.name().into_inner() == name => tag,
                _ => todo!(),
            };

            let mut attribute_map = HashMap::new();

            for attribute in tag.attributes() {
                let attribute = attribute.unwrap();

                attribute_map.insert(attribute.key.into_inner(), attribute.value);
            }

            let attributes = Attributes { map: attribute_map };

            attr_read_fn(attributes);

            inner_read_fn(self);

            match self.reader.read_event_into(&mut self.buf).unwrap() {
                Event::End(tag) if tag.name().into_inner() == name => {}
                e => todo!("{e:?}"),
            }
        }

        pub fn with_empty(&mut self, name: &[u8], mut attr_read_fn: impl FnMut(Attributes)) {
            let tag = match self.reader.read_event_into(&mut self.buf).unwrap() {
                Event::Empty(tag) if tag.name().into_inner() == name => tag,
                _ => todo!(),
            };

            let mut attribute_map = HashMap::new();

            for attribute in tag.attributes() {
                let attribute = attribute.unwrap();

                attribute_map.insert(attribute.key.into_inner(), attribute.value);
            }

            let attributes = Attributes { map: attribute_map };

            attr_read_fn(attributes);
        }

        pub fn eof(&mut self) {
            match self.reader.read_event_into(&mut self.buf).unwrap() {
                Event::Eof => {}
                _ => todo!(),
            };
        }

        pub fn until_end(&mut self, name: &[u8]) {
            match self.reader.read_event_into(&mut self.buf).unwrap() {
                Event::Start(tag) if tag.name().into_inner() == name => {}
                _ => todo!(),
            };

            loop {
                match self.reader.read_event_into(&mut self.buf).unwrap() {
                    Event::End(tag) if tag.name().into_inner() == name => break,
                    _ => {}
                };
            }
        }
    }

    pub struct Attributes<'a> {
        map: HashMap<&'a [u8], Cow<'a, [u8]>>,
    }

    impl Attributes<'_> {
        pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
            match self.map.get(key) {
                None => None,
                Some(value) => Some(value.as_ref()),
            }
        }

        pub fn get_u32(&self, key: &[u8]) -> Option<u32> {
            match self.get_str(key) {
                None => None,
                Some(s) => Some(s.parse().unwrap()),
            }
        }

        pub fn get_u32_or_null(&self, key: &[u8]) -> Option<u32> {
            match self.get_str(key) {
                None => None,
                Some("-1") => Some(0xffffffff),
                Some(s) => Some(s.parse().unwrap()),
            }
        }

        pub fn get_str(&self, key: &[u8]) -> Option<&str> {
            match self.map.get(key) {
                None => None,
                Some(value) => Some(str::from_utf8(value).unwrap()),
            }
        }
    }
}
