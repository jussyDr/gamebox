//! Challenge

use crate::{ClassId, SubExtensions};

/// A challenge.
#[derive(Default)]
pub struct Challenge;

impl ClassId for Challenge {
    const CLASS_ID: u32 = 0x03043000;
}

impl SubExtensions for Challenge {
    const SUB_EXTENSIONS: &[&str] = &["Map"];
}

mod read {
    use crate::{
        class::game::ctn::{
            challenge::Challenge, challenge_parameters::ChallengeParameters,
            collector_list::CollectorList,
        },
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            error_unknown_chunk_version, error_unknown_version, read_body_chunks,
            reader::{BodyReader, HeaderReader},
        },
    };

    impl Readable for Challenge {}

    impl HeaderChunks for Challenge {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            [
                HeaderChunk::new(2, Self::read_chunk_2),
                HeaderChunk::new(3, Self::read_chunk_3),
                HeaderChunk::new(4, Self::read_chunk_4),
                HeaderChunk::new(5, Self::read_chunk_5),
                HeaderChunk::new(7, Self::read_chunk_7),
                HeaderChunk::new(8, Self::read_chunk_8),
            ]
        }
    }

    impl ReadBody for Challenge {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Challenge {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(13, Self::read_chunk_13),
                BodyChunk::new(17, Self::read_chunk_17),
            ]
        }
    }

    impl Challenge {
        fn read_chunk_2(&mut self, r: &mut impl HeaderReader) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 13 {
                return Err(error_unknown_chunk_version(version as u32));
            }

            let _need_unlock = r.bool32()?;
            let _bronze_time = r.u32()?;
            let _silver_time = r.u32()?;
            let _gold_time = r.u32()?;
            let _author_time = r.u32()?;
            let _cost = r.u32()?;
            let _is_lap_race = r.bool32()?;
            let _mode = r.u32()?;
            r.u32()?;
            let _author_score = r.u32()?;
            let _editor = r.u32()?;
            r.u32()?;
            let _num_checkpoints = r.u32()?;
            let _num_laps = r.u32()?;

            Ok(())
        }

        fn read_chunk_3(&mut self, r: &mut impl HeaderReader) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 11 {
                return Err(error_unknown_chunk_version(version as u32));
            }

            let _map_info = r.repeat(3, |r| r.id())?;
            let _map_name = r.string()?;
            let _kind_in_header = r.u8()?;
            r.u32()?;
            let _password = r.string()?;
            let _decoration = r.repeat(3, |r| r.id())?;
            let _map_coord_origin = r.vec2()?;
            let _map_coord_target = r.vec2()?;
            let _pack_mask = r.u128()?;
            let _map_type = r.string()?;
            let _map_style = r.string()?;
            let _lightmap_cache_uid = r.u64()?;
            let _lightmap_version = r.u8()?;
            let _title_id = r.id()?;

            Ok(())
        }

        fn read_chunk_4(&mut self, r: &mut impl HeaderReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(error_unknown_chunk_version(version));
            }

            Ok(())
        }

        fn read_chunk_5(&mut self, r: &mut impl HeaderReader) -> Result<(), Error> {
            let _xml = r.string()?;

            Ok(())
        }

        fn read_chunk_7(&mut self, r: &mut impl HeaderReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            let thumbnail_size = r.u32()?;

            if &r.byte_array()? != b"<Thumbnail.jpg>" {
                todo!()
            }

            let _thumbnail = r.bytes(thumbnail_size as usize)?;

            if &r.byte_array()? != b"</Thumbnail.jpg>" {
                todo!()
            }

            if &r.byte_array()? != b"<Comments>" {
                todo!()
            }

            let _comments = r.string()?;

            if &r.byte_array()? != b"</Comments>" {
                todo!()
            }

            Ok(())
        }

        fn read_chunk_8(&mut self, r: &mut impl HeaderReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            let author_version = r.u32()?;

            if author_version != 0 {
                return Err(error_unknown_version("author", author_version));
            }

            let _author_login = r.string()?;
            let _author_nickname = r.string()?;
            let _author_zone = r.string()?;
            let _author_extra_info = r.string()?;

            Ok(())
        }

        fn read_chunk_13(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _player_model = r.repeat(3, |r| r.id_or_null())?;

            Ok(())
        }

        fn read_chunk_17(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _block_stock = r.internal_node_ref::<CollectorList>()?;
            let _challenge_parameters = r.internal_node_ref::<ChallengeParameters>()?;
            let _kind = r.u32()?;

            Ok(())
        }
    }
}
