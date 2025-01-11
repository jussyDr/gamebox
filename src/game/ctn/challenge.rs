//! Challenge.

use std::sync::Arc;

use crate::{
    read::reader::FromVariant, script::TraitsMetadata, Byte3, Class, FileRef, Nat3, Vec3,
    YawPitchRoll,
};

use super::{
    block::{Block, BlockType},
    AnchoredObject, Direction, ElemColor, Ghost, LightmapQuality, MediaClip, MediaClipGroup,
    ZoneGenealogy,
};

/// Challenge.
#[derive(PartialEq, Debug)]
pub struct Challenge {
    validation: Option<Validation>,
    cost: u32,
    play_mode: u32,
    editor_mode: EditorMode,
    num_checkpoints: u32,
    num_laps: Option<u32>,
    id: Arc<str>,
    author_id: Arc<str>,
    author_name: String,
    name: String,
    ty: ChallengeType,
    password: String,
    decoration_id: Arc<str>,
    pack_mask: [u8; 16],
    map_type: String,
    map_style: Option<String>,
    has_ghost_blocks: bool,
    lightmap_cache_id: u64,
    has_lightmap: bool,
    title_id: Arc<str>,
    game_build_date: String,
    game_build_tag: GameBuildTag,
    thumbnail: Vec<u8>,
    author_zone: String,
    texture_mod: Option<FileRef>,
    size: Nat3,
    blocks: Vec<Block>,
    music: Option<FileRef>,
    items: Vec<AnchoredObject>,
    zones: Vec<ZoneGenealogy>,
    script_metadata: TraitsMetadata,
    baked_blocks: Vec<Block>,
    intro_clip: Option<Arc<MediaClip>>,
    podium_clip: Option<Arc<MediaClip>>,
    in_game_clips: Option<Arc<MediaClipGroup>>,
    end_race_clips: Option<Arc<MediaClipGroup>>,
    ambiance_clip: Option<Arc<MediaClip>>,
    embedded_items: Option<EmbeddedItems>,
    password_hash: Option<[u8; 16]>,
    checksum: u32,
    thumbnail_position: Vec3,
    thumbnail_rotation: YawPitchRoll,
    thumbnail_fov: f32,
    time_of_day: Option<u32>,
}

impl Class for Challenge {
    const CLASS_ID: u32 = 0x03043000;
}

impl Challenge {
    /// Validation.
    pub const fn validation(&self) -> Option<&Validation> {
        self.validation.as_ref()
    }

    /// Display cost.
    pub const fn cost(&self) -> u32 {
        self.cost
    }

    /// Number of checkpoints.
    pub const fn num_checkpoints(&self) -> u32 {
        self.num_checkpoints
    }

    /// Number of laps, or `None` if the challenge is not a laps race.
    pub const fn num_laps(&self) -> Option<u32> {
        self.num_laps
    }

    /// Unique identifier.
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
    pub const fn decoration_id(&self) -> &Arc<str> {
        &self.decoration_id
    }

    /// Title identifier.
    pub const fn title_id(&self) -> &Arc<str> {
        &self.title_id
    }

    /// Custom texture mod.
    pub const fn texture_mod(&self) -> Option<&FileRef> {
        self.texture_mod.as_ref()
    }

    /// Size.
    pub const fn size(&self) -> Nat3 {
        self.size
    }

    /// Blocks.
    pub const fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    /// Custom music.
    pub const fn music(&self) -> Option<&FileRef> {
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

    /// Baked blocks, containing clips and grass blocks.
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

    /// In game media clips.
    pub const fn in_game_clips(&self) -> Option<&Arc<MediaClipGroup>> {
        self.in_game_clips.as_ref()
    }

    /// End race media clips.
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

    /// Thumbnail position.
    pub const fn thumbnail_position(&self) -> Vec3 {
        self.thumbnail_position
    }

    /// Thumbnail rotation.
    pub const fn thumbnail_rotation(&self) -> YawPitchRoll {
        self.thumbnail_rotation
    }

    /// Thumbnail field-of-view.
    pub const fn thumbnail_fov(&self) -> f32 {
        self.thumbnail_fov
    }
}

impl Default for Challenge {
    fn default() -> Self {
        let size = 48;

        let mut baked_blocks = vec![];

        let model_id = Arc::from("Grass");

        for x in 0..size {
            for z in 0..size {
                baked_blocks.push(Block {
                    model_id: Arc::clone(&model_id),
                    ty: BlockType::Normal {
                        direction: Direction::North,
                        coord: Byte3::new(x, 0, z),
                        is_ghost: false,
                    },
                    has_flags: true,
                    mobil_index: 0,
                    mobil_sub_index: 0,
                    is_ground: true,
                    is_pillar: false,
                    skin: None,
                    waypoint_property: None,
                    variant_index: 0,
                    elem_color: ElemColor::Default,
                    lightmap_quality: LightmapQuality::Normal,
                });
            }
        }

        Self {
            validation: None,
            cost: 0,
            play_mode: 0,
            editor_mode: EditorMode::default(),
            num_checkpoints: 0,
            num_laps: None,
            author_name: String::new(),
            id: Default::default(), // should be random
            author_id: Default::default(),
            name: Default::default(),
            ty: ChallengeType::InProgress,
            password: String::new(),
            decoration_id: Arc::from("48x48Screen155Day"),
            pack_mask: [0; 16],
            map_type: "TrackMania\\TM_Race".to_string(),
            map_style: None,
            lightmap_cache_id: 0, // should be random
            has_lightmap: false,
            has_ghost_blocks: false,
            thumbnail: vec![],
            title_id: Arc::from("TMStadium"),
            author_zone: String::new(),
            texture_mod: None,
            size: Nat3::new(size as u32, 40, size as u32),
            blocks: vec![],
            music: None,
            zones: vec![],
            items: vec![],
            script_metadata: TraitsMetadata::default(),
            baked_blocks,
            intro_clip: None,
            podium_clip: None,
            in_game_clips: None,
            end_race_clips: None,
            ambiance_clip: None,
            embedded_items: None,
            game_build_date: "2024-09-17_11_17".to_string(),
            game_build_tag: GameBuildTag::Git("127252-120dea21a9e".to_string()),
            password_hash: None,
            checksum: 0, // generate
            thumbnail_position: Vec3::new(0.0, 0.0, 0.0),
            thumbnail_rotation: YawPitchRoll::new(0.0, 0.0, 0.0),
            thumbnail_fov: 90.0,
            time_of_day: None,
        }
    }
}

/// Validation.
#[derive(PartialEq, Debug)]
pub struct Validation {
    objective: Objective,
    ghost: Option<Arc<Ghost>>,
}

impl Validation {
    /// Objective.
    pub const fn objective(&self) -> &Objective {
        &self.objective
    }

    /// Ghost.
    pub const fn ghost(&self) -> Option<&Arc<Ghost>> {
        self.ghost.as_ref()
    }
}

/// Objective.
#[derive(PartialEq, Debug)]
pub enum Objective {
    /// Medal times.
    MedalTimes(MedalTimes),
    /// Author score.
    AuthorScore(u32),
}

/// Medal times.
#[derive(PartialEq, Debug)]
pub struct MedalTimes {
    bronze_time: u32,
    silver_time: u32,
    gold_time: u32,
    author_time: u32,
}

impl MedalTimes {
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
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum EditorMode {
    #[default]
    Advanced,
    Simple,
    HasGhostBlocks,
    Gamepad,
}

impl FromVariant<u32> for EditorMode {
    fn from_variant(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Advanced),
            1 => Some(Self::Simple),
            2 => Some(Self::HasGhostBlocks),
            4 => Some(Self::Gamepad),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum ChallengeType {
    #[default]
    EndMarker,
    Campaign,
    Puzzle,
    Retro,
    TimeAttack,
    Rounds,
    InProgress,
    Campaign7,
    Multi,
    Solo,
    Site,
    SoloNadeo,
    MultiNadeo,
}

impl FromVariant<u8> for ChallengeType {
    fn from_variant(value: u8) -> Option<Self> {
        Self::from_variant(value as u32)
    }
}

impl FromVariant<u32> for ChallengeType {
    fn from_variant(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::EndMarker),
            1 => Some(Self::Campaign),
            2 => Some(Self::Puzzle),
            3 => Some(Self::Retro),
            4 => Some(Self::TimeAttack),
            5 => Some(Self::Rounds),
            6 => Some(Self::InProgress),
            7 => Some(Self::Campaign7),
            8 => Some(Self::Multi),
            9 => Some(Self::Solo),
            10 => Some(Self::Site),
            11 => Some(Self::SoloNadeo),
            12 => Some(Self::MultiNadeo),
            _ => None,
        }
    }
}

/// Embedded items.
#[derive(PartialEq, Debug)]
pub struct EmbeddedItems {
    ids: Vec<Arc<str>>,
    zip_archive: Vec<u8>,
}

impl EmbeddedItems {
    /// Identifiers.
    pub const fn ids(&self) -> &Vec<Arc<str>> {
        &self.ids
    }

    /// Zip archive.
    pub const fn zip_archive(&self) -> &Vec<u8> {
        &self.zip_archive
    }
}

/// Game build tag.
#[derive(PartialEq, Debug)]
pub enum GameBuildTag {
    /// Git.
    Git(String),
    /// Svn.
    Svn(String),
}

const GAME_VERSION: &str = "3.3.0";

mod read {
    use std::{
        borrow::Cow,
        io::{BufRead, Read, Seek},
        str::FromStr,
        sync::Arc,
    };

    use quick_xml::events::{attributes::Attributes, Event};

    use crate::{
        game::ctn::{
            block::{Block, BlockType},
            challenge_parameters::ChallengeParameters,
            collector_list::CollectorList,
            media_clip::MediaClip,
            media_clip_group::MediaClipGroup,
        },
        read::{
            read_body_chunks,
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{string_non_empty, IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ErrorKind, ReadBody, Readable,
        },
        script::traits_metadata::TraitsMetadata,
        ID_MARKER_BIT,
    };

    use super::{
        Challenge, EmbeddedItems, GameBuildTag, MedalTimes, Objective, Validation, GAME_VERSION,
    };

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
            let bronze_time = r.u32_or_null()?;
            let silver_time = r.u32_or_null()?;
            let gold_time = r.u32_or_null()?;
            let author_time = r.u32_or_null()?;
            self.cost = r.u32()?;
            let is_lap_race = r.bool()?;
            self.play_mode = r.u32()?;
            r.u32()?;
            let author_score = r.u32_or_zero()?;
            self.editor_mode = r.enum_u32()?;
            r.u32()?;
            self.num_checkpoints = r.u32()?;
            let num_laps = r.u32()?;

            if let (Some(bronze_time), Some(silver_time), Some(gold_time), Some(author_time)) =
                (bronze_time, silver_time, gold_time, author_time)
            {
                self.validation = Some(Validation {
                    objective: Objective::MedalTimes(MedalTimes {
                        bronze_time,
                        silver_time,
                        gold_time,
                        author_time,
                    }),
                    ghost: None,
                })
            } else if let Some(author_score) = author_score {
                self.validation = Some(Validation {
                    objective: Objective::AuthorScore(author_score),
                    ghost: None,
                })
            } else {
                self.validation = None;
            }

            self.num_laps = is_lap_race.then_some(num_laps);

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
            self.ty = r.enum_u8()?;
            r.u32()?;
            self.password = r.string()?;
            self.decoration_id = r.id()?;
            let _deco_collection = r.id_or_null()?;
            let _deco_author = r.id()?;
            let _coord_origin = r.vec2()?;
            let _coord_target = r.vec2()?;
            self.pack_mask = r.byte_array::<16>()?;
            self.map_type = r.string()?;
            self.map_style = r.string_or_empty()?;
            self.lightmap_cache_id = r.u64()?;
            self.has_lightmap = has_lightmap(r.u8()?)?;
            self.title_id = r.id()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(Error::version("challenge", version));
            }

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let xml = r.byte_buf()?;
            let mut r = XmlReader::new(xml.as_slice());

            r.tag(
                b"header",
                |r| {
                    let ty = r.attribute(b"type")?;

                    if ty != "map" {
                        todo!()
                    }

                    let game_version = r.attribute(b"exever")?;

                    if game_version != GAME_VERSION {
                        todo!()
                    }

                    self.game_build_date = r.attribute(b"exebuild")?.to_string();
                    let _title_id = r.attribute(b"title")?;
                    self.has_lightmap = has_lightmap(r.attribute_from_str(b"lightmap")?)?;

                    Ok(())
                },
                |r| {
                    r.tag_empty(b"ident", |r| {
                        self.id = Arc::from(r.attribute(b"uid")?);
                        self.name = r.attribute(b"name")?.to_string();
                        self.author_id = Arc::from(r.attribute(b"author")?);
                        self.author_zone = r.attribute(b"authorzone")?.to_string();

                        Ok(())
                    })?;
                    r.tag_empty(b"desc", |r| {
                        let _environment = r.attribute(b"envir")?;
                        let _mood = r.attribute(b"mood")?;
                        let _type = r.attribute(b"type")?;
                        self.map_type = r.attribute(b"maptype")?.to_string();
                        self.map_style = string_non_empty(r.attribute(b"mapstyle")?.to_string());
                        let _is_validated = r.attribute(b"validated")?;
                        let _num_laps = r.attribute(b"nblaps")?;
                        self.cost = r.attribute_from_str(b"displaycost")?;
                        let _texture_mod = r.attribute(b"mod")?;
                        self.has_ghost_blocks = match r.attribute(b"hasghostblocks")?.as_ref() {
                            "0" => false,
                            "1" => true,
                            _ => todo!(),
                        };

                        Ok(())
                    })?;
                    r.tag_empty(b"playermodel", |r| {
                        let _player_model_id = r.attribute(b"id")?;

                        Ok(())
                    })?;
                    r.tag_empty(b"times", |r| {
                        let bronze_time: i32 = r.attribute_from_str(b"bronze")?;
                        let silver_time: i32 = r.attribute_from_str(b"silver")?;
                        let gold_time: i32 = r.attribute_from_str(b"gold")?;
                        let author_time: i32 = r.attribute_from_str(b"authortime")?;
                        let author_score: u32 = r.attribute_from_str(b"authorscore")?;

                        if bronze_time != -1
                            && silver_time != -1
                            && gold_time != -1
                            && author_time != -1
                        {
                            self.validation = Some(Validation {
                                objective: Objective::MedalTimes(MedalTimes {
                                    bronze_time: bronze_time as u32,
                                    silver_time: silver_time as u32,
                                    gold_time: gold_time as u32,
                                    author_time: author_time as u32,
                                }),
                                ghost: None,
                            })
                        } else if author_score != 0 {
                            self.validation = Some(Validation {
                                objective: Objective::AuthorScore(author_score),
                                ghost: None,
                            })
                        }

                        Ok(())
                    })?;
                    r.tag_list(b"deps", b"dep", |r| {
                        r.attribute(b"file")?;
                        r.optional_attribute(b"url")?;

                        Ok(())
                    })?;

                    Ok(())
                },
            )?;

            Ok(())
        }

        fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let thumbnail_size = r.u32()?;

            if &r.byte_array()? != b"<Thumbnail.jpg>" {
                todo!()
            }

            self.thumbnail = r.bytes(thumbnail_size as usize)?;

            if &r.byte_array()? != b"</Thumbnail.jpg>" {
                todo!()
            }

            if &r.byte_array()? != b"<Comments>" {
                todo!()
            }

            let _comments = r.string_or_empty()?;

            if &r.byte_array()? != b"</Comments>" {
                todo!()
            }

            Ok(())
        }

        fn read_chunk_8<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            self.read_author(r)?;

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
            let parameters = r.internal_node_ref::<ChallengeParameters>()?;
            self.map_type = parameters.map_type.clone();
            self.map_style = parameters.map_style.clone();
            self.ty = r.enum_u32()?;

            Ok(())
        }

        fn read_chunk_24<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let is_lap_race = r.bool()?;
            let num_laps = r.u32()?;
            self.num_laps = is_lap_race.then_some(num_laps);

            Ok(())
        }

        fn read_chunk_25<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.texture_mod = r.file_ref_or_null()?;

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
            self.decoration_id = r.id()?;
            let _decoration_collection = r.id_or_null()?;
            let _decoration_author = r.id()?;
            self.size = r.nat3()?;
            let _need_unlock = r.bool()?;
            self.blocks = read_blocks(r)?;

            Ok(())
        }

        fn read_chunk_34<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?; // 1 | 3 | 5

            Ok(())
        }

        fn read_chunk_36<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.music = r.file_ref_or_null()?;

            Ok(())
        }

        fn read_chunk_37<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _coord_origin = r.vec2()?;
            let _coord_target = r.vec2()?;

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
                r.vec3()?;
                r.vec3()?;
                r.vec3()?;
                let _thumbnail_position = r.vec3()?;
                let _thumbnail_fov = r.f32()?;
                let _thumbnail_near_clip_plame = r.f32()?;
                let _thumbnail_far_clip_plane = r.f32()?;
            }

            let _comments = r.string_or_empty()?;

            Ok(())
        }

        fn read_chunk_41<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.password_hash = match r.byte_array::<16>()? {
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] => None,
                hash => Some(hash),
            };

            self.checksum = r.u32()?;

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
            self.thumbnail_position = r.vec3()?;
            self.thumbnail_rotation = r.yaw_pitch_roll()?;
            self.thumbnail_fov = r.f32()?;
            r.f32()?; // 10.0
            r.f32()?; // 0.0
            let _thumbnail_near_clip_plane = r.f32()?; // -1.0
            let _thumbnail_far_clip_plane = r.f32()?; // -1.0
            let _comments = r.string_or_empty()?;

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
                    let _items_on_item = r.list(|r| {
                        r.u32()?;
                        r.u32()?;

                        Ok(())
                    })?;
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

            self.read_author(r)?;

            Ok(())
        }

        fn read_chunk_67<I, N>(
            &mut self,
            r: &mut Reader<impl Read + Seek, I, N>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.encapsulation(|r| {
                self.zones = r.list(|r| r.node())?;

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

            self.baked_blocks = read_blocks(r)?;
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
            let _clip_trigger_size = r.nat3()?;

            Ok(())
        }

        fn read_chunk_75<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _objective_text_author = r.string_or_empty()?;
            let _objective_text_gold = r.string_or_empty()?;
            let _objective_text_silver = r.string_or_empty()?;
            let _objective_text_bronze = r.string_or_empty()?;

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

            let _offzone_trigger_size = r.nat3()?;
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

            self.title_id = r.id()?;
            let game_build = r.string()?;

            let mut entries = game_build.split_ascii_whitespace();
            let entry_1 = entries.next().unwrap();
            let entry_2 = entries.next().unwrap();
            let entry_3 = entries.next().unwrap();

            if entries.next().is_some() {
                panic!()
            }

            let (key, value) = entry_1.split_once('=').unwrap();

            if key != "date" {
                panic!()
            }

            self.game_build_date = value.to_string();

            let (key, value) = entry_2.split_once('=').unwrap();

            match key {
                "git" => {
                    self.game_build_tag = GameBuildTag::Git(value.to_string());
                }
                "Svn" => {
                    self.game_build_tag = GameBuildTag::Svn(value.to_string());
                }
                _ => panic!(),
            }

            let (key, value) = entry_3.split_once('=').unwrap();

            if key != "GameVersion" {
                panic!()
            }

            if value != GAME_VERSION {
                panic!();
            }

            Ok(())
        }

        fn read_chunk_82<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _decoration_base_height_offset = r.u32()?;

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
                    let _collection = r.id_or_null()?;
                    let _author = r.id_or_null()?;

                    Ok(id)
                })?;
                let zip_archive = r.byte_buf()?;
                let _textures = r.list(|r| r.string())?;

                if !zip_archive.is_empty() {
                    self.embedded_items = Some(EmbeddedItems { ids, zip_archive })
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

            self.read_dynamic_lighting(r)?;

            Ok(())
        }

        fn read_chunk_87<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 4 | 5) {
                return Err(Error::chunk_version(version));
            }

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

            let _world_distortion = r.nat3()?;
            r.bool()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_90<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            if r.bool()? {
                // only deco base map.

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

                let lightmap_frames = r.list(|r| {
                    let _webp = r.byte_buf()?;
                    let _webp = r.byte_buf()?;
                    let _webp = r.byte_buf()?;

                    Ok(())
                })?;

                if !lightmap_frames.is_empty() {
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
            let version = r.u32()?;

            if !matches!(version, 0 | 1) {
                return Err(Error::chunk_version(version));
            }

            if r.bool()? {
                let a = r.u32()?;
                let b = r.u32()?;
                let c = r.u32()?;
                let d = r.u32()?;
                let e = r.u32()?;

                match (a, b, c, d, e) {
                    (256, 221, 55, 200, 4292) => {
                        r.bytes(83832)?;
                    }
                    (256, 87, 255, 109, 3384) => {
                        r.bytes(51051)?;
                    }
                    (128, 98, 38, 66, 8) => {
                        r.bytes(256)?;
                        r.u8()?;
                    }
                    (512, 47, 56, 512, 272) => {
                        r.bytes(6196)?;
                    }
                    (256, 43, 158, 42, 321) => {
                        r.bytes(5046)?;
                    }
                    _ => todo!("{a}, {b}, {c}, {d}, {e}"),
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
                    *rotation = r.yaw_pitch_roll()?;
                }
            }

            for baked_block in &mut self.baked_blocks {
                if let BlockType::Free { position, rotation } = &mut baked_block.ty {
                    *position = r.vec3()?;
                    *rotation = r.yaw_pitch_roll()?;
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
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            r.list(|r| r.u32())?;
            r.byte_buf()?;
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
                    item.skin_effect = Some(r.file_ref()?);
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
                let _macroblock_index = r.u32()?;
            }

            for _ in &self.items {
                let _macroblock_index = r.u32()?;
            }

            let _id_flags_pair = r.list(|r| {
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_107<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.read_dynamic_lighting(r)
        }

        fn read_chunk_108<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u8()?;

            Ok(())
        }

        fn read_author<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::version("author", version));
            }

            self.author_id = r.string()?.into();
            self.author_name = r.string()?;
            self.author_zone = r.string()?;
            let _author_extra_info = r.string_or_empty()?;

            Ok(())
        }

        fn read_dynamic_lighting<I, N>(
            &mut self,
            r: &mut Reader<impl Read, I, N>,
        ) -> Result<(), Error> {
            r.u32()?;
            self.time_of_day = r.u32_or_null()?;
            r.u32()?;
            let _dynamic_daylight = r.bool()?;
            let _day_duration = r.u32()?;

            Ok(())
        }
    }

    fn has_lightmap(lightmap_version: u8) -> Result<bool, Error> {
        let has_lightmap = lightmap_version != 0;

        if lightmap_version != 0 && lightmap_version != 8 {
            return Err(Error::version("lightmap", lightmap_version as u32));
        }

        Ok(has_lightmap)
    }

    fn read_blocks(
        r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<Vec<Block>, Error> {
        let version = r.u32()?;

        if version != 6 {
            return Err(Error::version("blocks", version));
        }

        let num_blocks = r.u32()?;
        let mut blocks = Vec::with_capacity(num_blocks as usize);

        while r.peek_u32()? & ID_MARKER_BIT != 0 {
            let block = Block::read_from_body(r)?;

            if block.has_flags {
                blocks.push(block);
            }
        }

        Ok(blocks)
    }

    struct XmlReader<R> {
        inner: quick_xml::Reader<R>,
        buf: Vec<u8>,
    }

    impl<R> XmlReader<R> {
        fn new(inner: R) -> Self {
            Self {
                inner: quick_xml::Reader::from_reader(inner),
                buf: vec![],
            }
        }
    }

    impl<R: BufRead> XmlReader<R> {
        fn read_event(&mut self) -> Result<Event, Error> {
            self.inner
                .read_event_into(&mut self.buf)
                .map_err(|_| Error::new(ErrorKind::Format("".into())))
        }

        fn tag(
            &mut self,
            name: &[u8],
            mut attribute_read_fn: impl FnMut(&mut XmlAttributes) -> Result<(), Error>,
            mut read_fn: impl FnMut(&mut Self) -> Result<(), Error>,
        ) -> Result<(), Error> {
            match self.read_event()? {
                Event::Start(event) if event.name().as_ref() == name => {
                    let mut attributes = XmlAttributes::new(event.attributes());

                    attribute_read_fn(&mut attributes)?;

                    if attributes.inner.next().is_some() {
                        return Err(Error::new(ErrorKind::Format("".into())));
                    }
                }
                _ => return Err(Error::new(ErrorKind::Format("".into()))),
            }

            read_fn(self)?;

            match self.read_event()? {
                Event::End(event) if event.name().as_ref() == name => Ok(()),
                _ => Err(Error::new(ErrorKind::Format("".into()))),
            }
        }

        fn tag_list(
            &mut self,
            name: &[u8],
            elem_name: &[u8],
            mut attribute_read_fn: impl FnMut(&mut XmlAttributes) -> Result<(), Error>,
        ) -> Result<(), Error> {
            match self.read_event()? {
                Event::Start(event) if event.name().as_ref() == name => {
                    let mut attributes = XmlAttributes::new(event.attributes());

                    if attributes.inner.next().is_some() {
                        return Err(Error::new(ErrorKind::Format("".into())));
                    }
                }
                _ => return Err(Error::new(ErrorKind::Format("".into()))),
            }

            loop {
                match self.read_event()? {
                    Event::End(event) if event.name().as_ref() == name => break,
                    Event::Empty(event) if event.name().as_ref() == elem_name => {
                        let mut attributes = XmlAttributes::new(event.attributes());

                        attribute_read_fn(&mut attributes)?;

                        if attributes.inner.next().is_some() {
                            return Err(Error::new(ErrorKind::Format("".into())));
                        }
                    }
                    _ => return Err(Error::new(ErrorKind::Format("".into()))),
                }
            }

            Ok(())
        }

        fn tag_empty(
            &mut self,
            name: &[u8],
            mut attribute_read_fn: impl FnMut(&mut XmlAttributes) -> Result<(), Error>,
        ) -> Result<(), Error> {
            match self.read_event()? {
                Event::Empty(event) if event.name().as_ref() == name => {
                    let mut attributes = XmlAttributes::new(event.attributes());

                    attribute_read_fn(&mut attributes)?;

                    if attributes.inner.next().is_some() {
                        return Err(Error::new(ErrorKind::Format("".into())));
                    }

                    Ok(())
                }
                _ => Err(Error::new(ErrorKind::Format("".into()))),
            }
        }
    }

    struct XmlAttributes<'a> {
        inner: Attributes<'a>,
    }

    impl<'a> XmlAttributes<'a> {
        fn new(inner: Attributes<'a>) -> Self {
            Self { inner }
        }

        fn optional_attribute(&mut self, name: &[u8]) -> Result<Option<Cow<'a, str>>, Error> {
            match self.inner.next() {
                Some(Ok(attribute)) if attribute.key.as_ref() == name => {
                    Ok(Some(attribute.unescape_value().unwrap()))
                }
                None => Ok(None),
                _ => Err(Error::new(ErrorKind::Format("".into()))),
            }
        }

        fn attribute(&mut self, name: &[u8]) -> Result<Cow<'a, str>, Error> {
            match self.optional_attribute(name)? {
                Some(value) => Ok(value),
                None => Err(Error::new(ErrorKind::Format("".into()))),
            }
        }

        fn attribute_from_str<T: FromStr>(&mut self, name: &[u8]) -> Result<T, Error> {
            let value = self.attribute(name)?;

            T::from_str(&value).map_err(|_| Error::new(ErrorKind::Format("".into())))
        }
    }
}

mod write {
    use std::{io::Write, sync::Arc};

    use quick_xml::events::{BytesEnd, BytesStart, Event};

    use crate::{
        game::ctn::{block::BlockType, Block, ChallengeParameters, CollectorList},
        write::{
            writable,
            writer::{IdStateMut, NodeStateMut},
            BodyChunk, BodyChunks, Error, Writable, Writer,
        },
        Nat3, Vec2,
    };

    use self::writable::{write_body_chunks, HeaderChunk, HeaderChunks, WriteBody};

    use super::{Challenge, GameBuildTag, Objective, Validation, GAME_VERSION};

    impl Writable for Challenge {}

    impl writable::Sealed for Challenge {}

    impl HeaderChunks for Challenge {
        fn header_chunks<W: Write, I: IdStateMut, N>(
        ) -> impl Iterator<Item = HeaderChunk<Self, W, I, N>> {
            [
                HeaderChunk::normal(2, |s, w| Self::write_chunk_2(s, w)),
                HeaderChunk::normal(3, |s, w| Self::write_chunk_3(s, w)),
                HeaderChunk::normal(4, |s, w| Self::write_chunk_4(s, w)),
                HeaderChunk::normal(5, |s, w| Self::write_chunk_5(s, w)),
                HeaderChunk::normal(7, |s, w| Self::write_chunk_7(s, w)),
                HeaderChunk::normal(8, |s, w| Self::write_chunk_8(s, w)),
            ]
            .into_iter()
        }
    }

    impl WriteBody for Challenge {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for Challenge {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [
                BodyChunk::normal(13, Self::write_chunk_13),
                BodyChunk::normal(17, Self::write_chunk_17),
                BodyChunk::skippable(24, |s, w| Self::write_chunk_24(s, w)),
                BodyChunk::skippable(25, |s, w| Self::write_chunk_25(s, w)),
                BodyChunk::normal(31, Self::write_chunk_31),
                BodyChunk::normal(34, Self::write_chunk_34),
                BodyChunk::normal(36, Self::write_chunk_36),
                BodyChunk::normal(37, Self::write_chunk_37),
                BodyChunk::normal(38, Self::write_chunk_38),
                BodyChunk::normal(40, Self::write_chunk_40),
                BodyChunk::skippable(41, |s, w| Self::write_chunk_41(s, w)),
                BodyChunk::normal(42, Self::write_chunk_42),
                BodyChunk::skippable(52, |s, w| Self::write_chunk_52(s, w)),
                BodyChunk::skippable(54, |s, w| Self::write_chunk_54(s, w)),
                BodyChunk::skippable(56, |s, w| Self::write_chunk_56(s, w)),
                BodyChunk::skippable(62, |s, w| Self::write_chunk_62(s, w)),
                BodyChunk::skippable(64, |s, w| Self::write_chunk_64(s, w)),
                BodyChunk::skippable(66, |s, w| Self::write_chunk_66(s, w)),
                BodyChunk::skippable(67, |s, w| Self::write_chunk_67(s, w)),
                BodyChunk::skippable(68, |s, w| Self::write_chunk_68(s, w)),
                BodyChunk::skippable(72, |s, w| Self::write_chunk_72(s, w)),
                BodyChunk::normal(73, |s, w| Self::write_chunk_73(s, w)),
                BodyChunk::skippable(75, |s, w| Self::write_chunk_75(s, w)),
                BodyChunk::skippable(79, |s, w| Self::write_chunk_79(s, w)),
                BodyChunk::skippable(80, |s, w| Self::write_chunk_80(s, w)),
                BodyChunk::skippable(81, |s, w| Self::write_chunk_81(s, w)),
                BodyChunk::skippable(82, |s, w| Self::write_chunk_82(s, w)),
                BodyChunk::skippable(83, |s, w| Self::write_chunk_83(s, w)),
                BodyChunk::skippable(84, |s, w| Self::write_chunk_84(s, w)),
                BodyChunk::skippable(85, |s, w| Self::write_chunk_85(s, w)),
                BodyChunk::skippable(86, |s, w| Self::write_chunk_86(s, w)),
                BodyChunk::skippable(87, |s, w| Self::write_chunk_87(s, w)),
                BodyChunk::skippable(88, |s, w| Self::write_chunk_88(s, w)),
                BodyChunk::skippable(89, |s, w| Self::write_chunk_89(s, w)),
                BodyChunk::skippable(90, |s, w| Self::write_chunk_90(s, w)),
                BodyChunk::skippable(91, |s, w| Self::write_chunk_91(s, w)),
                BodyChunk::skippable(92, |s, w| Self::write_chunk_92(s, w)),
                BodyChunk::skippable(93, |s, w| Self::write_chunk_93(s, w)),
                BodyChunk::skippable(94, |s, w| Self::write_chunk_94(s, w)),
                BodyChunk::skippable(95, |s, w| Self::write_chunk_95(s, w)),
                BodyChunk::skippable(96, |s, w| Self::write_chunk_96(s, w)),
                BodyChunk::skippable(97, |s, w| Self::write_chunk_97(s, w)),
                BodyChunk::skippable(98, |s, w| Self::write_chunk_98(s, w)),
                BodyChunk::skippable(99, |s, w| Self::write_chunk_99(s, w)),
                BodyChunk::skippable(100, |s, w| Self::write_chunk_100(s, w)),
                BodyChunk::skippable(101, |s, w| Self::write_chunk_101(s, w)),
                BodyChunk::skippable(103, |s, w| Self::write_chunk_103(s, w)),
                BodyChunk::skippable(104, |s, w| Self::write_chunk_104(s, w)),
                BodyChunk::skippable(105, |s, w| Self::write_chunk_105(s, w)),
                BodyChunk::skippable(107, |s, w| Self::write_chunk_107(s, w)),
                BodyChunk::skippable(108, |s, w| Self::write_chunk_108(s, w)),
            ]
            .into_iter()
        }
    }

    impl Challenge {
        fn write_chunk_2<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u8(13)?;
            w.bool(false)?;

            if let Some(Validation {
                objective: Objective::MedalTimes(ref medal_times),
                ..
            }) = self.validation
            {
                w.u32(medal_times.bronze_time)?;
                w.u32(medal_times.silver_time)?;
                w.u32(medal_times.gold_time)?;
                w.u32(medal_times.author_time)?;
            } else {
                w.u32(0xffffffff)?;
                w.u32(0xffffffff)?;
                w.u32(0xffffffff)?;
                w.u32(0xffffffff)?;
            }

            w.u32(self.cost)?;
            w.bool(self.num_laps.is_some())?;
            w.u32(self.play_mode)?;
            w.u32(0)?;

            if let Some(Validation {
                objective: Objective::AuthorScore(author_score),
                ..
            }) = self.validation
            {
                w.u32(author_score)?;
            } else {
                w.u32(0)?;
            }

            w.u32(self.editor_mode as u32)?;
            w.u32(0)?;
            w.u32(self.num_checkpoints)?;

            if let Some(num_laps) = self.num_laps {
                w.u32(num_laps)?;
            } else {
                w.u32(3)?;
            }

            Ok(())
        }

        fn write_chunk_3<N>(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            w.u8(11)?;
            w.id(&self.id)?;
            w.u32(0x1a)?;
            w.id(&self.author_id)?;
            w.string(&self.name)?;
            w.u8(self.ty as u8)?;
            w.u32(0)?;
            w.string(&self.password)?;
            w.id(&self.decoration_id)?;
            w.u32(0x1a)?;
            w.id(&Arc::from("Nadeo"))?;
            w.vec2(Vec2::new(0.0, 0.0))?;
            w.vec2(Vec2::new(0.0, 0.0))?;
            w.bytes(&self.pack_mask)?;
            w.string(&self.map_type)?;
            w.string_or_empty(self.map_style.as_ref())?;
            w.u64(self.lightmap_cache_id)?;
            w.u8(self.lightmap_version())?;
            w.id(&self.title_id)?;

            Ok(())
        }

        fn write_chunk_4<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(6)?;

            Ok(())
        }

        fn write_chunk_5<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let mut w = XmlWriter::new(w.get_mut());

            w.tag(
                "header",
                |w| {
                    w.attribute("type", "map");
                    w.attribute("exever", GAME_VERSION);
                    w.attribute("exebuild", &self.game_build_date);
                    w.attribute("title", &self.title_id);
                    w.attribute("lightmap", &self.lightmap_version().to_string());
                },
                |w| {
                    w.tag_empty("ident", |w| {})?;
                    w.tag_empty("desc", |w| {})?;
                    w.tag_empty("playermodel", |w| {})?;
                    w.tag_empty("times", |w| {})?;
                    w.tag("deps", |_| {}, |_| Ok(()))?;

                    Ok(())
                },
            )?;

            Ok(())
        }

        fn write_chunk_7<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            w.u32(self.thumbnail.len() as u32)?;
            w.bytes(b"<Thumbnail.jpg>")?;
            w.bytes(&self.thumbnail)?;
            w.bytes(b"</Thumbnail.jpg>")?;
            w.bytes(b"<Comments>")?;
            w.u32(0)?;
            w.bytes(b"</Comments>")?;

            Ok(())
        }

        fn write_chunk_8<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            w.u32(0)?;
            w.string(&self.author_id)?;
            w.string(&self.author_name)?;
            w.string(&self.author_zone)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_13<N>(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            w.id_or_null(None)?;
            w.id_or_null(None)?;
            w.id_or_null(None)?;

            Ok(())
        }

        fn write_chunk_17(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            w.internal_node_ref(&Arc::new(CollectorList::default()))?;
            w.internal_node_ref(&Arc::new(ChallengeParameters::default()))?;
            w.u32(self.ty as u32)?;

            Ok(())
        }

        fn write_chunk_24<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.bool(self.num_laps.is_some())?;
            w.u32(self.num_laps.unwrap_or(3))?;

            Ok(())
        }

        fn write_chunk_25<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.file_ref_or_null(self.texture_mod.as_ref())?;

            Ok(())
        }

        fn write_chunk_31(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            w.id(&self.id)?;
            w.u32(0x1a)?;
            w.id(&self.author_id)?;
            w.string(&self.name)?;
            w.id(&self.decoration_id)?;
            w.u32(0x1a)?;
            w.id(&Arc::from("Nadeo"))?;
            w.nat3(self.size)?;
            w.bool(false)?;
            write_blocks(w, &self.blocks)?;

            Ok(())
        }

        fn write_chunk_34<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(5)?;

            Ok(())
        }

        fn write_chunk_36<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.file_ref_or_null(self.music.as_ref())?;

            Ok(())
        }

        fn write_chunk_37<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.vec2(Vec2::new(0.0, 0.0))?;
            w.vec2(Vec2::new(0.0, 0.0))?;

            Ok(())
        }

        fn write_chunk_38<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0xffffffff)?;

            Ok(())
        }

        fn write_chunk_40<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.bool(false)?;
            w.string_or_empty(None)?;

            Ok(())
        }

        fn write_chunk_41<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            match self.password_hash {
                Some(hash) => w.bytes(&hash)?,
                None => w.bytes(&[0; 16])?,
            }

            w.u32(self.checksum)?;

            Ok(())
        }

        fn write_chunk_42<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.bool(false)?;

            Ok(())
        }

        fn write_chunk_52<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_54<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.vec3(self.thumbnail_position)?;
            w.yaw_pitch_roll(self.thumbnail_rotation)?;
            w.f32(self.thumbnail_fov)?;
            w.f32(10.0)?;
            w.f32(0.0)?;
            w.f32(-1.0)?;
            w.f32(-1.0)?;
            w.string_or_empty(None)?;

            Ok(())
        }

        fn write_chunk_56<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_62<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.u32(10)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_64<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(8)?;
            w.u32(0)?;
            w.encapsulation(|w| {
                w.list_with_version(&self.items, |w, item| w.node(item))?;
                w.u32(0)?;
                w.u32(0)?;
                w.u32(0)?;
                w.u32(0)?;

                Ok(())
            })?;

            Ok(())
        }

        fn write_chunk_66<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            self.write_author(w)?;

            Ok(())
        }

        fn write_chunk_67<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.encapsulation(|w| {
                w.list(&self.zones, |w, zone| w.node(zone))?;

                Ok(())
            })?;

            Ok(())
        }

        fn write_chunk_68<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.encapsulation(|w| {
                self.script_metadata.write_body(w)?;

                Ok(())
            })?;

            Ok(())
        }

        fn write_chunk_72(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            w.u32(0)?;
            write_blocks(w, &self.baked_blocks)?;
            w.u32(0)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_73(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            w.u32(2)?;
            w.internal_node_ref_or_null(self.intro_clip.as_ref())?;
            w.internal_node_ref_or_null(self.podium_clip.as_ref())?;
            w.internal_node_ref_or_null(self.in_game_clips.as_ref())?;
            w.internal_node_ref_or_null(self.end_race_clips.as_ref())?;
            w.internal_node_ref_or_null(self.ambiance_clip.as_ref())?;
            w.nat3(Nat3::new(3, 1, 3))?;

            Ok(())
        }

        fn write_chunk_75<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.string_or_empty(None)?;
            w.string_or_empty(None)?;
            w.string_or_empty(None)?;
            w.string_or_empty(None)?;

            Ok(())
        }

        fn write_chunk_79<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(3)?;
            w.u8(0)?;

            Ok(())
        }

        fn write_chunk_80<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.nat3(Nat3::new(3, 1, 3))?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_81<N>(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let (key, tag) = match self.game_build_tag {
                GameBuildTag::Git(ref tag) => ("git", tag),
                GameBuildTag::Svn(ref tag) => ("Svn", tag),
            };

            let game_build = format!(
                "date={} {}={} GameVersion=3.3.0",
                self.game_build_date, key, tag
            );

            w.u32(0)?;
            w.id(&self.title_id)?;
            w.string(&game_build)?;

            Ok(())
        }

        fn write_chunk_82<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.u32(8)?;

            Ok(())
        }

        fn write_chunk_83<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(3)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_84<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            w.u32(0)?;
            w.encapsulation(|w| {
                if let Some(ref embedded_items) = self.embedded_items {
                    w.list(&embedded_items.ids, |w, id| {
                        w.id(id)?;
                        w.id_or_null(None)?;
                        w.id_or_null(None)?;

                        Ok(())
                    })?;
                    w.byte_buf(&embedded_items.zip_archive)?;
                } else {
                    w.u32(0)?;
                    w.u32(0)?;
                }

                w.u32(0)?;

                Ok(())
            })?;

            Ok(())
        }

        fn write_chunk_85<W, I, N>(&self, _: &mut Writer<W, I, N>) -> Result<(), Error> {
            Ok(())
        }

        fn write_chunk_86<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(3)?;
            w.u32(0)?;
            w.u32(self.time_of_day.unwrap_or(0xffffffff))?;
            w.u32(0)?;
            w.bool(false)?;
            w.u32(300000)?;

            Ok(())
        }

        fn write_chunk_87<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(5)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_88<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_89<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(3)?;
            w.nat3(Nat3::new(0, 0, 0))?;
            w.bool(false)?;
            w.u32(0x41a00000)?;
            w.u32(0x40400000)?;

            Ok(())
        }

        fn write_chunk_90<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.bool(false)?;

            Ok(())
        }

        fn write_chunk_91<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.bool(false)?;
            w.bool(false)?;
            w.bool(false)?;

            Ok(())
        }

        fn write_chunk_92<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.bool(true)?;

            Ok(())
        }

        fn write_chunk_93<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            w.bool(false)?;

            Ok(())
        }

        fn write_chunk_94<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            w.u32(0)?;
            w.u32(8)?;
            w.u32(0)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_95<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;

            for block in &self.blocks {
                if let BlockType::Free { position, rotation } = block.ty {
                    w.vec3(position)?;
                    w.yaw_pitch_roll(rotation)?;
                }
            }

            for baked_block in &self.baked_blocks {
                if let BlockType::Free { position, rotation } = baked_block.ty {
                    w.vec3(position)?;
                    w.yaw_pitch_roll(rotation)?;
                }
            }

            Ok(())
        }

        fn write_chunk_96<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_97<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            w.u32(0)?;
            w.u32(0)?;
            w.u32(0)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_98<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;

            for block in &self.blocks {
                w.u8(block.elem_color as u8)?;
            }

            for baked_block in &self.baked_blocks {
                w.u8(baked_block.elem_color as u8)?;
            }

            for item in &self.items {
                w.u8(item.elem_color as u8)?;
            }

            Ok(())
        }

        fn write_chunk_99<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;

            for item in &self.items {
                w.u8(item.anim_offset as u8)?;
            }

            Ok(())
        }

        fn write_chunk_100<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.u32(0)?;
            w.u32(4)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_101<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;

            for item in &self.items {
                if let Some(ref skin_effect) = item.skin_effect {
                    w.bool8(true)?;
                    w.file_ref(skin_effect)?;
                } else {
                    w.bool8(false)?;
                }
            }

            Ok(())
        }

        fn write_chunk_103<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.u32(0)?;
            w.u32(4)?;
            w.u32(0xffffffff)?;

            Ok(())
        }

        fn write_chunk_104<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;

            for block in &self.blocks {
                w.u8(block.lightmap_quality as u8)?;
            }

            for baked_block in &self.baked_blocks {
                w.u8(baked_block.lightmap_quality as u8)?;
            }

            for item in &self.items {
                w.u8(item.lightmap_quality as u8)?;
            }

            Ok(())
        }

        fn write_chunk_105<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;

            for _ in &self.blocks {
                w.u32(0xffffffff)?;
            }

            for _ in &self.items {
                w.u32(0xffffffff)?;
            }

            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_107<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.u32(self.time_of_day.unwrap_or(0xffffffff))?;
            w.u32(0)?;
            w.bool(false)?;
            w.u32(300000)?;

            Ok(())
        }

        fn write_chunk_108<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.u8(0)?;

            Ok(())
        }

        fn lightmap_version(&self) -> u8 {
            if self.has_lightmap {
                8
            } else {
                0
            }
        }

        fn write_author<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.string(&self.author_id)?;
            w.string(&self.author_name)?;
            w.string(&self.author_zone)?;
            w.string_or_empty(None)?;

            Ok(())
        }
    }

    fn write_blocks(
        w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        blocks: &[Block],
    ) -> Result<(), Error> {
        w.u32(6)?;
        w.list(blocks, |w, block| block.write_body(w))?;

        Ok(())
    }

    struct XmlWriter<W> {
        inner: quick_xml::Writer<W>,
    }

    impl<W> XmlWriter<W> {
        fn new(inner: W) -> Self {
            Self {
                inner: quick_xml::Writer::new(inner),
            }
        }
    }

    impl<W: Write> XmlWriter<W> {
        fn write_event(&mut self, event: Event) -> Result<(), Error> {
            self.inner.write_event(event).map_err(Error::io)
        }

        fn tag(
            &mut self,
            name: &str,
            mut attribute_fn: impl FnMut(&mut Attributes),
            mut write_fn: impl FnMut(&mut Self) -> Result<(), Error>,
        ) -> Result<(), Error> {
            let mut bytes_start = BytesStart::new(name);

            attribute_fn(&mut Attributes {
                bytes_start: &mut bytes_start,
            });

            self.write_event(Event::Start(bytes_start))?;

            write_fn(self)?;

            self.write_event(Event::End(BytesEnd::new(name)))?;

            Ok(())
        }

        fn tag_empty(
            &mut self,
            name: &str,
            mut attribute_fn: impl FnMut(&mut Attributes),
        ) -> Result<(), Error> {
            let mut bytes_start = BytesStart::new(name);

            attribute_fn(&mut Attributes {
                bytes_start: &mut bytes_start,
            });

            self.write_event(Event::Empty(bytes_start))?;

            Ok(())
        }
    }

    struct Attributes<'a, 'b> {
        bytes_start: &'a mut BytesStart<'b>,
    }

    impl Attributes<'_, '_> {
        fn attribute(&mut self, name: &str, value: &str) {
            self.bytes_start.push_attribute((name, value));
        }
    }
}
