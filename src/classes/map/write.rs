use std::{f32::consts::FRAC_PI_4, io::Write};

use crate::{
    classes::map::MapType,
    serialize::{IdStateMut, NodeStateMut, Serializer},
    write::{
        writable::{HeaderChunk, HeaderChunks, Sealed, WriteBody},
        Result, Writable,
    },
};

use super::{ChallengeParameters, CollectorList, Map};

impl Writable for Map {}

impl Sealed for Map {}

impl HeaderChunks for Map {
    #[allow(clippy::redundant_closure)]
    fn header_chunks() -> impl Iterator<Item = HeaderChunk<Self>> {
        [
            HeaderChunk {
                chunk_id: 0x03043002,
                is_heavy: false,
                write_fn: |n, s| Self::write_chunk_2(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043003,
                is_heavy: false,
                write_fn: |n, s| Self::write_chunk_3(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043004,
                is_heavy: false,
                write_fn: |n, s| Self::write_chunk_4(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043005,
                is_heavy: true,
                write_fn: |n, s| Self::write_chunk_5(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043007,
                is_heavy: true,
                write_fn: |n, s| Self::write_chunk_7(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043008,
                is_heavy: false,
                write_fn: |n, s| Self::write_chunk_8(n, s),
            },
        ]
        .into_iter()
    }
}

impl<W: Write, I: IdStateMut, N: NodeStateMut> WriteBody<W, I, N> for Map {
    fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result {
        self.write_chunk_13(s)?;
        self.write_chunk_17(s)?;
        self.write_chunk_24(s)?;
        self.write_chunk_25(s)?;
        self.write_chunk_31(s)?;
        self.write_chunk_34(s)?;
        self.write_chunk_36(s)?;
        self.write_chunk_37(s)?;
        self.write_chunk_38(s)?;
        self.write_chunk_40(s)?;
        self.write_chunk_41(s)?;
        self.write_chunk_42(s)?;
        self.write_chunk_52(s)?;
        self.write_chunk_54(s)?;
        self.write_chunk_56(s)?;
        self.write_chunk_62(s)?;
        self.write_chunk_64(s)?;
        self.write_chunk_66(s)?;
        self.write_chunk_67(s)?;
        self.write_chunk_68(s)?;
        self.write_chunk_72(s)?;
        self.write_chunk_73(s)?;
        self.write_chunk_75(s)?;
        self.write_chunk_79(s)?;
        self.write_chunk_80(s)?;
        self.write_chunk_81(s)?;
        self.write_chunk_82(s)?;
        self.write_chunk_83(s)?;
        self.write_chunk_84(s)?;
        self.write_chunk_85(s)?;
        self.write_chunk_86(s)?;
        self.write_chunk_87(s)?;
        self.write_chunk_89(s)?;
        self.write_chunk_90(s)?;
        self.write_chunk_91(s)?;
        self.write_chunk_92(s)?;
        self.write_chunk_93(s)?;
        self.write_chunk_94(s)?;
        self.write_chunk_95(s)?;
        self.write_chunk_96(s)?;
        self.write_chunk_97(s)?;
        self.write_chunk_98(s)?;
        self.write_chunk_99(s)?;
        self.write_chunk_100(s)?;
        self.write_chunk_101(s)?;
        self.write_chunk_103(s)?;
        self.write_chunk_104(s)?;
        self.write_chunk_105(s)?;
        self.write_chunk_107(s)?;

        s.u32(0xfacade01)?;

        Ok(())
    }
}

impl Map {
    fn write_chunk_2<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u8(13)?;
        s.u32(0)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(self.cost)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(1)?;
        s.u32(1)?;

        Ok(())
    }

    fn write_chunk_3<W: Write, I: IdStateMut, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u8(11)?;
        s.id("clPHg9CHQjSqYP9wY4nRR6kSqM3")?;
        s.u32(26)?;
        s.id(&self.author_id)?;
        s.string("Empty")?;
        s.u8(6)?;
        s.u32(0)?;
        s.u32(0)?;
        s.id("48x48Day")?;
        s.u32(26)?;
        s.id("Nadeo")?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.string(match self.params.ty {
            MapType::Race => "TrackMania\\TM_Race",
            MapType::Script { ref path } => path,
        })?;
        s.string(&self.params.style)?;
        s.u32(0x4983cc85)?;
        s.u32(0xff58b673)?;
        s.u8(0)?;
        s.id("TMStadium")?;

        Ok(())
    }

    fn write_chunk_4<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(6)?;

        Ok(())
    }

    fn write_chunk_5<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        use quick_xml::{Error, Writer};

        s.buffer(|s| {
            let mut xml_writer = Writer::new(s.get_writer_mut());

            xml_writer
                .create_element("header")
                .with_attribute(("type", "map"))
                .with_attribute(("exever", "3.3.0"))
                .with_attribute(("exebuild", "2023-11-24_17_34"))
                .with_attribute(("title", "TMStadium"))
                .with_attribute(("lightmap", "0"))
                .write_inner_content::<_, Error>(|xml_writer| {
                    xml_writer
                        .create_element("ident")
                        .with_attribute(("uid", "clPHg9CHQjSqYP9wY4nRR6kSqM3"))
                        .with_attribute(("name", "Empty"))
                        .with_attribute(("author", self.author_name.as_str()))
                        .with_attribute(("authorzone", self.author_region.as_str()))
                        .write_empty()?;

                    xml_writer
                        .create_element("desc")
                        .with_attribute(("envir", "Stadium"))
                        .with_attribute(("mood", "Day"))
                        .with_attribute((
                            "type",
                            if matches!(self.params.ty, MapType::Race) {
                                "Race"
                            } else {
                                "Script"
                            },
                        ))
                        .with_attribute((
                            "maptype",
                            match self.params.ty {
                                MapType::Race => "TrackMania\\TM_Race",
                                MapType::Script { ref path } => path,
                            },
                        ))
                        .with_attribute(("mapstyle", self.params.style.as_str()))
                        .with_attribute((
                            "validated",
                            if self.params.validation.is_some() {
                                "1"
                            } else {
                                "0"
                            },
                        ))
                        .with_attribute(("nblaps", "0"))
                        .with_attribute(("displaycost", self.cost.to_string().as_str()))
                        .with_attribute(("mod", ""))
                        .with_attribute(("hasghostblocks", "0"))
                        .write_empty()?;

                    xml_writer
                        .create_element("playermodel")
                        .with_attribute(("id", ""))
                        .write_empty()?;

                    if let Some(ref medal_times) = self.params.validation {
                        xml_writer
                            .create_element("times")
                            .with_attribute((
                                "bronze",
                                medal_times.bronze_time.to_string().as_str(),
                            ))
                            .with_attribute((
                                "silver",
                                medal_times.silver_time.to_string().as_str(),
                            ))
                            .with_attribute(("gold", medal_times.gold_time.to_string().as_str()))
                            .with_attribute((
                                "authortime",
                                medal_times.author_time.to_string().as_str(),
                            ))
                            .with_attribute(("authorscore", "0"))
                            .write_empty()?;
                    } else {
                        xml_writer
                            .create_element("times")
                            .with_attribute(("bronze", "-1"))
                            .with_attribute(("silver", "-1"))
                            .with_attribute(("gold", "-1"))
                            .with_attribute(("authortime", "-1"))
                            .with_attribute(("authorscore", "0"))
                            .write_empty()?;
                    }

                    xml_writer
                        .create_element("deps")
                        .write_inner_content::<_, Error>(|_| Ok(()))?;

                    Ok(())
                })
                .unwrap();

            Ok(())
        })?;

        Ok(())
    }

    fn write_chunk_7<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(1)?;
        s.u32(self.thumbnail.len() as u32)?;
        s.bytes(b"<Thumbnail.jpg>")?;
        s.bytes(&self.thumbnail)?;
        s.bytes(b"</Thumbnail.jpg><Comments>")?;
        s.string(&self.comments)?;
        s.bytes(b"</Comments>")?;

        Ok(())
    }

    fn write_chunk_8<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(1)?;
        s.u32(0)?;
        s.string(&self.author_id)?;
        s.string(&self.author_name)?;
        s.string(&self.author_region)?;
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_13<W: Write, I: IdStateMut, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304300d)?;
        s.null_id()?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    fn write_chunk_17<W: Write, I: IdStateMut, N: NodeStateMut>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result {
        s.u32(0x03043011)?;
        s.unique_node_ref(&CollectorList)?;
        s.unique_node_ref(&self.params)?;
        s.u32(6)?;

        Ok(())
    }

    fn write_chunk_24<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043018)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(3)
        })?;

        Ok(())
    }

    fn write_chunk_25<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043019)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u8(3)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_31<W: Write, I: IdStateMut, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304301f)?;
        s.id("clPHg9CHQjSqYP9wY4nRR6kSqM3")?;
        s.u32(26)?;
        s.id(&self.author_id)?;
        s.string("Empty")?;
        s.id("48x48Day")?;
        s.u32(26)?;
        s.id("Nadeo")?;
        s.u32(self.size.x)?;
        s.u32(self.size.y)?;
        s.u32(self.size.z)?;
        s.u32(0)?;
        s.u32(6)?;
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_34<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043022)?;
        s.u32(1)?;

        Ok(())
    }

    fn write_chunk_36<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043024)?;
        s.u8(3)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_37<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043025)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_38<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043026)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    fn write_chunk_40<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043028)?;
        s.u32(0)?;
        s.string(&self.comments)?;

        Ok(())
    }

    fn write_chunk_41<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043029)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0xe151a5b1)
        })?;

        Ok(())
    }

    fn write_chunk_42<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304302a)?;
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_52<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043034)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| s.u32(0))?;

        Ok(())
    }

    fn write_chunk_54<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043036)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.f32(640.0)?;
            s.f32(181.01933)?;
            s.f32(640.0)?;
            s.f32(FRAC_PI_4)?;
            s.f32(FRAC_PI_4)?;
            s.u32(0)?;
            s.f32(90.0)?;
            s.f32(10.0)?;
            s.u32(0)?;
            s.f32(-1.0)?;
            s.f32(-1.0)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_56<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043038)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| s.u32(0))?;

        Ok(())
    }

    fn write_chunk_62<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304303e)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(10)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_64<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043040)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(7)?;
            s.u32(0)?;
            s.scoped_buffer(|s| {
                s.u32(10)?;
                s.u32(0)?;
                s.u32(0)?;
                s.u32(0)?;
                s.u32(0)?;
                s.u32(0)?;
                s.u32(0)?;
                s.u32(0)
            })
        })?;

        Ok(())
    }

    fn write_chunk_66<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043042)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(1)?;
            s.u32(0)?;
            s.string(&self.author_id)?;
            s.string(&self.author_name)?;
            s.string(&self.author_region)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_67<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043043)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.scoped_buffer(|s| {
                s.u32(2304)?;
                for _ in 0..2304 {
                    s.u32(0x0311d000)?;
                    s.u32(0x0311d002)?;
                    s.u32(1)?;
                    s.id("VoidToGrass")?;
                    s.u32(0)?;
                    s.u32(0)?;
                    s.id("Grass")?;
                    s.u32(0xfacade01)?;
                }

                Ok(())
            })
        })?;

        Ok(())
    }

    fn write_chunk_68<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043044)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.scoped_buffer(|s| {
                s.u32(0x11002000)?;
                s.u32(6)?;
                s.u8(2)?;
                s.u8(2)?;
                s.u8(7)?;
                s.u8(0)?;
                s.u8(2)?;
                s.u8(2)?;
                s.u8(25)?;
                s.bytes(b"LibMapType_MapTypeVersion")?;
                s.u8(0)?;
                s.u32(1)?;
                s.u8(28)?;
                s.bytes(b"Race_AuthorRaceWaypointTimes")?;
                s.u8(1)?;
                s.u8(0)?;
                s.u32(0xfacade01)
            })
        })?;

        Ok(())
    }

    fn write_chunk_72<W: Write, I: IdStateMut, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043048)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(6)?;
            s.u32(2304)?;
            for x in 1..=48 {
                for z in 1..=48 {
                    s.id("Grass")?;
                    s.u8(0)?;
                    s.u8(x)?;
                    s.u8(9)?;
                    s.u8(z)?;
                    s.u32(0x00001000)?;
                }
            }
            s.u32(0)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_73<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043049)?;
        s.u32(2)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(3)?;
        s.u32(1)?;
        s.u32(3)?;

        Ok(())
    }

    fn write_chunk_75<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304304b)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_79<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304304f)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u8(3)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_80<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043050)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(3)?;
            s.u32(1)?;
            s.u32(3)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_81<W: Write, I: IdStateMut, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043051)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.id("TMStadium")?;
            s.string("date=2023-11-24_17_34 git=126569-5ad9ff6053d GameVersion=3.3.0")
        })?;

        Ok(())
    }

    fn write_chunk_82<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043052)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(8)
        })?;

        Ok(())
    }

    fn write_chunk_83<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043053)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(3)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_84<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043054)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(1)?;
            s.u32(0)?;
            s.scoped_buffer(|s| {
                s.u32(0)?;
                s.u32(0)?;
                s.u32(0)
            })
        })?;

        Ok(())
    }

    fn write_chunk_85<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043055)?;
        s.u32(0x534b4950)?;
        s.buffer(|_| Ok(()))?;

        Ok(())
    }

    fn write_chunk_86<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043056)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(3)?;
            s.u32(0)?;
            s.u32(0xffffffff)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(300000)
        })?;

        Ok(())
    }

    fn write_chunk_87<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043057)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(5)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_89<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043059)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(3)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.f32(20.0)?;
            s.f32(3.0)
        })?;

        Ok(())
    }

    fn write_chunk_90<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304305a)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_91<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304305b)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(1)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(10)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_92<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304305c)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| s.u32(1))?;

        Ok(())
    }

    fn write_chunk_93<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304305d)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(1)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_94<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304305e)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(1)?;
            s.u32(0)?;
            s.u32(8)?;
            s.u32(0)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_95<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304305f)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| s.u32(0))?;

        Ok(())
    }

    fn write_chunk_96<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043060)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_97<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043061)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(1)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_98<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043062)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            for _ in 0..2304 {
                s.u8(0)?;
            }

            Ok(())
        })?;

        Ok(())
    }

    fn write_chunk_99<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043063)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| s.u32(0))?;

        Ok(())
    }

    fn write_chunk_100<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043064)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;
            s.u32(4)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_101<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043065)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| s.u32(0))?;

        Ok(())
    }

    fn write_chunk_103<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043067)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;
            s.u32(4)?;
            s.u32(0xffffffff)
        })?;

        Ok(())
    }

    fn write_chunk_104<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043068)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(1)?;
            for _ in 0..2304 {
                s.u8(0)?;
            }

            Ok(())
        })?;

        Ok(())
    }

    fn write_chunk_105<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x03043069)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_107<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0304306b)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0xffffffff)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(300000)
        })?;

        Ok(())
    }
}

impl<W: Write, I, N> WriteBody<W, I, N> for CollectorList {
    fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result {
        self.write_chunk_0(s)?;

        Ok(())
    }
}

impl CollectorList {
    fn write_chunk_0<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0301b000)?;
        s.u32(0)?;

        Ok(())
    }
}

impl<W: Write, I, N> WriteBody<W, I, N> for ChallengeParameters {
    fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result {
        self.write_chunk_1(s)?;
        self.write_chunk_4(s)?;
        self.write_chunk_8(s)?;
        self.write_chunk_10(s)?;
        self.write_chunk_13(s)?;
        self.write_chunk_14(s)?;

        Ok(())
    }
}

impl ChallengeParameters {
    fn write_chunk_1<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0305b001)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_4<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0305b004)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_8<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0305b008)?;
        s.u32(60000)?;
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_10<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0305b00a)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0xffffffff)?;
            s.u32(0xffffffff)?;
            s.u32(0xffffffff)?;
            s.u32(0xffffffff)?;
            s.u32(60000)?;
            s.u32(0)
        })?;

        Ok(())
    }

    fn write_chunk_13<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0305b00d)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    fn write_chunk_14<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x0305b00e)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.string(match self.ty {
                MapType::Race => "TrackMania\\TM_Race",
                MapType::Script { ref path } => path,
            })?;
            s.string(&self.style)?;
            s.u32(0)
        })?;

        Ok(())
    }
}
