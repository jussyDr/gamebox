//! Block unit info

use crate::ClassId;

/// Block unit info.
#[derive(Default)]
pub struct BlockUnitInfo;

impl ClassId for BlockUnitInfo {
    const CLASS_ID: u32 = 0x03036000;
}

mod read {
    use crate::{
        Delme,
        class::game::{block_info_clip::BlockInfoClip, block_unit_info::BlockUnitInfo},
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for BlockUnitInfo {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for BlockUnitInfo {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(0, Self::read_chunk_0),
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::skippable(2, Self::read_chunk_2),
                BodyChunk::new(4, Self::read_chunk_4),
                BodyChunk::new(5, Self::read_chunk_5),
                BodyChunk::new(7, Self::read_chunk_7),
                BodyChunk::new(12, Self::read_chunk_12),
                BodyChunk::new(13, Self::read_chunk_13),
            ]
        }
    }

    impl BlockUnitInfo {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _place_pylons = r.u32()?;
            r.bool32()?;
            r.bool32()?;
            let _relative_offset = r.repeat(3, |r| r.u32())?;
            let _clips = r.list(|r| r.external_node_ref::<BlockInfoClip>())?;

            Ok(())
        }

        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _surface = r.id_or_null()?;
            let _frontier = r.u32()?;
            let _dir = r.u32()?;

            Ok(())
        }

        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _underground = r.bool32()?;

            Ok(())
        }

        fn read_chunk_4(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _accept_pylons = r.u32()?;

            Ok(())
        }

        fn read_chunk_5(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _terrain_modifier_id = r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_7(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _pylon_north = r.external_node_ref_or_null::<Delme>()?;
            let _pylon_south = r.external_node_ref_or_null::<Delme>()?;
            let _pylon_east = r.external_node_ref_or_null::<Delme>()?;
            let _pylon_west = r.external_node_ref_or_null::<Delme>()?;

            Ok(())
        }

        fn read_chunk_12(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_chunk_version(version));
            }

            let clip_count_bits = r.u32()?;
            let _clips_north = r.repeat((clip_count_bits & 0x00000007) as usize, |r| {
                r.external_node_ref::<BlockInfoClip>()
            })?;
            let _clips_east = r.repeat(((clip_count_bits >> 3) & 0x00000007) as usize, |r| {
                r.external_node_ref::<BlockInfoClip>()
            })?;
            let _clips_south = r.repeat(((clip_count_bits >> 6) & 0x00000007) as usize, |r| {
                r.external_node_ref::<BlockInfoClip>()
            })?;
            let _clips_west = r.repeat(((clip_count_bits >> 9) & 0x00000007) as usize, |r| {
                r.external_node_ref::<BlockInfoClip>()
            })?;
            let _clips_top = r.repeat(((clip_count_bits >> 12) & 0x00000007) as usize, |r| {
                r.external_node_ref::<BlockInfoClip>()
            })?;
            let _clips_bottom = r.repeat(((clip_count_bits >> 15) & 0x00000007) as usize, |r| {
                r.external_node_ref::<BlockInfoClip>()
            })?;
            r.u16()?;
            r.u16()?;

            Ok(())
        }

        fn read_chunk_13(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }
    }
}
