use std::io::{BufRead, Read, Seek};

use crate::{
    common::{Class, ClassId, EngineId, Vec3, ID_INDEX_MASK, ID_MARKER_BIT, NULL},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    engines::{
        game::{ghost::Ghost, map::BlockSkin, zone_genealogy::ZoneGenealogy},
        game_data::waypoint_special_property::WaypointSpecialProperty,
        script::traits_metadata::TraitsMetadata,
    },
    read::{
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
    ExternalFileRef, FileRef, InternalFileRef,
};

use super::{
    media::{MediaClip, MediaClipGroup},
    Block, BlockKind, ChallengeParameters, CollectorList, Direction, ElemColor, EmbeddedObjects,
    FreeBlock, Item, LightmapQuality, Map, MapType, NormalBlock, PhaseOffset, Validation,
    YawPitchRoll,
};

impl Readable for Map {}

impl Sealed for Map {
    fn read(
        reader: impl Read,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for Map {
    #[allow(clippy::redundant_closure)]
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

impl<R: Read + Seek, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Map {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read + Seek, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Map {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
        let version = d.u8()?;

        if version != 13 {
            return Err("".into());
        }

        d.expect_u32(0)?;
        let bronze_time = d.u32()?;
        let silver_time = d.u32()?;
        let gold_time = d.u32()?;
        let author_time = d.u32()?;
        self.cost = d.u32()?;
        let _is_multilap = d.bool32()?;
        let _play_mode = d.u32()?; // 0
        d.expect_u32(0)?;
        let _author_score = d.u32()?; // 0
        let _editor_mode = d.u32()?; // 2
        d.expect_u32(0)?;
        let _num_cps = d.u32()?; // 38
        let _num_laps = d.u32()?; // 1

        if bronze_time != 0xffffffff
            && silver_time != 0xffffffff
            && gold_time != 0xffffffff
            && author_time != 0xffffffff
        {
            if let Some(validation) = &mut self.params.validation {
                validation.bronze_time = bronze_time;
                validation.silver_time = silver_time;
                validation.gold_time = gold_time;
                validation.author_time = author_time;
            } else {
                self.params.validation = Some(Validation {
                    bronze_time,
                    silver_time,
                    gold_time,
                    author_time,
                    ghost: None,
                });
            }
        }

        Ok(())
    }

    fn read_chunk_03043003<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u8()?;

        if version != 11 {
            return Err("".into());
        }

        self.id = d.id()?.into();
        d.u32()?; // 26
        self.author_id = d.id()?.into();
        self.name = d.string()?;
        let _map_kind = d.u8()?; // 8
        d.expect_u32(0)?;
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
        self.params.ty = match d.string()? {
            path if path == "TrackMania\\TM_Race" => MapType::Race,
            path => MapType::Script { path },
        };
        self.params.style = d.string()?;
        let _lightmap_cache_id = d.u64()?;
        let _lightmap_version = d.u8()?; // 8
        let _title_id = d.id()?; // "TMStadium"

        Ok(())
    }

    fn read_chunk_03043004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let version = d.u32()?;

        if version != 6 {
            return Err("".into());
        }

        Ok(())
    }

    fn read_chunk_03043005<R: BufRead, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let len = d.u32()?;

        let mut xml_reader = xml::Deserializer::new(d.get_reader_mut(), len as usize);

        xml_reader.with_inner_content(
            b"header",
            |attributes| {
                if attributes.get(b"type")? != b"map" {
                    return Err("".into());
                }

                if attributes.get(b"exever")? != b"3.3.0" {
                    return Err("".into());
                }

                attributes.get(b"exebuild")?;
                attributes.get(b"title")?;
                attributes.get(b"lightmap")?;

                Ok(())
            },
            |xml_reader| {
                xml_reader.with_empty(b"ident", |attributes| {
                    self.id = attributes.get_str(b"uid")?.into();
                    self.name = attributes.get_str(b"name")?.into();
                    self.author_id = attributes.get_str(b"author")?.into();
                    self.author_region = attributes.get_str(b"authorzone")?.into();

                    Ok(())
                })?;

                xml_reader.with_empty(b"desc", |attributes| {
                    if attributes.get(b"envir")? != b"Stadium" {
                        return Err("".into());
                    }

                    attributes.get(b"mood")?;

                    let has_script = match attributes.get(b"type")? {
                        b"Race" => false,
                        b"Script" => true,
                        _ => return Err("".into()),
                    };

                    self.params.ty = match attributes.get_str(b"maptype")? {
                        "TrackMania\\TM_Race" if !has_script => MapType::Race,
                        path if has_script => MapType::Script {
                            path: path.to_owned(),
                        },
                        _ => return Err("".into()),
                    };
                    attributes
                        .get_str(b"mapstyle")?
                        .clone_into(&mut self.params.style);
                    attributes.get(b"validated")?;
                    attributes.get(b"nblaps")?;
                    self.cost = attributes.get_u32(b"displaycost")?;
                    attributes.get(b"mod")?;
                    attributes.get(b"hasghostblocks")?;

                    Ok(())
                })?;

                xml_reader.with_empty(b"playermodel", |attributes| {
                    attributes.get(b"id")?;

                    Ok(())
                })?;

                xml_reader.with_empty(b"times", |attributes| {
                    let bronze_time = attributes.get_u32_or_null(b"bronze")?;
                    let silver_time = attributes.get_u32_or_null(b"silver")?;
                    let gold_time = attributes.get_u32_or_null(b"gold")?;
                    let author_time = attributes.get_u32_or_null(b"authortime")?;
                    attributes.get(b"authorscore")?;

                    if bronze_time != 0xffffffff
                        && silver_time != 0xffffffff
                        && gold_time != 0xffffffff
                        && author_time != 0xffffffff
                    {
                        if let Some(validation) = &mut self.params.validation {
                            validation.bronze_time = bronze_time;
                            validation.silver_time = silver_time;
                            validation.gold_time = gold_time;
                            validation.author_time = author_time;
                        } else {
                            self.params.validation = Some(Validation {
                                bronze_time,
                                silver_time,
                                gold_time,
                                author_time,
                                ghost: None,
                            });
                        }
                    }

                    Ok(())
                })?;

                xml_reader.until_end(b"deps")?;

                Ok(())
            },
        )?;

        xml_reader.eof()?;

        Ok(())
    }

    fn read_chunk_03043007<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        let thumbnail_size = d.u32()?;
        d.expect_bytes(b"<Thumbnail.jpg>")?;
        self.thumbnail = d.bytes(thumbnail_size as usize)?;
        d.expect_bytes(b"</Thumbnail.jpg>")?;
        d.expect_bytes(b"<Comments>")?;
        self.comments = d.string()?;
        d.expect_bytes(b"</Comments>")?;

        Ok(())
    }

    fn read_chunk_03043008<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let version = d.u32()?;

        if version != 1 {
            return Err("".into());
        }

        let author_version = d.u32()?;

        if author_version != 0 {
            return Err("".into());
        }

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

    fn read_chunk_03043011<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.unique_internal_node_ref::<CollectorList>()?;
        self.params = d.unique_internal_node_ref::<ChallengeParameters>()?;
        let _map_kind = d.u32()?; // 6

        Ok(())
    }

    fn read_chunk_03043018<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let _is_multilap = d.bool32()?; // 0
        let _num_laps = d.u32()?; // 3

        Ok(())
    }

    fn read_chunk_03043019<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let _mod = ExternalFileRef::read(d)?;

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
        self.size = Vec3 {
            x: d.u32()?,
            y: d.u32()?,
            z: d.u32()?,
        };
        d.u32()?; // 0
        let blocks_version = d.u32()?;

        if blocks_version != 6 {
            return Err("".into());
        }

        let num_blocks = d.u32()?;
        self.blocks = Vec::with_capacity(num_blocks as usize);
        while d.peek_u32()? & !ID_INDEX_MASK == ID_MARKER_BIT {
            let id = d.id()?;
            let direction = d.u8()?;
            let x = d.u8()?;
            let y = d.u8()?;
            let z = d.u8()?;
            let flags = d.u32()?;

            if flags == NULL {
                continue;
            }

            let skin = if flags & 0x00008000 != 0 {
                d.id()?; // "Nadeo"
                d.internal_node_ref_or_null::<BlockSkin>()?
            } else {
                None
            };

            let waypoint_property = if flags & 0x00100000 != 0 {
                Some(d.internal_node_ref::<WaypointSpecialProperty>()?)
            } else {
                None
            };

            let variant_index = if flags & 0x00200000 != 0 { 1 } else { 0 };

            let is_free = flags & 0x20000000 != 0;

            if is_free {
                self.blocks.push(Block {
                    id,
                    skin,
                    waypoint_property,
                    variant_index,
                    kind: BlockKind::Free(FreeBlock::default()),
                    elem_color: ElemColor::default(),
                    lightmap_quality: LightmapQuality::default(),
                });
            } else {
                let direction = Direction::try_from_u8(direction)?;

                let coord = Vec3 {
                    x: x - 1,
                    y,
                    z: z - 1,
                };

                let is_ground = flags & 0x00001000 != 0;

                let is_ghost = flags & 0x10000000 != 0;

                self.blocks.push(Block {
                    id,
                    skin,
                    waypoint_property,
                    variant_index,
                    kind: BlockKind::Normal(NormalBlock {
                        direction,
                        coord,
                        is_ground,
                        is_ghost,
                    }),
                    elem_color: ElemColor::default(),
                    lightmap_quality: LightmapQuality::default(),
                });
            }
        }

        Ok(())
    }

    fn read_chunk_03043022<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1 | 5

        Ok(())
    }

    fn read_chunk_03043024<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let _music = FileRef::read(d)?;

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
        self.comments = d.string()?;

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
        d.string()?;

        Ok(())
    }

    fn read_chunk_03043038<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0304303e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.expect_u32(10)?;
        d.list(|_| Ok(()))?;

        Ok(())
    }

    fn read_chunk_03043040<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let version = d.u32()?;

        if !matches!(version, 5 | 7) {
            return Err("".into());
        }

        d.expect_u32(0)?;
        d.scoped_buffer(|d| {
            d.expect_u32(10)?;
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

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_03043042<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let version = d.u32()?;

        if version != 1 {
            return Err("".into());
        }

        let author_version = d.u32()?;

        if author_version != 0 {
            return Err("".into());
        }

        self.author_id = d.string()?.into(); // "qYw071iWQXu9_jXI7SXEvA"
        self.author_name = d.string()?; // "YannexTM"
        self.author_region = d.string()?; // "World|Europe|Switzerland|Fribourg"
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03043043<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.scoped_buffer(|d| {
            d.list(|d| {
                d.node::<ZoneGenealogy>()?;

                Ok(())
            })?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_03043044<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.scoped_buffer(|d| {
            d.node::<TraitsMetadata>()?;

            Ok(())
        })?;

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

            let variant_index = if flags & 0x00200000 != 0 { 1 } else { 0 };

            let is_free = flags & 0x20000000 != 0;

            if is_free {
                Ok(Block {
                    id,
                    skin: None,
                    waypoint_property: None,
                    variant_index,
                    kind: BlockKind::Free(FreeBlock::default()),
                    elem_color: ElemColor::default(),
                    lightmap_quality: LightmapQuality::default(),
                })
            } else {
                let direction = Direction::try_from_u8(direction)?;

                let coord = Vec3 { x, y, z };

                let is_ground = flags & 0x00001000 != 0;

                let is_ghost = flags & 0x10000000 != 0;

                Ok(Block {
                    id,
                    skin: None,
                    waypoint_property: None,
                    variant_index,
                    kind: BlockKind::Normal(NormalBlock {
                        direction,
                        coord,
                        is_ground,
                        is_ghost,
                    }),
                    elem_color: ElemColor::default(),
                    lightmap_quality: LightmapQuality::default(),
                })
            }
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
        d.scoped_buffer(|d| {
            let ids = d.list(|d| {
                let id = d.id()?;
                d.u32()?; // 26
                d.id_or_null()?;

                Ok(id)
            })?;
            let size = d.u32()?;
            let data = d.bytes(size as usize)?;

            self.embedded_objects = Some(EmbeddedObjects { ids, data });

            d.u32()?; // 0

            Ok(())
        })?;

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
        d.read_to_end()?;

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

                free_block.position = Vec3 { x, y, z };

                let yaw = d.f32()?;
                let pitch = d.f32()?;
                let roll = d.f32()?;

                free_block.rotation = YawPitchRoll { yaw, pitch, roll };
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
            block.elem_color = ElemColor::read(d)?;
        }
        for item in &mut self.items {
            item.elem_color = ElemColor::read(d)?;
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
        for item in &mut self.items {
            if d.bool8()? {
                item.foreground_skin = FileRef::read(d)?;
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

impl<R: Read, I, N> ReadBody<R, I, N> for CollectorList {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for CollectorList {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for ChallengeParameters {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for ChallengeParameters {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
        let bronze_time = d.u32()?;
        let silver_time = d.u32()?;
        let gold_time = d.u32()?;
        let author_time = d.u32()?;
        d.u32()?;
        d.u32()?; // 0

        if bronze_time != 0xffffffff
            && silver_time != 0xffffffff
            && gold_time != 0xffffffff
            && author_time != 0xffffffff
        {
            if let Some(validation) = &mut self.validation {
                validation.bronze_time = bronze_time;
                validation.silver_time = silver_time;
                validation.gold_time = gold_time;
                validation.author_time = author_time;
            } else {
                self.validation = Some(Validation {
                    bronze_time,
                    silver_time,
                    gold_time,
                    author_time,
                    ghost: None,
                });
            }
        }

        Ok(())
    }

    fn read_chunk_0305b00d<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let validation_ghost = d.internal_node_ref_or_null::<Ghost>()?;

        if let Some(ghost) = validation_ghost {
            self.validation.as_mut().ok_or("validation is null")?.ghost = Some(ghost);
        }

        Ok(())
    }

    fn read_chunk_0305b00e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        self.ty = match d.string()? {
            path if path == "TrackMania\\TM_Race" => MapType::Race,
            path => MapType::Script { path },
        };
        self.style = d.string()?;
        d.u32()?; // 1

        Ok(())
    }
}

impl<R: Read, I, N> ReadBody<R, I, N> for BlockSkin {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for BlockSkin {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
        d.string()?;
        self.background = FileRef::read(d)?;
        InternalFileRef::read(d)?;

        Ok(())
    }

    fn read_chunk_03059003<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        self.foreground = FileRef::read(d)?;

        Ok(())
    }
}

type AnchoredObject = Item;

impl Class for AnchoredObject {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 257);
}

impl<R: Read, I: IdStateMut, N> ReadBody<R, I, N> for AnchoredObject {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N> BodyChunks<R, I, N> for AnchoredObject {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
    fn read_chunk_03101002<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 8
        self.id = d.id()?.into();
        d.u32()?; // 26
        d.id_or_null()?; // "qYw071iWQXu9_jXI7SXEvA"
        self.rotation = YawPitchRoll {
            yaw: d.f32()?,
            pitch: d.f32()?,
            roll: d.f32()?,
        };
        d.u16()?; // 0
        d.u8()?; // 0
        d.u32()?; // 0xffffffff
        self.position = Vec3 {
            x: d.f32()?,
            y: d.f32()?,
            z: d.f32()?,
        };
        self.waypoint_property = d.node_or_null::<WaypointSpecialProperty>()?;
        let flags = d.u16()?;
        self.pivot_position = Vec3 {
            x: d.f32()?,
            y: d.f32()?,
            z: d.f32()?,
        };
        d.u32()?;
        if flags & 0x0004 != 0 {
            self.background_skin = FileRef::read(d)?;
        }
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }

    fn read_chunk_03101004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_03101005<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 4
        d.u8()?; // 0

        Ok(())
    }
}

impl Direction {
    fn try_from_u8(x: u8) -> Result<Self> {
        let direction = match x {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => return Err("expected direction".into()),
        };

        Ok(direction)
    }
}

impl ElemColor {
    fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Self> {
        let elem_color = match d.u8()? {
            0 => Self::Default,
            1 => Self::White,
            2 => Self::Green,
            3 => Self::Blue,
            4 => Self::Red,
            5 => Self::Black,
            _ => return Err("expected element color".into()),
        };

        Ok(elem_color)
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
            _ => return Err("expected phase offset".into()),
        };

        Ok(phase_offset)
    }
}

mod xml {
    use std::{borrow::Cow, collections::HashMap, io::BufRead, str};

    use quick_xml::{events::Event, Reader};

    use crate::read::Result;

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
            mut attr_read_fn: impl FnMut(Attributes) -> Result<()>,
            mut inner_read_fn: impl FnMut(&mut Self) -> Result<()>,
        ) -> Result<()> {
            let tag = match self.reader.read_event_into(&mut self.buf).map_err(|_| "")? {
                Event::Start(tag) if tag.name().into_inner() == name => tag,
                _ => return Err("expected start".into()),
            };

            let mut attribute_map = HashMap::new();

            for attribute in tag.attributes() {
                let attribute = attribute.map_err(|_| "")?;

                attribute_map.insert(attribute.key.into_inner(), attribute.value);
            }

            let attributes = Attributes { map: attribute_map };

            attr_read_fn(attributes)?;

            inner_read_fn(self)?;

            match self.reader.read_event_into(&mut self.buf).map_err(|_| "")? {
                Event::End(tag) if tag.name().into_inner() == name => {}
                _ => return Err("expected end".into()),
            }

            Ok(())
        }

        pub fn with_empty(
            &mut self,
            name: &[u8],
            mut attr_read_fn: impl FnMut(Attributes) -> Result<()>,
        ) -> Result<()> {
            let tag = match self.reader.read_event_into(&mut self.buf).map_err(|_| "")? {
                Event::Empty(tag) if tag.name().into_inner() == name => tag,
                _ => return Err("expected empty".into()),
            };

            let mut attribute_map = HashMap::new();

            for attribute in tag.attributes() {
                let attribute = attribute.map_err(|_| "")?;

                attribute_map.insert(attribute.key.into_inner(), attribute.value);
            }

            let attributes = Attributes { map: attribute_map };

            attr_read_fn(attributes)?;

            Ok(())
        }

        pub fn eof(&mut self) -> Result<()> {
            match self.reader.read_event_into(&mut self.buf).map_err(|_| "")? {
                Event::Eof => {}
                _ => return Err("expected eof".into()),
            };

            Ok(())
        }

        pub fn until_end(&mut self, name: &[u8]) -> Result<()> {
            match self.reader.read_event_into(&mut self.buf).map_err(|_| "")? {
                Event::Start(tag) if tag.name().into_inner() == name => {}
                _ => return Err("expected start".into()),
            };

            loop {
                match self.reader.read_event_into(&mut self.buf).map_err(|_| "")? {
                    Event::End(tag) if tag.name().into_inner() == name => break,
                    _ => {}
                };
            }

            Ok(())
        }
    }

    pub struct Attributes<'a> {
        map: HashMap<&'a [u8], Cow<'a, [u8]>>,
    }

    impl Attributes<'_> {
        pub fn get(&self, key: &[u8]) -> Result<&[u8]> {
            match self.map.get(key) {
                None => Err("".into()),
                Some(value) => Ok(value.as_ref()),
            }
        }

        pub fn get_u32(&self, key: &[u8]) -> Result<u32> {
            let s = self.get_str(key)?;

            Ok(s.parse().map_err(|_| "")?)
        }

        pub fn get_u32_or_null(&self, key: &[u8]) -> Result<u32> {
            match self.get_str(key)? {
                "-1" => Ok(0xffffffff),
                s => Ok(s.parse().map_err(|_| "")?),
            }
        }

        pub fn get_str(&self, key: &[u8]) -> Result<&str> {
            match self.map.get(key) {
                None => Err("".into()),
                Some(value) => Ok(str::from_utf8(value).map_err(|_| "")?),
            }
        }
    }
}
