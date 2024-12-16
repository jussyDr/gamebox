//! Challenge.

use std::sync::Arc;

use crate::{script::TraitsMetadata, Class, PackDesc, Vec3};

use super::{block::Block, AnchoredObject, ChallengeParameters, MediaClip, MediaClipGroup};

/// A challenge.
#[derive(Default)]
pub struct Challenge {
    bronze_time: u32,
    silver_time: u32,
    gold_time: u32,
    author_time: u32,
    cost: u32,
    id: Arc<str>,
    author_id: Arc<str>,
    name: String,
    deco_id: Arc<str>,
    parameters: Arc<ChallengeParameters>,
    texture_mod: Option<PackDesc>,
    size: Vec3<u32>,
    blocks: Vec<Block>,
    music: Option<PackDesc>,
    items: Vec<AnchoredObject>,
    script_metadata: TraitsMetadata,
    baked_blocks: Vec<Block>,
    intro_clip: Option<Arc<MediaClip>>,
    podium_clip: Option<Arc<MediaClip>>,
    in_game_clips: Option<Arc<MediaClipGroup>>,
    end_race_clips: Option<Arc<MediaClipGroup>>,
    ambiance_clip: Option<Arc<MediaClip>>,
    embedded_items: Option<EmbeddedItems>,
}

impl Class for Challenge {
    const CLASS_ID: u32 = 0x03043000;
}

impl Challenge {
    /// Bronze time.
    pub const fn bronze_time(&self) -> u32 {
        self.bronze_time
    }

    /// Silver time.
    pub const fn silver_time(&self) -> u32 {
        self.silver_time
    }

    /// Gold time.
    pub const fn gold_time(&self) -> u32 {
        self.gold_time
    }

    /// Author time.
    pub const fn author_time(&self) -> u32 {
        self.author_time
    }

    /// Cost.
    pub const fn cost(&self) -> u32 {
        self.cost
    }

    /// Identifier.
    pub const fn id(&self) -> &Arc<str> {
        &self.id
    }

    /// Author identifier.
    pub const fn author_id(&self) -> &Arc<str> {
        &self.author_id
    }

    /// Name.
    pub const fn name(&self) -> &String {
        &self.name
    }

    /// Decoration identifier.
    pub const fn deco_id(&self) -> &Arc<str> {
        &self.deco_id
    }

    /// Challenge parameters.
    pub const fn parameters(&self) -> &Arc<ChallengeParameters> {
        &self.parameters
    }

    /// Texture mod.
    pub const fn texture_mod(&self) -> Option<&PackDesc> {
        self.texture_mod.as_ref()
    }

    /// Size.
    pub const fn size(&self) -> &Vec3<u32> {
        &self.size
    }

    /// Blocks placed in this challenge.
    pub const fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    /// Music.
    pub const fn music(&self) -> Option<&PackDesc> {
        self.music.as_ref()
    }

    /// Items.
    pub const fn items(&self) -> &Vec<AnchoredObject> {
        &self.items
    }

    /// Script metadata.
    pub const fn script_metadata(&self) -> &TraitsMetadata {
        &self.script_metadata
    }

    /// Baked blocks.
    pub const fn baked_blocks(&self) -> &Vec<Block> {
        &self.baked_blocks
    }

    /// Intro media clip.
    pub const fn intro_clip(&self) -> Option<&Arc<MediaClip>> {
        self.intro_clip.as_ref()
    }

    /// Podium media clip.
    pub const fn podium_clip(&self) -> Option<&Arc<MediaClip>> {
        self.podium_clip.as_ref()
    }

    /// In game media clip group.
    pub const fn in_game_clips(&self) -> Option<&Arc<MediaClipGroup>> {
        self.in_game_clips.as_ref()
    }

    /// End race media clip group.
    pub const fn end_race_clips(&self) -> Option<&Arc<MediaClipGroup>> {
        self.end_race_clips.as_ref()
    }

    /// Ambiance media clip.
    pub const fn ambiance_clip(&self) -> Option<&Arc<MediaClip>> {
        self.ambiance_clip.as_ref()
    }

    /// Embedded items.
    pub const fn embedded_items(&self) -> Option<&EmbeddedItems> {
        self.embedded_items.as_ref()
    }
}

/// Embedded items.
pub struct EmbeddedItems {
    ids: Vec<Arc<str>>,
    zip: Vec<u8>,
}

impl EmbeddedItems {
    /// Identifiers.
    pub const fn ids(&self) -> &Vec<Arc<str>> {
        &self.ids
    }

    /// Zip.
    pub const fn zip(&self) -> &Vec<u8> {
        &self.zip
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::{
            block::{Block, BlockType},
            challenge_parameters::ChallengeParameters,
            collector_list::CollectorList,
            media_clip::MediaClip,
            media_clip_group::MediaClipGroup,
            zone_genealogy::ZoneGenealogy,
        },
        read::{
            read_body_chunks,
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody, Readable,
        },
        script::traits_metadata::TraitsMetadata,
    };

    use super::{Challenge, EmbeddedItems};

    impl Readable for Challenge {}

    impl Sealed for Challenge {}

    impl HeaderChunks for Challenge {
        fn header_chunks<R: Read, I: IdStateMut, N>(
        ) -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [
                HeaderChunk::new(2, Self::read_chunk_2),
                HeaderChunk::new(3, Self::read_chunk_3),
                HeaderChunk::new(4, Self::read_chunk_4),
                HeaderChunk::new(5, Self::read_chunk_5),
                HeaderChunk::new(7, Self::read_chunk_7),
                HeaderChunk::new(8, Self::read_chunk_8),
            ]
            .into_iter()
        }
    }

    impl ReadBody for Challenge {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Challenge {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(13, Self::read_chunk_13),
                BodyChunk::normal(17, Self::read_chunk_17),
                BodyChunk::skippable(24, Self::read_chunk_24),
                BodyChunk::skippable(25, Self::read_chunk_25),
                BodyChunk::normal(31, Self::read_chunk_31),
                BodyChunk::normal(34, Self::read_chunk_34),
                BodyChunk::normal(36, Self::read_chunk_36),
                BodyChunk::normal(37, Self::read_chunk_37),
                BodyChunk::normal(38, Self::read_chunk_38),
                BodyChunk::normal(40, Self::read_chunk_40),
                BodyChunk::skippable(41, Self::read_chunk_41),
                BodyChunk::normal(42, Self::read_chunk_42),
                BodyChunk::skippable(52, Self::read_chunk_52),
                BodyChunk::skippable(54, Self::read_chunk_54),
                BodyChunk::skippable(56, Self::read_chunk_56),
                BodyChunk::skippable(62, Self::read_chunk_62),
                BodyChunk::skippable(64, Self::read_chunk_64),
                BodyChunk::skippable(66, Self::read_chunk_66),
                BodyChunk::skippable(67, Self::read_chunk_67),
                BodyChunk::skippable(68, Self::read_chunk_68),
                BodyChunk::skippable(72, Self::read_chunk_72),
                BodyChunk::normal(73, Self::read_chunk_73),
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
                BodyChunk::skippable(97, Self::read_chunk_97),
                BodyChunk::skippable(98, Self::read_chunk_98),
                BodyChunk::skippable(99, Self::read_chunk_99),
                BodyChunk::skippable(100, Self::read_chunk_100),
                BodyChunk::skippable(101, Self::read_chunk_101),
                BodyChunk::skippable(103, Self::read_chunk_103),
                BodyChunk::skippable(104, Self::read_chunk_104),
                BodyChunk::skippable(105, Self::read_chunk_105),
                BodyChunk::skippable(107, Self::read_chunk_107),
                BodyChunk::skippable(108, Self::read_chunk_108),
            ]
            .into_iter()
        }
    }

    impl Challenge {
        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 13 {
                return Err(Error::chunk_version(version as u32));
            }

            r.bool()?;
            self.bronze_time = r.u32()?;
            self.silver_time = r.u32()?;
            self.gold_time = r.u32()?;
            self.author_time = r.u32()?;
            self.cost = r.u32()?;
            let _is_lap_race = r.bool()?;
            let _mode = r.u32()?;
            r.u32()?;
            let _author_score = r.u32()?;
            let _editor = r.u32()?;
            r.u32()?;
            let _num_checkpoints = r.u32()?;
            let _num_laps = r.u32()?;

            Ok(())
        }

        fn read_chunk_3<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 11 {
                return Err(Error::chunk_version(version as u32));
            }

            self.id = r.id()?;
            r.id_or_null()?;
            self.author_id = r.id()?;
            self.name = r.string()?;
            let _map_kind = r.u8()?;
            r.u32()?;
            let _password = r.string()?;
            self.deco_id = r.id()?;
            let _deco_collection = r.id_or_null()?;
            let _deco_author = r.id()?;
            let _map_coord_origin = r.vec2::<f32>()?;
            let _map_coord_target = r.vec2::<f32>()?;
            let _pack_mask = r.byte_array::<16>()?;
            let _map_type = r.string()?;
            let _map_style = r.string()?;
            let _lightmap_cache_uid = r.u64()?;
            let _lightmap_version = r.u8()?;
            let _title_id = r.id()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _version = r.u32()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _xml = r.string()?;

            Ok(())
        }

        fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let thumbnail_size = r.u32()?;
            r.byte_array::<15>()?;
            r.bytes(thumbnail_size as usize)?;
            r.byte_array::<16>()?;
            r.byte_array::<10>()?;
            r.string()?;
            r.byte_array::<11>()?;

            Ok(())
        }

        fn read_chunk_8<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let author_version = r.u32()?;

            if author_version != 0 {
                return Err(Error::version("author", author_version));
            }

            let _author_login = r.string()?;
            let _author_nickname = r.string()?;
            let _author_zone = r.string()?;
            let _author_extra_info = r.string()?;

            Ok(())
        }

        fn read_chunk_13<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _player_model_id = r.id_or_null()?;
            let _player_model_collection = r.id_or_null()?;
            let _player_model_author = r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_17(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _block_stock = r.internal_node_ref::<CollectorList>()?;
            self.parameters = r.internal_node_ref::<ChallengeParameters>()?;
            let _kind = r.u32()?;

            Ok(())
        }

        fn read_chunk_24<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _is_laps_race = r.bool()?;
            let _num_laps = r.u32()?;

            Ok(())
        }

        fn read_chunk_25<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.texture_mod = r.pack_desc_or_null()?;

            Ok(())
        }

        fn read_chunk_31(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.id = r.id()?;
            let _collection = r.id_or_null()?;
            self.author_id = r.id()?;
            self.name = r.string()?;
            self.deco_id = r.id()?;
            let _deco_collection = r.id_or_null()?;
            let _deco_author = r.id()?;
            self.size = r.vec3()?;
            let _need_unlock = r.bool()?;
            let blocks_version = r.u32()?;

            if blocks_version != 6 {
                return Err(Error::version("blocks", blocks_version));
            }

            let num_blocks = r.u32()? as usize;
            self.blocks = Vec::with_capacity(num_blocks);

            while r.peek_u32()? & 0x40000000 != 0 {
                let block = Block::read_from_body(r)?;

                if block.has_flags() {
                    self.blocks.push(block);
                }
            }

            Ok(())
        }

        fn read_chunk_34<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_36<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.music = r.pack_desc_or_null()?;

            Ok(())
        }

        fn read_chunk_37<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _map_coord_origin = r.vec2::<f32>()?;
            let _map_coord_target = r.vec2::<f32>()?;

            Ok(())
        }

        fn read_chunk_38<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_40<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let has_custom_cam_thumbnail = r.bool()?;

            if has_custom_cam_thumbnail {
                r.u8()?;
                r.vec3::<f32>()?;
                r.vec3::<f32>()?;
                r.vec3::<f32>()?;
                let _thumbnail_position = r.vec3::<f32>()?;
                let _thumbnail_fov = r.f32()?;
                let _thumbnail_near_clip_plame = r.f32()?;
                let _thumbnail_far_clip_plane = r.f32()?;
            }

            let _comments = r.string()?;

            Ok(())
        }

        fn read_chunk_41<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _hashed_password = r.byte_array::<16>()?;
            let _crc32 = r.u32()?;

            Ok(())
        }

        fn read_chunk_42<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _created_with_simple_editor = r.bool()?;

            Ok(())
        }

        fn read_chunk_52<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.byte_buf()?;

            Ok(())
        }

        fn read_chunk_54<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _thumbnail_position = r.vec3::<f32>()?;
            let _thumbnail_pitch_yaw_roll = r.vec3::<f32>()?;
            let _thumbnail_fov = r.f32()?;
            r.f32()?;
            r.f32()?;
            let _thumbnail_near_clip_plane = r.f32()?;
            let _thumbnail_far_clip_plane = r.f32()?;
            let _comments = r.string()?;

            Ok(())
        }

        fn read_chunk_56<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_62<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _car_marks_buffer = r.list_with_version(|_| Ok(()))?;

            Ok(())
        }

        fn read_chunk_64<I, N>(
            &mut self,
            r: &mut Reader<impl Read + Seek, I, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 5 | 7 | 8) {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            r.encapsulation(|r| {
                self.items = r.list_with_version(|r| r.node())?;

                if version == 7 {
                    let _items_on_item = r.list(|r| r.vec2::<u32>())?;
                }

                let _block_indices = r.list(|r| r.u32())?;

                if version >= 6 {
                    let _item_indices = r.list(|r| r.u32())?;
                }

                let _snap_item_groups = r.list(|r| r.u32())?;
                r.list(|r| r.u32())?;
                let _snapped_indexes = r.list(|r| r.u32())?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_66<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let author_version = r.u32()?;

            if author_version != 0 {
                return Err(Error::version("author", author_version));
            }

            let _author_login = r.string()?;
            let _author_nickname = r.string()?;
            let _author_zone = r.string()?;
            let _author_extra_info = r.string()?;

            Ok(())
        }

        fn read_chunk_67<I, N>(
            &mut self,
            r: &mut Reader<impl Read + Seek, I, N>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.encapsulation(|r| {
                r.list(|r| r.node::<ZoneGenealogy>())?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_68<I, N>(
            &mut self,
            r: &mut Reader<impl Read + Seek, I, N>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.encapsulation(|r| {
                self.script_metadata = TraitsMetadata::read_from_body(r)?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_72(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let blocks_version = r.u32()?;

            if blocks_version != 6 {
                return Err(Error::version("blocks", blocks_version));
            }

            self.baked_blocks = r.list(|r| Block::read_from_body(r))?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_73(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            self.intro_clip = r.internal_node_ref_or_null::<MediaClip>()?;
            self.podium_clip = r.internal_node_ref_or_null::<MediaClip>()?;
            self.in_game_clips = r.internal_node_ref_or_null::<MediaClipGroup>()?;
            self.end_race_clips = r.internal_node_ref_or_null::<MediaClipGroup>()?;
            self.ambiance_clip = r.internal_node_ref_or_null::<MediaClip>()?;
            let _clip_trigger_size = r.vec3::<u32>()?;

            Ok(())
        }

        fn read_chunk_75<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _objective_text_author = r.string()?;
            let _objective_text_gold = r.string()?;
            let _objective_text_silver = r.string()?;
            let _objective_text_bronze = r.string()?;

            Ok(())
        }

        fn read_chunk_79<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            r.u8()?;

            Ok(())
        }

        fn read_chunk_80<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _offzone_trigger_size = r.vec3::<u32>()?;
            let _offzones = r.list(|r| r.box3d())?;

            Ok(())
        }

        fn read_chunk_81<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _title_id = r.id()?;
            let _build_version = r.string()?;

            Ok(())
        }

        fn read_chunk_82<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _deco_base_height_offset = r.u32()?;

            Ok(())
        }

        fn read_chunk_83<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            let _bot_paths = r.u32()?;

            Ok(())
        }

        fn read_chunk_84<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            r.encapsulation(|r| {
                let ids = r.list(|r| {
                    let id = r.id()?;
                    r.id_or_null()?;
                    r.id()?;

                    Ok(id)
                })?;
                let zip = r.byte_buf()?;
                let _textures = r.list(|r| r.string())?;

                if !zip.is_empty() {
                    self.embedded_items = Some(EmbeddedItems { ids, zip })
                }

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_85<R, I, N>(&mut self, _: &mut Reader<R, I, N>) -> Result<(), Error> {
            Ok(())
        }

        fn read_chunk_86<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            let _day_time = r.u32()?;
            r.u32()?;
            let _dynamic_daylight = r.bool()?;
            let _day_duration = r.u32()?;

            Ok(())
        }

        fn read_chunk_87<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_88<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_89<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            let _world_distortion = r.vec3::<u32>()?;
            r.bool()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_90<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            if r.bool()? {
                r.u32()?;
                r.list(|r| r.u32())?;
                r.u32()?;
                r.u32()?;
                r.u8()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_91<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let has_lightmaps = r.bool()?;
            r.bool()?;
            r.bool()?;

            if has_lightmaps {
                let lightmaps_version = r.u32()?;

                if !matches!(lightmaps_version, 8 | 10) {
                    return Err(Error::version("lightmaps", lightmaps_version));
                }

                if r.u32()? != 0 {
                    let _webp = r.byte_buf()?;
                    let _webp = r.byte_buf()?;
                    let _webp = r.byte_buf()?;
                    let _webp = r.byte_buf()?;
                    r.u32()?;
                    r.u32()?;
                    let _webp = r.byte_buf()?;
                    r.u32()?;
                    r.u32()?;
                    let _lightmap_cache_data_size = r.u32()?;
                    let _compressed_lightmap_cache_data = r.byte_buf()?;
                }
            }

            Ok(())
        }

        fn read_chunk_92<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            if !r.bool()? {
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_93<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            if r.bool()? {
                r.u32()?; // 256
                let x = r.u32()?;

                match x {
                    221 => {
                        r.bytes(83844)?;
                    }
                    87 => {
                        r.bytes(51063)?;
                    }
                    _ => todo!("{x}"),
                }
            }

            Ok(())
        }

        fn read_chunk_94<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_95<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            for block in &mut self.blocks {
                if let BlockType::Free { position, rotation } = &mut block.ty {
                    *position = r.vec3()?;
                    *rotation = r.pitch_yaw_roll()?;
                }
            }

            for baked_block in &mut self.baked_blocks {
                if let BlockType::Free { position, rotation } = &mut baked_block.ty {
                    *position = r.vec3()?;
                    *rotation = r.pitch_yaw_roll()?;
                }
            }

            Ok(())
        }

        fn read_chunk_96<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_97<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_98<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            for block in &mut self.blocks {
                block.elem_color = r.enum_u8()?;
            }

            for baked_block in &mut self.baked_blocks {
                baked_block.elem_color = r.enum_u8()?;
            }

            for item in &mut self.items {
                item.elem_color = r.enum_u8()?;
            }

            Ok(())
        }

        fn read_chunk_99<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            for item in &mut self.items {
                item.anim_offset = r.enum_u8()?;
            }

            Ok(())
        }

        fn read_chunk_100<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_101<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            for item in &mut self.items {
                if r.bool8()? {
                    item.foreground_pack_desc = Some(r.pack_desc()?);
                }
            }

            Ok(())
        }

        fn read_chunk_103<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_104<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            for block in &mut self.blocks {
                block.lightmap_quality = r.enum_u8()?;
            }

            for baked_block in &mut self.baked_blocks {
                baked_block.lightmap_quality = r.enum_u8()?;
            }

            for item in &mut self.items {
                item.lightmap_quality = r.enum_u8()?;
            }

            Ok(())
        }

        fn read_chunk_105<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            for _ in &self.blocks {
                r.u32()?;
            }

            for _ in &self.items {
                r.u32()?;
            }

            let _id_flags_pair = r.list(|r| {
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_107<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            let _day_time = r.u32()?;
            r.u32()?;
            let _dynamic_daylight = r.bool()?;
            let _day_duration = r.u32()?;

            Ok(())
        }

        fn read_chunk_108<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u8()?;

            Ok(())
        }
    }
}

mod write {
    use crate::write::{writable, BodyChunk, BodyChunks, Writable};

    use self::writable::{HeaderChunk, HeaderChunks};

    use super::Challenge;

    impl Writable for Challenge {}

    impl writable::Sealed for Challenge {}

    impl HeaderChunks for Challenge {
        fn header_chunks<W, I, N>() -> impl Iterator<Item = HeaderChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }

    impl BodyChunks for Challenge {
        fn body_chunks<W, I, N>() -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
