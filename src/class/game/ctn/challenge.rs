//! Challenge

use crate::{ClassId, SubExtensions, class::game::ctn::block::Block};

/// A challenge.
#[derive(Default)]
pub struct Challenge {
    blocks: Vec<Block>,
    baked_blocks: Vec<Block>,
}

impl ClassId for Challenge {
    const CLASS_ID: u32 = 0x03043000;
}

impl SubExtensions for Challenge {
    const SUB_EXTENSIONS: &[&str] = &["Map"];
}

mod read {
    use crate::{
        class::{
            game::ctn::{
                anchored_object::AnchoredObject, block::Block, challenge::Challenge,
                challenge_parameters::ChallengeParameters, collector_list::CollectorList,
                media_clip::MediaClip, media_clip_group::MediaClipGroup, read_file_ref,
                zone_genealogy::ZoneGenealogy,
            },
            script::traits_metadata::TraitsMetadata,
        },
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            error_unknown_chunk_version, error_unknown_version, read_body_chunks,
            read_node_from_body,
            reader::{BR, BodyReader, HeaderReader, IdTable, NodeTable, Reader},
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
                BodyChunk::skippable(24, Self::read_chunk_24),
                BodyChunk::skippable(25, Self::read_chunk_25),
                BodyChunk::new(31, Self::read_chunk_31),
                BodyChunk::new(34, Self::read_chunk_34),
                BodyChunk::new(36, Self::read_chunk_36),
                BodyChunk::new(37, Self::read_chunk_37),
                BodyChunk::new(38, Self::read_chunk_38),
                BodyChunk::new(40, Self::read_chunk_40),
                BodyChunk::skippable(41, Self::read_chunk_41),
                BodyChunk::new(42, Self::read_chunk_42),
                BodyChunk::skippable(52, Self::read_chunk_52),
                BodyChunk::skippable(54, Self::read_chunk_54),
                BodyChunk::skippable(56, Self::read_chunk_56),
                BodyChunk::skippable(62, Self::read_chunk_62),
                BodyChunk::skippable(64, Self::read_chunk_64),
                BodyChunk::skippable(66, Self::read_chunk_66),
                BodyChunk::skippable(67, Self::read_chunk_67),
                BodyChunk::skippable(68, Self::read_chunk_68),
                BodyChunk::skippable(72, Self::read_chunk_72),
                BodyChunk::new(73, Self::read_chunk_73),
                BodyChunk::skippable(75, Self::read_chunk_75),
                BodyChunk::skippable(79, Self::read_chunk_79),
                BodyChunk::skippable(80, Self::read_chunk_80),
                BodyChunk::skippable(81, Self::read_chunk_81),
                BodyChunk::skippable(82, Self::read_chunk_82),
                BodyChunk::skippable(83, Self::read_chunk_83),
                BodyChunk::skippable(84, Self::read_chunk_84),
                BodyChunk::skippable(85, Self::read_chunk_85),
                BodyChunk::skippable(86, Self::read_chunk_86),
                BodyChunk::skippable(87, Self::read_chunk_87),
                BodyChunk::skippable(88, Self::read_chunk_88),
                BodyChunk::skippable(89, Self::read_chunk_89),
                BodyChunk::skippable(90, Self::read_chunk_90),
                BodyChunk::skippable(91, Self::read_chunk_91),
                BodyChunk::skippable(92, Self::read_chunk_92),
                BodyChunk::skippable(93, Self::read_chunk_93),
                BodyChunk::skippable(94, Self::read_chunk_94),
                BodyChunk::skippable(95, Self::read_chunk_95),
                BodyChunk::skippable(96, Self::read_chunk_96),
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

        fn read_chunk_24(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _is_lap_race = r.bool32()?;
            let _num_laps = r.u32()?;

            Ok(())
        }

        fn read_chunk_25(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _texture_mod = read_file_ref(r)?;

            Ok(())
        }

        fn read_chunk_31(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _map_info = r.repeat(3, |r| r.id())?;
            let _map_name = r.string()?;
            let _decoration = r.repeat(3, |r| r.id())?;
            let _size = r.uvec3()?;
            let _need_unlock = r.bool32()?;
            let blocks_version = r.u32()?;

            if blocks_version != 6 {
                return Err(error_unknown_version("blocks", blocks_version));
            }

            self.blocks = r.list(|r| read_node_from_body::<Block>(r))?;

            Ok(())
        }

        fn read_chunk_34(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_36(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _music_file_ref = read_file_ref(r)?;

            Ok(())
        }

        fn read_chunk_37(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _map_coord_origin = r.vec2()?;
            let _map_coord_target = r.vec2()?;

            Ok(())
        }

        fn read_chunk_38(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_40(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            if r.bool32()? {
                todo!();
            }

            let _comments = r.string()?;

            Ok(())
        }

        fn read_chunk_41(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _password_hash = r.u128()?;
            let _crc32 = r.u32()?;

            Ok(())
        }

        fn read_chunk_42(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _created_with_simple_editor = r.bool32()?;

            Ok(())
        }

        fn read_chunk_52(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.byte_buf()?;

            Ok(())
        }

        fn read_chunk_54(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _thumbnail_position = r.vec3()?;
            let _thumbnail_pitch_yaw_roll = r.vec3()?;
            let _thumbnail_fov = r.f32()?;
            r.f32()?;
            r.f32()?;
            let _thumbnail_near_clip_plane = r.f32()?;
            let _thumbnail_far_clip_plane = r.f32()?;
            let _comments = r.string()?;

            Ok(())
        }

        fn read_chunk_56(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_62(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let _car_marks_buffer: Vec<()> = r.list_with_version(|r| todo!())?;

            Ok(())
        }

        fn read_chunk_64(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;
            let _size = r.u32()?;

            let mut r = BR {
                reader: r,
                id_table: IdTable::new(),
                node_table: NodeTable::new(0),
            };

            let _anchored_objects = r.list_with_version(|r| r.node::<AnchoredObject>())?;
            let _block_indices = r.list(|r| r.u32())?;
            let _snap_item_groups = r.list(|r| r.u32())?;
            r.list(|r| r.u32())?;
            let _snapped_indices = r.list(|r| r.u32())?;

            Ok(())
        }

        fn read_chunk_66(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
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

        fn read_chunk_67(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            let _size = r.u32()?;

            let mut r = BR {
                reader: r,
                id_table: IdTable::new(),
                node_table: NodeTable::new(0),
            };

            let _zone_genealogy = r.list(|r| r.node::<ZoneGenealogy>())?;

            Ok(())
        }

        fn read_chunk_68(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            let _size = r.u32()?;

            let mut r = BR {
                reader: r,
                id_table: IdTable::new(),
                node_table: NodeTable::new(0),
            };

            let _script_metadata = read_node_from_body::<TraitsMetadata>(&mut r)?;

            Ok(())
        }

        fn read_chunk_72(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let blocks_version = r.u32()?;

            if blocks_version != 6 {
                return Err(error_unknown_version("blocks", blocks_version));
            }

            self.baked_blocks = r.list(|r| read_node_from_body::<Block>(r))?;
            r.u32()?;
            let _baked_clips_additional_data = r.u32()?;

            Ok(())
        }

        fn read_chunk_73(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_chunk_version(version));
            }

            let _intro_clip = r.internal_node_ref::<MediaClip>()?;
            let _podium_clip = r.internal_node_ref_or_null::<MediaClip>()?;
            let _in_game_clips = r.internal_node_ref::<MediaClipGroup>()?;
            let _end_race_clips = r.internal_node_ref_or_null::<MediaClipGroup>()?;
            let _ambiance_clip = r.internal_node_ref::<MediaClip>()?;
            let _clip_trigger_size = r.uvec3()?;

            Ok(())
        }

        fn read_chunk_75(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _objective_text_author = r.string()?;
            let _objective_text_gold = r.string()?;
            let _objective_text_silver = r.string()?;
            let _objective_text_bronze = r.string()?;

            Ok(())
        }

        fn read_chunk_79(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u8()?;

            Ok(())
        }

        fn read_chunk_80(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let _offzone_trigger_size = r.uvec3()?;
            let _offzones = r.list(|r| r.box3d())?;

            Ok(())
        }

        fn read_chunk_81(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let _title_id = r.id()?;
            let _build_version = r.string()?;

            Ok(())
        }

        fn read_chunk_82(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let _deco_base_height_offset = r.u32()?;

            Ok(())
        }

        fn read_chunk_83(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            let _bot_paths: Vec<()> = r.list(|r| todo!())?;

            Ok(())
        }

        fn read_chunk_84(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;
            let _size = r.u32()?;

            let mut r = BR {
                reader: r,
                id_table: IdTable::new(),
                node_table: NodeTable::new(0),
            };

            let _embedded_item_models = r.list(|r| r.repeat(3, |r| r.id()))?;
            let _embedded_zip_data = r.byte_buf()?;
            let _textures = r.list(|r| r.string())?;

            Ok(())
        }

        fn read_chunk_85(&mut self, _: &mut impl BodyReader) -> Result<(), Error> {
            Ok(())
        }

        fn read_chunk_86(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;
            let _day_time = r.u32()?;
            r.u32()?;
            let _dynamic_daylight = r.f32()?;
            let _day_duration = r.u32()?;

            Ok(())
        }

        fn read_chunk_87(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_88(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_89(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            let _world_distortion = r.vec3()?;

            if r.bool32()? {
                todo!()
            }

            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_90(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_91(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let has_lightmaps = r.bool32()?;
            r.bool32()?;
            r.bool32()?;

            if has_lightmaps {
                let lightmap_version = r.u32()?;

                if lightmap_version != 8 {
                    return Err(error_unknown_version("lightmap", lightmap_version));
                }

                let _lightmap_frames: Vec<()> = r.list(|r| {
                    r.byte_buf()?;
                    r.byte_buf()?;
                    r.byte_buf()?;

                    Ok(())
                })?;

                let _size = r.u32()?;
                r.byte_buf()?;
            }

            Ok(())
        }

        fn read_chunk_92(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_93(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_94(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_95(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            for block in &mut self.blocks {
                if block.is_free {
                    let _absolute_position_in_map = r.vec3()?;
                    let _yaw_pitch_roll = r.vec3()?;
                }
            }

            for block in &mut self.baked_blocks {
                if block.is_free {
                    let _absolute_position_in_map = r.vec3()?;
                    let _yaw_pitch_roll = r.vec3()?;
                }
            }

            Ok(())
        }

        fn read_chunk_96(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }
    }
}
