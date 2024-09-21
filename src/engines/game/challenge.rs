use std::rc::Rc;

use crate::{
    engines::{game::block::Block, scene::VehicleCarMarksSamples, script::TraitsMetadata},
    read::{readable, Readable},
    write::{writable, Writable},
    Box3, Ident, PackDesc, Vec2, Vec3,
};

use super::{
    AnchoredObject, ChallengeParameters, CollectorList, MediaClip, MediaClipGroup, ZoneGenealogy,
};

#[derive(Default)]
enum PlayMode {
    #[default]
    Race,
    Platform,
    Puzzle,
    Crazy,
    Shortcut,
    Stunts,
    Script,
}

#[derive(Default)]
enum EditorMode {
    #[default]
    Advanced,
    Simple,
    HasGhostBlocks,
    Gamepad,
}

#[derive(Default)]
enum MapKind {
    #[default]
    EndMarker,
    Campaign,
    Puzzle,
    Retro,
    TimeAttack,
    Rounds,
    InProgress,
    Multi,
    Solo,
    Site,
    SoloNadeo,
    MultiNadeo,
}

enum MapElemColor {
    Default,
    White,
    Green,
    Blue,
    Red,
    Black,
}

enum PhaseOffset {
    None,
    One8th,
    Two8th,
    Three8th,
    Four8th,
    Five8th,
    Six8th,
    Seven8th,
}

enum LightmapQuality {
    Normal,
    High,
    VeryHigh,
    Highest,
    Low,
    VeryLow,
    Lowest,
}

/// A challenge.
#[derive(Default)]
pub struct Challenge {
    bronze_time: u32,
    silver_time: u32,
    gold_time: u32,
    author_time: u32,
    cost: u32,
    is_lap_race: bool,
    mode: PlayMode,
    author_score: u32,
    editor: EditorMode,
    num_checkpoints: u32,
    num_laps: u32,
    map_info: Ident,
    map_name: String,
    map_kind: MapKind,
    password: String,
    decoration: Ident,
    map_coord_origin: Vec2<f32>,
    map_coord_target: Vec2<f32>,
    pack_mask: u128,
    map_type: String,
    map_style: String,
    lightmap_cache_uid: u64,
    lightmap_version: u8,
    title_id: Option<Rc<str>>,
    xml: String,
    thumbnail: Box<[u8]>,
    comments: String,
    author_login: String,
    author_nickname: String,
    author_zone: String,
    author_extra_info: String,
    player_model: Ident,
    block_stock: Option<Rc<CollectorList>>,
    challenge_parameters: Option<Rc<ChallengeParameters>>,
    mod_pack_desc: Option<PackDesc>,
    size: Vec3<u32>,
    need_unlock: bool,
    custom_music_pack_desc: Option<PackDesc>,
    blocks: Box<[Block]>,
    hashed_password: [u8; 16],
    crc32: u32,
    created_with_simple_editor: bool,
    thumbnail_position: Vec3<f32>,
    thumbnail_pitch_yaw_roll: Vec3<f32>,
    thumbnail_fov: f32,
    thumbnail_near_clip_plane: f32,
    thumbnail_far_clip_plane: f32,
    car_marks_buffer: Box<[Option<Rc<VehicleCarMarksSamples>>]>,
    anchored_objects: Box<[Option<AnchoredObject>]>,
    items_on_item: Box<[Vec2<u32>]>,
    block_indices: Box<[u32]>,
    item_indices: Box<[u32]>,
    snap_item_groups: Box<[u32]>,
    snapped_indices: Box<[u32]>,
    zone_genealogy: Box<[Option<ZoneGenealogy>]>,
    script_metadata: Option<TraitsMetadata>,
    baked_blocks: Box<[Block]>,
    clip_intro: Option<Rc<MediaClip>>,
    clip_podium: Option<Rc<MediaClip>>,
    clip_group_in_game: Option<Rc<MediaClipGroup>>,
    clip_group_end_race: Option<Rc<MediaClipGroup>>,
    clip_ambiance: Option<Rc<MediaClip>>,
    clip_trigger_size: Vec3<u32>,
    objective_text_author: String,
    objective_text_gold: String,
    objective_text_silver: String,
    objective_text_bronze: String,
    offzone_trigger_size: Vec3<u32>,
    offzones: Box<[Box3<u32>]>,
    build_version: String,
    deco_base_height_offset: u32,
    embedded_item_models: Box<[Ident]>,
    embedded_zip_data: Box<[u8]>,
    textures: Box<[String]>,
    day_time: f32,
    dynamic_daylight: bool,
    day_duration: u32,
    world_distortion: Vec3<f32>,
}

impl Challenge {
    /// TODO.
    pub const fn gold_time(&self) -> u32 {
        self.gold_time
    }

    /// TODO.
    pub const fn cost(&self) -> u32 {
        self.cost
    }
}

impl Readable for Challenge {}

impl readable::Sealed for Challenge {}

impl Writable for Challenge {}

impl writable::Sealed for Challenge {
    const CLASS_ID: u32 = 0x03043000;
}

mod read {
    use std::io::Read;

    use crate::{
        engines::{
            game::{
                AnchoredObject, Block, ChallengeParameters, CollectorList, MediaClip,
                MediaClipGroup, ZoneGenealogy,
            },
            game_data::WaypointSpecialProperty,
            scene::VehicleCarMarksSamples,
            script::TraitsMetadata,
        },
        read::{
            readable::{BodyChunk, BodyChunks, UserDataChunk, UserDataChunks},
            IdState, IdStateMut, NodeStateMut, Reader,
        },
        Error,
    };

    use super::{
        Challenge, EditorMode, LightmapQuality, MapElemColor, MapKind, PhaseOffset, PlayMode,
    };

    impl PlayMode {
        fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
            let play_mode = match r.u32()? {
                0 => Self::Race,
                1 => Self::Platform,
                2 => Self::Puzzle,
                3 => Self::Crazy,
                4 => Self::Shortcut,
                5 => Self::Stunts,
                6 => Self::Script,
                _ => return Err(Error),
            };

            Ok(play_mode)
        }
    }

    impl EditorMode {
        fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
            let editor_mode = match r.u32()? {
                0 => Self::Advanced,
                1 => Self::Simple,
                2 => Self::HasGhostBlocks,
                4 => Self::Gamepad,
                _ => return Err(Error),
            };

            Ok(editor_mode)
        }
    }

    impl MapKind {
        fn read_u8<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
            let map_kind = match r.u8()? {
                0 => Self::EndMarker,
                1 | 7 => Self::Campaign,
                2 => Self::Puzzle,
                3 => Self::Retro,
                4 => Self::TimeAttack,
                5 => Self::Rounds,
                6 => Self::InProgress,
                8 => Self::Multi,
                9 => Self::Solo,
                10 => Self::Site,
                11 => Self::SoloNadeo,
                12 => Self::MultiNadeo,
                _ => return Err(Error),
            };

            Ok(map_kind)
        }

        fn read_u32<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
            let map_kind = match r.u32()? {
                0 => Self::EndMarker,
                1 | 7 => Self::Campaign,
                2 => Self::Puzzle,
                3 => Self::Retro,
                4 => Self::TimeAttack,
                5 => Self::Rounds,
                6 => Self::InProgress,
                8 => Self::Multi,
                9 => Self::Solo,
                10 => Self::Site,
                11 => Self::SoloNadeo,
                12 => Self::MultiNadeo,
                _ => return Err(Error),
            };

            Ok(map_kind)
        }
    }

    impl MapElemColor {
        fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
            let map_elem_color = match r.u8()? {
                0 => Self::Default,
                1 => Self::White,
                2 => Self::Green,
                3 => Self::Blue,
                4 => Self::Red,
                5 => Self::Black,
                _ => return Err(Error),
            };

            Ok(map_elem_color)
        }
    }

    impl PhaseOffset {
        fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
            let map_elem_color = match r.u8()? {
                0 => Self::None,
                1 => Self::One8th,
                2 => Self::Two8th,
                3 => Self::Three8th,
                4 => Self::Four8th,
                5 => Self::Five8th,
                6 => Self::Six8th,
                7 => Self::Seven8th,
                _ => return Err(Error),
            };

            Ok(map_elem_color)
        }
    }

    impl LightmapQuality {
        fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
            let lightmap_quality = match r.u8()? {
                0 => Self::Normal,
                1 => Self::High,
                2 => Self::VeryHigh,
                3 => Self::Highest,
                4 => Self::Low,
                5 => Self::VeryLow,
                6 => Self::Lowest,
                _ => return Err(Error),
            };

            Ok(lightmap_quality)
        }
    }

    impl UserDataChunks for Challenge {
        fn user_data_chunks() -> impl Iterator<Item = UserDataChunk<Self>> {
            let chunks: [UserDataChunk<Self>; 6] = [
                (2, |n, r| Self::read_chunk_2(n, r)),
                (3, |n, r| Self::read_chunk_3(n, r)),
                (4, |n, r| Self::read_chunk_4(n, r)),
                (5, |n, r| Self::read_chunk_5(n, r)),
                (7, |n, r| Self::read_chunk_7(n, r)),
                (8, |n, r| Self::read_chunk_8(n, r)),
            ];

            chunks.into_iter()
        }
    }

    impl BodyChunks for Challenge {
        type Parent = Self;

        fn parent(&mut self) -> Option<&mut Self> {
            None
        }

        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            let chunks: [BodyChunk<Self, R, I, N>; 45] = [
                (13, |n, r| Self::read_chunk_13(n, r), false),
                (17, |n, r| Self::read_chunk_17(n, r), false),
                (24, |n, r| Self::read_chunk_24(n, r), true),
                (25, |n, r| Self::read_chunk_25(n, r), true),
                (31, |n, r| Self::read_chunk_31(n, r), false),
                (34, |n, r| Self::read_chunk_34(n, r), false),
                (36, |n, r| Self::read_chunk_36(n, r), false),
                (37, |n, r| Self::read_chunk_37(n, r), false),
                (41, |n, r| Self::read_chunk_41(n, r), true),
                (42, |n, r| Self::read_chunk_42(n, r), false),
                (52, |n, r| Self::read_chunk_52(n, r), true),
                (54, |n, r| Self::read_chunk_54(n, r), true),
                (62, |n, r| Self::read_chunk_62(n, r), true),
                (64, |n, r| Self::read_chunk_64(n, r), true),
                (66, |n, r| Self::read_chunk_66(n, r), true),
                (67, |n, r| Self::read_chunk_67(n, r), true),
                (68, |n, r| Self::read_chunk_68(n, r), true),
                (72, |n, r| Self::read_chunk_72(n, r), true),
                (73, |n, r| Self::read_chunk_73(n, r), false),
                (75, |n, r| Self::read_chunk_75(n, r), true),
                (79, |n, r| Self::read_chunk_79(n, r), true),
                (80, |n, r| Self::read_chunk_80(n, r), true),
                (81, |n, r| Self::read_chunk_81(n, r), true),
                (82, |n, r| Self::read_chunk_82(n, r), true),
                (83, |n, r| Self::read_chunk_83(n, r), true),
                (84, |n, r| Self::read_chunk_84(n, r), true),
                (85, |n, r| Self::read_chunk_85(n, r), true),
                (86, |n, r| Self::read_chunk_86(n, r), true),
                (87, |n, r| Self::read_chunk_87(n, r), true),
                (89, |n, r| Self::read_chunk_89(n, r), true),
                (90, |n, r| Self::read_chunk_90(n, r), true),
                (91, |n, r| Self::read_chunk_91(n, r), true),
                (93, |n, r| Self::read_chunk_93(n, r), true),
                (94, |n, r| Self::read_chunk_94(n, r), true),
                (95, |n, r| Self::read_chunk_95(n, r), true),
                (96, |n, r| Self::read_chunk_96(n, r), true),
                (97, |n, r| Self::read_chunk_97(n, r), true),
                (98, |n, r| Self::read_chunk_98(n, r), true),
                (99, |n, r| Self::read_chunk_99(n, r), true),
                (100, |n, r| Self::read_chunk_100(n, r), true),
                (101, |n, r| Self::read_chunk_101(n, r), true),
                (103, |n, r| Self::read_chunk_103(n, r), true),
                (104, |n, r| Self::read_chunk_104(n, r), true),
                (105, |n, r| Self::read_chunk_105(n, r), true),
                (107, |n, r| Self::read_chunk_107(n, r), true),
            ];

            chunks.into_iter()
        }
    }

    impl Challenge {
        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 13 {
                return Err(Error);
            }

            r.bool()?;
            self.bronze_time = r.u32()?;
            self.silver_time = r.u32()?;
            self.gold_time = r.u32()?;
            self.author_time = r.u32()?;
            self.cost = r.u32()?;
            self.is_lap_race = r.bool()?;
            self.mode = PlayMode::read(r)?;
            r.u32()?;
            self.author_score = r.u32()?;
            self.editor = EditorMode::read(r)?;
            r.u32()?;
            self.num_checkpoints = r.u32()?;
            self.num_laps = r.u32()?;

            Ok(())
        }

        fn read_chunk_3<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 11 {
                return Err(Error);
            }

            self.map_info = r.ident()?;
            self.map_name = r.string()?;
            self.map_kind = MapKind::read_u8(r)?;
            r.u32()?;
            self.password = r.string()?;
            self.decoration = r.ident()?;
            self.read_map_origin_and_target(r)?;
            self.pack_mask = r.u128()?;
            self.map_type = r.string()?;
            self.map_style = r.string()?;
            self.lightmap_cache_uid = r.u64()?;
            self.lightmap_version = r.u8()?;
            self.title_id = r.id()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(Error);
            }

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.xml = r.string()?;

            Ok(())
        }

        fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error);
            }

            let thumbnail_len = r.u32()?;

            if &r.byte_array()? != b"<Thumbnail.jpg>" {
                return Err(Error);
            }

            self.thumbnail = r.bytes(thumbnail_len as usize)?;

            if &r.byte_array()? != b"</Thumbnail.jpg>" {
                return Err(Error);
            }

            if &r.byte_array()? != b"<Comments>" {
                return Err(Error);
            }

            self.comments = r.string()?;

            if &r.byte_array()? != b"</Comments>" {
                return Err(Error);
            }

            Ok(())
        }

        fn read_chunk_8<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error);
            }

            self.read_author(r)?;

            Ok(())
        }

        fn read_chunk_13<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            self.player_model = r.ident()?;

            Ok(())
        }

        fn read_chunk_17(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.block_stock = r.node::<CollectorList>()?;
            self.challenge_parameters = r.node::<ChallengeParameters>()?;
            self.map_kind = MapKind::read_u32(r)?;

            Ok(())
        }

        fn read_chunk_24<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.is_lap_race = r.bool()?;
            self.num_laps = r.u32()?;

            Ok(())
        }

        fn read_chunk_25<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.mod_pack_desc = r.pack_desc()?;

            Ok(())
        }

        fn read_chunk_31(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.map_info = r.ident()?;
            self.map_name = r.string()?;
            self.decoration = r.ident()?;
            self.size = r.vec3::<u32>()?;
            self.need_unlock = r.bool()?;
            self.blocks = read_blocks(r)?;

            Ok(())
        }

        fn read_chunk_34<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_36<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.custom_music_pack_desc = r.pack_desc()?;

            Ok(())
        }

        fn read_chunk_37<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.read_map_origin_and_target(r)?;

            Ok(())
        }

        fn read_chunk_41<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.hashed_password = r.byte_array::<16>()?;
            self.crc32 = r.u32()?;

            Ok(())
        }

        fn read_chunk_42<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.created_with_simple_editor = r.bool()?;

            Ok(())
        }

        fn read_chunk_52<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_54<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.thumbnail_position = r.vec3::<f32>()?;
            self.thumbnail_pitch_yaw_roll = r.vec3::<f32>()?;
            self.thumbnail_fov = r.f32()?;
            r.f32()?;
            r.f32()?;
            self.thumbnail_near_clip_plane = r.f32()?;
            self.thumbnail_far_clip_plane = r.f32()?;
            self.comments = r.string()?;

            Ok(())
        }

        fn read_chunk_62(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            self.car_marks_buffer = r.versioned_list(|r| r.node::<VehicleCarMarksSamples>())?;

            Ok(())
        }

        fn read_chunk_64<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 7 {
                return Err(Error);
            }

            r.u32()?;
            let len = r.u32()?;

            {
                let mut r = r.take_with(len as u64, IdState::new(), ());

                self.anchored_objects = r.versioned_list(|r| r.node_inline::<AnchoredObject>())?;
                self.items_on_item = r.list(|r| r.vec2::<u32>())?;
                self.block_indices = r.list(|r| r.u32())?;
                self.item_indices = r.list(|r| r.u32())?;
                self.snap_item_groups = r.list(|r| r.u32())?;
                r.list(|r| r.u32())?;
                self.snapped_indices = r.list(|r| r.u32())?;
            }

            Ok(())
        }

        fn read_chunk_66<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error);
            }

            self.read_author(r)?;

            Ok(())
        }

        fn read_chunk_67<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            let len = r.u32()?;

            {
                let mut r = r.take_with(len as u64, IdState::new(), ());

                self.zone_genealogy = r.list(|r| r.node_inline::<ZoneGenealogy>())?;
            }

            Ok(())
        }

        fn read_chunk_68<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            let len = r.u32()?;

            {
                let mut r = r.take_with(len as u64, IdState::new(), ());

                self.script_metadata = r.node_inline_v2::<TraitsMetadata>()?;
            }

            Ok(())
        }

        fn read_chunk_72(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            self.baked_blocks = read_blocks(r)?;
            r.u32()?;
            let _baked_clips_additional_data = r.list(|_| Ok(()))?;

            Ok(())
        }

        fn read_chunk_73(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            if r.u32()? != 2 {
                return Err(Error);
            }

            self.clip_intro = r.node::<MediaClip>()?;
            self.clip_podium = r.node::<MediaClip>()?;
            self.clip_group_in_game = r.node::<MediaClipGroup>()?;
            self.clip_group_end_race = r.node::<MediaClipGroup>()?;
            self.clip_ambiance = r.node::<MediaClip>()?;
            self.clip_trigger_size = r.vec3::<u32>()?;

            Ok(())
        }

        fn read_chunk_75<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.objective_text_author = r.string()?;
            self.objective_text_gold = r.string()?;
            self.objective_text_silver = r.string()?;
            self.objective_text_bronze = r.string()?;

            Ok(())
        }

        fn read_chunk_79<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error);
            }

            r.u8()?;

            Ok(())
        }

        fn read_chunk_80<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            self.offzone_trigger_size = r.vec3::<u32>()?;
            self.offzones = r.list(|r| r.box3::<u32>())?;

            Ok(())
        }

        fn read_chunk_81<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            self.title_id = r.id()?;
            self.build_version = r.string()?;

            Ok(())
        }

        fn read_chunk_82<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            self.deco_base_height_offset = r.u32()?;

            Ok(())
        }

        fn read_chunk_83(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error);
            }

            let _bot_paths = r.list(|r| {
                let _clan = r.u32()?;
                let _path = r.list(|r| r.vec3::<f32>())?;
                let _is_flying = r.bool()?;
                let _waypoint_special_property = r.node::<WaypointSpecialProperty>()?;
                let _is_autonomous = r.bool()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_84<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error);
            }

            r.u32()?;
            let len = r.u32()?;

            {
                let mut r = r.take_with(len as u64, IdState::new(), ());

                self.embedded_item_models = r.list(|r| r.ident())?;
                self.embedded_zip_data = r.byte_buf()?;
                self.textures = r.list(|r| r.string())?;
            }

            Ok(())
        }

        fn read_chunk_85<R, I, N>(&mut self, _: &mut Reader<R, I, N>) -> Result<(), Error> {
            Ok(())
        }

        fn read_chunk_86<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error);
            }

            self.read_light_settings(r)?;

            Ok(())
        }

        fn read_chunk_87<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_89<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error);
            }

            self.world_distortion = r.vec3::<f32>()?;
            r.bool()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_90<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_91<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            let has_lightmaps = r.bool()?;
            r.bool()?;
            r.bool()?;

            if has_lightmaps {
                let lightmaps_version = r.u32()?;

                if lightmaps_version != 10 {
                    return Err(Error);
                }

                let _lightmap_frames = r.list(|r| {
                    r.byte_buf()?;
                    r.byte_buf()?;
                    r.byte_buf()?;

                    Ok(())
                })?;
                let _lightmap_cache_data_len = r.u32()?;
                let _compressed_lightmap_cache_data = r.byte_buf()?;
            }

            Ok(())
        }

        fn read_chunk_93<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.bytes(51079)?;

            Ok(())
        }

        fn read_chunk_94<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.bytes(20)?;

            Ok(())
        }

        fn read_chunk_95<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            for _block in self
                .blocks
                .iter_mut()
                .chain(&mut self.baked_blocks)
                .filter(|block| block.is_free)
            {
                let _absolute_position_in_map = r.vec3::<f32>()?;
                let _pitch_yaw_roll = r.vec3::<f32>()?;
            }

            Ok(())
        }

        fn read_chunk_96<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.bytes(8)?;

            Ok(())
        }

        fn read_chunk_97<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.bytes(20)?;

            Ok(())
        }

        fn read_chunk_98<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            for _block in self.blocks.iter_mut().chain(&mut self.baked_blocks) {
                let _color = MapElemColor::read(r)?;
            }

            for _anchored_object in &mut self.anchored_objects {
                let _color = MapElemColor::read(r)?;
            }

            Ok(())
        }

        fn read_chunk_99<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            for _anchored_object in &mut self.anchored_objects {
                let _anim_phase_offset = PhaseOffset::read(r)?;
            }

            Ok(())
        }

        fn read_chunk_100<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.bytes(16)?;

            Ok(())
        }

        fn read_chunk_101<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            for _anchored_object in &mut self.anchored_objects {
                let has_foreground_pack_desc = r.bool8()?;

                if has_foreground_pack_desc {
                    let _foreground_pack_desc = r.pack_desc()?;
                }
            }

            Ok(())
        }

        fn read_chunk_103<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.bytes(16)?;

            Ok(())
        }

        fn read_chunk_104<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error);
            }

            for _block in self.blocks.iter_mut().chain(&mut self.baked_blocks) {
                let _lightmap_quality = LightmapQuality::read(r)?;
            }

            for _anchored_object in &mut self.anchored_objects {
                let _lightmap_quality = LightmapQuality::read(r)?;
            }

            Ok(())
        }

        fn read_chunk_105<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error);
            }

            for _block in &self.blocks {
                let _macroblock_id = r.u32()?;
            }

            for _anchored_object in &self.anchored_objects {
                let _macroblock_id = r.u32()?;
            }

            let _id_flags_pair = r.list(|r| r.vec2::<u32>());

            Ok(())
        }

        fn read_chunk_107<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.read_light_settings(r)?;

            Ok(())
        }

        fn read_map_origin_and_target<I, N>(
            &mut self,
            r: &mut Reader<impl Read, I, N>,
        ) -> Result<(), Error> {
            self.map_coord_origin = r.vec2::<f32>()?;
            self.map_coord_target = r.vec2::<f32>()?;

            Ok(())
        }

        fn read_author<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let author_version = r.u32()?;

            if author_version != 0 {
                return Err(Error);
            }

            self.author_login = r.string()?;
            self.author_nickname = r.string()?;
            self.author_zone = r.string()?;
            self.author_extra_info = r.string()?;

            Ok(())
        }

        fn read_light_settings<I, N>(
            &mut self,
            r: &mut Reader<impl Read, I, N>,
        ) -> Result<(), Error> {
            r.u32()?;
            self.day_time = r.f32()?;
            r.u32()?;
            self.dynamic_daylight = r.bool()?;
            self.day_duration = r.u32()?;

            Ok(())
        }
    }

    fn read_blocks(
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<Box<[Block]>, Error> {
        let version = r.u32()?;

        if version != 6 {
            return Err(Error);
        }

        r.list(|r| Block::read(r))
    }
}

mod write {
    use std::io::Write;

    use crate::{
        write::{
            writable::{WriteBody, WriteUserData},
            IdStateMut, Writer,
        },
        Error,
    };

    use super::{Challenge, EditorMode, MapKind, PlayMode};

    impl PlayMode {
        fn write<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let value = match *self {
                Self::Race => 0,
                Self::Platform => 1,
                Self::Puzzle => 2,
                Self::Crazy => 3,
                Self::Shortcut => 4,
                Self::Stunts => 5,
                Self::Script => 6,
            };

            w.u32(value)
        }
    }

    impl EditorMode {
        fn write<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let value = match *self {
                Self::Advanced => 0,
                Self::Simple => 1,
                Self::HasGhostBlocks => 2,
                Self::Gamepad => 4,
            };

            w.u32(value)
        }
    }

    impl MapKind {
        fn write<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let value = match *self {
                Self::EndMarker => 0,
                Self::Campaign => 1,
                Self::Puzzle => 2,
                Self::Retro => 3,
                Self::TimeAttack => 4,
                Self::Rounds => 5,
                Self::InProgress => 6,
                Self::Multi => 8,
                Self::Solo => 9,
                Self::Site => 10,
                Self::SoloNadeo => 11,
                Self::MultiNadeo => 12,
            };

            w.u8(value)
        }
    }

    impl WriteUserData for Challenge {
        fn write_user_data<W: Write, I: IdStateMut, N>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            self.write_chunk_2(w)?;
            self.write_chunk_3(w)?;
            self.write_chunk_4(w)?;
            self.write_chunk_5(w)?;
            self.write_chunk_7(w)?;
            self.write_chunk_8(w)?;

            Ok(())
        }
    }

    impl WriteBody for Challenge {
        fn write_body<W: Write, I: IdStateMut, N>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            self.write_chunk_13(w)?;

            Ok(())
        }
    }

    impl Challenge {
        fn write_chunk_2<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let version = 13;

            w.u8(version)?;
            w.bool(false)?;
            w.u32(self.bronze_time)?;
            w.u32(self.silver_time)?;
            w.u32(self.gold_time)?;
            w.u32(self.author_time)?;
            w.u32(self.cost)?;
            w.bool(self.is_lap_race)?;
            self.mode.write(w)?;
            w.u32(0)?;
            w.u32(self.author_score)?;
            self.editor.write(w)?;
            w.u32(0)?;
            w.u32(self.num_checkpoints)?;
            w.u32(self.num_laps)?;

            Ok(())
        }

        fn write_chunk_3<N>(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = 11;

            w.u8(version)?;
            w.ident(&self.map_info)?;
            w.string(&self.map_name)?;
            self.map_kind.write(w)?;
            w.u32(0)?;
            w.string(&self.password)?;
            w.ident(&self.decoration)?;
            self.write_map_origin_and_target(w)?;
            w.u128(self.pack_mask)?;
            w.string(&self.map_type)?;
            w.string(&self.map_style)?;
            w.u64(self.lightmap_cache_uid)?;
            w.u8(self.lightmap_version)?;
            w.id(self.title_id.as_ref())?;

            Ok(())
        }

        fn write_chunk_4<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let version = 6;

            w.u32(version)?;

            Ok(())
        }

        fn write_chunk_5<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.string(&self.xml)?;

            Ok(())
        }

        fn write_chunk_7<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let version = 1;

            w.u32(version)?;
            w.u32(self.thumbnail.len() as u32)?;
            w.bytes(b"<Thumbnail.jpg>")?;
            w.bytes(&self.thumbnail)?;
            w.bytes(b"</Thumbnail.jpg>")?;
            w.bytes(b"<Comments>")?;
            w.string(&self.comments)?;
            w.bytes(b"</Comments>")?;

            Ok(())
        }

        fn write_chunk_8<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let version = 1;

            w.u32(version)?;
            self.write_author(w)?;

            Ok(())
        }

        fn write_chunk_13<N>(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            w.ident(&self.player_model)?;

            Ok(())
        }

        fn write_map_origin_and_target<I, N>(
            &self,
            w: &mut Writer<impl Write, I, N>,
        ) -> Result<(), Error> {
            w.vec2(&self.map_coord_origin)?;
            w.vec2(&self.map_coord_target)?;

            Ok(())
        }

        fn write_author<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let version = 0;

            w.u32(version)?;
            w.string(&self.author_login)?;
            w.string(&self.author_nickname)?;
            w.string(&self.author_zone)?;
            w.string(&self.author_extra_info)?;

            Ok(())
        }
    }
}
