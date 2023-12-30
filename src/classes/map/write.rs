use std::io::Write;

use crate::write::{
    serialize::{IdStateMut, NodeStateMut, Serializer},
    writable::{HeaderChunk, HeaderChunks, Sealed, WriteBody},
    Result, Writable,
};

use super::Map;

impl Writable for Map {}

impl Sealed for Map {}

impl HeaderChunks for Map {
    fn header_chunks() -> impl Iterator<Item = HeaderChunk<Self>> {
        [
            HeaderChunk {
                chunk_id: 0x03043002,
                write_fn: |n, s| Self::write_chunk_2(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043003,
                write_fn: |n, s| Self::write_chunk_3(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043004,
                write_fn: |n, s| Self::write_chunk_4(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043005,
                write_fn: |n, s| Self::write_chunk_5(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043007,
                write_fn: |n, s| Self::write_chunk_7(n, s),
            },
            HeaderChunk {
                chunk_id: 0x03043008,
                write_fn: |n, s| Self::write_chunk_8(n, s),
            },
        ]
        .into_iter()
    }
}

impl WriteBody for Map {
    fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result<()> {
        s.u32(0x0304300d)?;
        s.null_id()?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;

        s.u32(0x03043011)?;
        s.node_index()?;
        s.u32(0x0301b000)?;
        s.u32(0x0301b000)?;
        s.u32(0)?;
        s.u32(0xfacade01)?;
        s.node_index()?;
        s.u32(0x0305b000)?;
        s.u32(0x0305b001)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0x0305b004)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0)?;
        s.u32(0x0305b008)?;
        s.u32(60000)?;
        s.u32(0)?;
        s.u32(0x0305b00a)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(0xffffffff)?;
            s.u32(0xffffffff)?;
            s.u32(0xffffffff)?;
            s.u32(0xffffffff)?;
            s.u32(60000)?;
            s.u32(0)
        })?;
        s.u32(0x0305b00d)?;
        s.u32(0xffffffff)?;
        s.u32(0x0305b00e)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.string("TrackMania\\TM_Race")?;
            s.u32(0)?;
            s.u32(0)
        })?;
        s.u32(0xfacade01)?;
        s.u32(6)?;

        s.u32(0x03043018)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(3)
        })?;

        s.u32(0x03043019)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
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

        s.u32(0x0304301f)?;
        s.id("clPHg9CHQjSqYP9wY4nRR6kSqM3")?;
        s.u32(26)?;
        s.id(&self.author_id)?;
        s.string("Empty")?;
        s.id("48x48Day")?;
        s.u32(26)?;
        s.id("Nadeo")?;
        s.u32(48)?;
        s.u32(40)?;
        s.u32(48)?;
        s.u32(0)?;
        s.u32(6)?;
        s.u32(0)?;

        s.u32(0x03043022)?;
        s.u32(1)?;

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

        s.u32(0x03043025)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;

        s.u32(0x03043026)?;
        s.u32(0xffffffff)?;

        s.u32(0x03043028)?;
        s.u32(0)?;
        s.u32(0)?;

        s.u32(0x03043029)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0xe151a5b1)
        })?;

        s.u32(0x0304302a)?;
        s.u32(0)?;

        s.u32(0x03043034)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| s.u32(0))?;

        s.u32(0x03043036)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.f32(640.0)?;
            s.f32(181.0193328857421875)?;
            s.f32(640.0)?;
            s.f32(0.785398185253143310546875)?;
            s.f32(0.785398185253143310546875)?;
            s.u32(0)?;
            s.f32(90.0)?;
            s.f32(10.0)?;
            s.u32(0)?;
            s.f32(-1.0)?;
            s.f32(-1.0)?;
            s.u32(0)
        })?;

        s.u32(0x03043038)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| s.u32(0))?;

        s.u32(0x0304303e)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(10)?;
            s.u32(0)
        })?;

        s.u32(0x03043040)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(7)?;
            s.u32(0)?;
            s.something(|s| {
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

        s.u32(0x03043042)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(1)?;
            s.u32(0)?;
            s.string(&self.author_id)?;
            s.string(&self.author_name)?;
            s.string(&self.author_region)?;
            s.u32(0)
        })?;

        s.u32(0x03043043)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.something(|s| {
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

        s.u32(0x03043044)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.something(|s| {
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

        s.u32(0x03043048)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
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

        s.u32(0x0304304b)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)
        })?;

        s.u32(0x0304304f)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u8(3)?;
            s.u32(0)
        })?;

        s.u32(0x03043050)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(3)?;
            s.u32(1)?;
            s.u32(3)?;
            s.u32(0)
        })?;

        s.u32(0x03043051)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.id("TMStadium")?;
            s.string("date=2023-11-24_17_34 git=126569-5ad9ff6053d GameVersion=3.3.0")
        })?;

        s.u32(0x03043052)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(8)
        })?;

        s.u32(0x03043053)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(3)?;
            s.u32(0)
        })?;

        s.u32(0x03043054)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(1)?;
            s.u32(0)?;
            s.something(|s| {
                s.u32(0)?;
                s.u32(0)?;
                s.u32(0)
            })
        })?;

        s.u32(0x03043055)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|_| Ok(()))?;

        s.u32(0x03043056)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(3)?;
            s.u32(0)?;
            s.u32(0xffffffff)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(300000)
        })?;

        s.u32(0x03043057)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(5)?;
            s.u32(0)
        })?;

        s.u32(0x03043059)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(3)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.f32(20.0)?;
            s.f32(3.0)
        })?;

        s.u32(0x0304305a)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(0)
        })?;

        s.u32(0x0304305b)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(1)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(10)?;
            s.u32(0)
        })?;

        s.u32(0x0304305c)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| s.u32(1))?;

        s.u32(0x0304305d)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(1)?;
            s.u32(0)
        })?;

        s.u32(0x0304305e)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(1)?;
            s.u32(0)?;
            s.u32(8)?;
            s.u32(0)?;
            s.u32(0)
        })?;

        s.u32(0x0304305f)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| s.u32(0))?;

        s.u32(0x03043060)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(0)
        })?;

        s.u32(0x03043061)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(1)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)
        })?;

        s.u32(0x03043062)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            for _ in 0..2304 {
                s.u8(0)?;
            }

            Ok(())
        })?;

        s.u32(0x03043063)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| s.u32(0))?;

        s.u32(0x03043064)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;
            s.u32(4)?;
            s.u32(0)
        })?;

        s.u32(0x03043065)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| s.u32(0))?;

        s.u32(0x03043067)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;
            s.u32(4)?;
            s.u32(0xffffffff)
        })?;

        s.u32(0x03043068)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(1)?;
            for _ in 0..2304 {
                s.u8(0)?;
            }

            Ok(())
        })?;

        s.u32(0x03043069)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(0)
        })?;

        s.u32(0x0304306b)?;
        s.u32(0x534b4950)?;
        s.byte_buffer(|s| {
            s.u32(0)?;
            s.u32(0xffffffff)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(300000)
        })?;

        s.u32(0xfacade01)?;

        Ok(())
    }
}

impl Map {
    fn write_chunk_2<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result<()> {
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

    fn write_chunk_3<W: Write, I: IdStateMut, N>(&self, s: &mut Serializer<W, I, N>) -> Result<()> {
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
        s.string("TrackMania\\TM_Race")?;
        s.u32(0)?;
        s.u32(0x4983cc85)?;
        s.u32(0xff58b673)?;
        s.u8(0)?;
        s.id("TMStadium")?;

        Ok(())
    }

    fn write_chunk_4<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result<()> {
        s.u32(6)?;

        Ok(())
    }

    fn write_chunk_5<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result<()> {
        use quick_xml::{Error, Writer};

        s.byte_buffer(|s| {
            let mut xml_writer = Writer::new(s.get_mut());

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
                        .with_attribute(("type", "Race"))
                        .with_attribute(("maptype", "TrackMania\\TM_Race"))
                        .with_attribute(("mapstyle", ""))
                        .with_attribute(("validated", "0"))
                        .with_attribute(("nblaps", "0"))
                        .with_attribute(("displaycost", self.cost.to_string().as_str()))
                        .with_attribute(("mod", ""))
                        .with_attribute(("hasghostblocks", "0"))
                        .write_empty()?;

                    xml_writer
                        .create_element("playermodel")
                        .with_attribute(("id", ""))
                        .write_empty()?;

                    if let Some(ref medal_times) = self.medal_times {
                        xml_writer
                            .create_element("times")
                            .with_attribute(("bronze", medal_times.bronze.to_string().as_str()))
                            .with_attribute(("silver", medal_times.silver.to_string().as_str()))
                            .with_attribute(("gold", medal_times.gold.to_string().as_str()))
                            .with_attribute(("authortime", medal_times.author.to_string().as_str()))
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

    fn write_chunk_7<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result<()> {
        s.u32(1)?;
        s.u32(34375)?;
        s.bytes(b"<Thumbnail.jpg>")?;
        for _ in 0..34375 {
            s.u8(0)?;
        }
        s.bytes(b"</Thumbnail.jpg><Comments>")?;
        s.u32(0)?;
        s.bytes(b"</Comments>")?;

        Ok(())
    }

    fn write_chunk_8<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result<()> {
        s.u32(1)?;
        s.u32(0)?;
        s.string(&self.author_id)?;
        s.string(&self.author_name)?;
        s.string(&self.author_region)?;
        s.u32(0)?;

        Ok(())
    }
}
