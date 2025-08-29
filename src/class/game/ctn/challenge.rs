use std::{io, sync::Arc};

use crate::{
    game::{
        WaypointSpecialProperty,
        ctn::{
            AnchoredObject, Block, ChallengeParameters, CollectorList, FileRef, Ident,
            ZoneGenealogy, block,
            media::{self, Clip},
        },
        read_encapsulation,
    },
    read::{
        self, BodyReader, Error, HeaderReader, Read, ReadEnum, ReadNode, Reader, Result,
        read_body_chunks, read_header_chunks,
    },
    scene::VehicleCarMarksSamples,
    script::TraitsMetadata,
    write::{self, BodyWriter, Write},
};

pub struct Challenge {
    chunk_2: Chunk2,
    chunk_3: Chunk3,
    chunk_4: Chunk4,
    chunk_5: Chunk5,
    chunk_7: Chunk7,
    chunk_8: Chunk8,
    chunk_13: Chunk13,
    chunk_17: Chunk17,
    chunk_24: Chunk24,
    chunk_25: Chunk25,
    chunk_31: Chunk31,
    chunk_34: Chunk34,
    chunk_36: Chunk36,
    chunk_37: Chunk37,
    chunk_38: Option<Chunk38>,
    chunk_40: Option<Chunk40>,
    chunk_41: Chunk41,
    chunk_42: Chunk42,
    chunk_52: Chunk52,
    chunk_54: Chunk54,
    chunk_56: Option<Chunk56>,
    chunk_62: Chunk62,
    chunk_64: Chunk64,
    chunk_66: Chunk66,
    chunk_67: Chunk67,
    chunk_68: Chunk68,
    chunk_72: Chunk72,
    chunk_73: Chunk73,
    chunk_75: Chunk75,
    chunk_79: Chunk79,
    chunk_80: Chunk80,
    chunk_81: Chunk81,
    chunk_82: Chunk82,
    chunk_83: Chunk83,
    chunk_84: Chunk84,
    chunk_85: Chunk85,
    chunk_86: Chunk86,
    chunk_87: Chunk87,
    chunk_88: Option<Chunk88>,
    chunk_89: Chunk89,
    chunk_90: Chunk90,
    chunk_91: Chunk91,
    chunk_92: Option<Chunk92>,
    chunk_93: Chunk93,
    chunk_94: Chunk94,
    chunk_95: Chunk95,
    chunk_96: Chunk96,
    chunk_97: Option<Chunk97>,
    chunk_98: Option<Chunk98>,
    chunk_99: Option<Chunk99>,
    chunk_100: Option<Chunk100>,
    chunk_101: Option<Chunk101>,
    chunk_103: Option<Chunk103>,
    chunk_104: Option<Chunk104>,
    chunk_105: Option<Chunk105>,
    chunk_107: Option<Chunk107>,
    chunk_108: Option<Chunk108>,
}

struct Chunk2;

struct Chunk3;

struct Chunk4;

struct Chunk5;

struct Chunk7;

struct Chunk8;

struct Chunk13;

struct Chunk17;

struct Chunk24;

struct Chunk25 {
    texture_mod: Option<FileRef>,
}

struct Chunk31 {
    blocks: Box<[Block]>,
}

struct Chunk34;

struct Chunk36 {
    music: Option<FileRef>,
}

struct Chunk37;

struct Chunk38;

struct Chunk40;

struct Chunk41;

struct Chunk42;

struct Chunk52;

struct Chunk54;

struct Chunk56;

struct Chunk62;

struct Chunk64 {
    items: Box<[AnchoredObject]>,
}

struct Chunk66;

struct Chunk67;

struct Chunk68;

struct Chunk72 {
    baked_blocks: Box<[Block]>,
}

struct Chunk73 {
    intro_media: Option<Arc<media::Clip>>,
    podium_media: Option<Arc<media::Clip>>,
    in_game_media: Option<Arc<media::ClipGroup>>,
    end_race_media: Option<Arc<media::ClipGroup>>,
    ambiance_media: Option<Arc<media::Clip>>,
}

struct Chunk75;

struct Chunk79;

struct Chunk80;

struct Chunk81;

struct Chunk82;

struct Chunk83;

struct Chunk84 {
    embedded_models: Option<EmbeddedModels>,
}

struct Chunk85;

struct Chunk86;

struct Chunk87;

struct Chunk88;

struct Chunk89;

struct Chunk90;

struct Chunk91;

struct Chunk92;

struct Chunk93;

struct Chunk94;

struct Chunk95;

struct Chunk96;

struct Chunk97;

struct Chunk98;

struct Chunk99;

struct Chunk100;

struct Chunk101;

struct Chunk103;

struct Chunk104;

struct Chunk105;

struct Chunk107;

struct Chunk108 {
    color_palette: ColorPalette,
}

enum PlayMode {
    Race,
    Platform,
    Puzzle,
    Crazy,
    Shortcut,
    Stunts,
    Script,
}

impl ReadEnum for PlayMode {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::Race),
            1 => Ok(Self::Platform),
            3 => Ok(Self::Puzzle),
            4 => Ok(Self::Crazy),
            5 => Ok(Self::Shortcut),
            6 => Ok(Self::Stunts),
            7 => Ok(Self::Script),
            _ => todo!(),
        }
    }
}

enum EditorMode {
    Advanced,
    Simple,
    HasGhostBlocks,
    Gamepad,
}

impl ReadEnum for EditorMode {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::Advanced),
            1 => Ok(Self::Simple),
            2 => Ok(Self::HasGhostBlocks),
            4 => Ok(Self::Gamepad),
            _ => todo!(),
        }
    }
}

enum MapKind {
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

impl ReadEnum for MapKind {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::EndMarker),
            1 => Ok(Self::Campaign),
            2 => Ok(Self::Puzzle),
            3 => Ok(Self::Retro),
            4 => Ok(Self::TimeAttack),
            5 => Ok(Self::Rounds),
            6 => Ok(Self::InProgress),
            7 => Ok(Self::Campaign7),
            8 => Ok(Self::Multi),
            9 => Ok(Self::Solo),
            10 => Ok(Self::Site),
            11 => Ok(Self::SoloNadeo),
            12 => Ok(Self::MultiNadeo),
            _ => Err(Error::Internal(
                "unknown variant index of enum MapKind".into(),
            )),
        }
    }
}

/// Items or custom blocks embedded in a challenge.
pub struct EmbeddedModels {
    zip_data: Box<[u8]>,
}

#[derive(Clone, Copy, Default)]
pub enum ColorPalette {
    #[default]
    Classic,
    Stunt,
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Cyan,
    Blue,
    Purple,
    Pink,
    White,
    Black,
}

impl ReadEnum for ColorPalette {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::Classic),
            1 => Ok(Self::Stunt),
            2 => Ok(Self::Red),
            3 => Ok(Self::Orange),
            4 => Ok(Self::Yellow),
            5 => Ok(Self::Lime),
            6 => Ok(Self::Green),
            7 => Ok(Self::Cyan),
            8 => Ok(Self::Blue),
            9 => Ok(Self::Purple),
            10 => Ok(Self::Pink),
            11 => Ok(Self::White),
            12 => Ok(Self::Black),
            _ => todo!(),
        }
    }
}

impl Challenge {
    /// Custom texture mod.
    pub fn texture_mod(&self) -> Option<&FileRef> {
        self.chunk_25.texture_mod.as_ref()
    }

    /// Blocks placed in the challenge.
    pub fn blocks(&self) -> &[Block] {
        &self.chunk_31.blocks
    }

    /// Custom music.
    pub fn music(&self) -> Option<&FileRef> {
        self.chunk_36.music.as_ref()
    }

    /// Items placed in the challenge.
    pub fn items(&self) -> &[AnchoredObject] {
        &self.chunk_64.items
    }

    pub fn baked_blocks(&self) -> &[Block] {
        &self.chunk_72.baked_blocks
    }

    /// Intro media tracker clip.
    pub fn intro_media(&self) -> Option<&media::Clip> {
        self.chunk_73.intro_media.as_deref()
    }

    /// Podium media tracker clip.
    pub fn podium_media(&self) -> Option<&media::Clip> {
        self.chunk_73.podium_media.as_deref()
    }

    /// In game media tracker clips.
    pub fn in_game_media(&self) -> Option<&media::ClipGroup> {
        self.chunk_73.in_game_media.as_deref()
    }

    /// End race media tracker clips.
    pub fn end_race_media(&self) -> Option<&media::ClipGroup> {
        self.chunk_73.end_race_media.as_deref()
    }

    /// Ambiance media tracker clip.
    pub fn ambiance_media(&self) -> Option<&media::Clip> {
        self.chunk_73.ambiance_media.as_deref()
    }

    /// Items or custom blocks embedded in the challenge.
    pub fn embedded_models(&self) -> Option<&EmbeddedModels> {
        self.chunk_84.embedded_models.as_ref()
    }

    pub fn color_palette(&self) -> ColorPalette {
        match self.chunk_108 {
            None => ColorPalette::default(),
            Some(ref chunk_108) => chunk_108.color_palette,
        }
    }
}

impl Read for Challenge {}

impl read::sealed::Read for Challenge {
    const CLASS_ID: u32 = 0x03043000;

    fn read(header_data: &[u8], r: &mut impl BodyReader) -> Result<Self> {
        read_header_chunks(header_data, |hr| {
            read_body_chunks(r, |r| {
                let chunk_2 = hr.chunk(0x03043002, |r| {
                    let version = r.u8()?;

                    if version != 13 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _need_unlock = r.bool32()?;
                    let _bronze_time = r.u32()?;
                    let _silver_time = r.u32()?;
                    let _gold_time = r.u32()?;
                    let _author_time = r.u32()?;
                    let _cost = r.u32()?;
                    let _is_lap_race = r.bool32()?;
                    let _mode = r.enum32::<PlayMode>()?;
                    r.u32()?;
                    let _author_score = r.u32()?;
                    let _editor = r.enum32::<EditorMode>()?;
                    r.u32()?;
                    let _num_checkpoints = r.u32()?;
                    let _num_laps = r.u32()?;

                    Ok(Chunk2)
                })?;
                let chunk_3 = hr.chunk(0x03043003, |r| {
                    let version = r.u8()?;

                    if version != 11 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _map_info = Ident::read(r)?;
                    let _map_name = r.string()?;
                    let _kind = r.enum8::<MapKind>()?;
                    r.u32()?;
                    let _password = r.string()?;
                    let _decoration = Ident::read(r)?;
                    let _map_coord_origin = r.vec2_f32()?;
                    let _map_coord_target = r.vec2_f32()?;
                    let _pack_mask = r.u128()?;
                    let _map_type = r.string()?;
                    let _map_style = r.string()?;
                    let _lightmap_cache_id = r.u64()?;
                    let _lightmap_version = r.u8()?;
                    let _title_id = r.string_ref::<Arc<str>>()?;

                    Ok(Chunk3)
                })?;
                let chunk_4 = hr.chunk(0x03043004, |r| {
                    let version = r.u32()?;

                    if version != 6 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    Ok(Chunk4)
                })?;
                let chunk_5 = hr.chunk(0x03043005, |r| {
                    let _xml = r.string()?;

                    Ok(Chunk5)
                })?;
                let chunk_7 = hr.chunk(0x03043007, |r| {
                    let version = r.u32()?;

                    if version != 1 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let thumbnail_size = r.u32()?;

                    if r.array_u8()? != *b"<Thumbnail.jpg>" {
                        todo!()
                    }

                    let _thumbnail = r.repeat_u8(thumbnail_size as usize)?;

                    if r.array_u8()? != *b"</Thumbnail.jpg>" {
                        todo!()
                    }

                    if r.array_u8()? != *b"<Comments>" {
                        todo!()
                    }

                    let _comments = r.string()?;

                    if r.array_u8()? != *b"</Comments>" {
                        todo!()
                    }

                    Ok(Chunk7)
                })?;
                let chunk_8 = hr.chunk(0x03043008, |r| {
                    let version = r.u32()?;

                    if version != 1 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let author_version = r.u32()?;

                    if author_version != 0 {
                        return Err(Error::Internal("unknown author version".into()));
                    }

                    let _author_login = r.string()?;
                    let _author_nickname = r.string()?;
                    let _author_zone = r.string()?;
                    let _author_extra_info = r.string()?;

                    Ok(Chunk8)
                })?;
                let chunk_13 = r.chunk(0x0304300d, |r| {
                    let _player_model = Ident::read(r)?;

                    Ok(Chunk13)
                })?;
                let chunk_17 = r.chunk(0x03043011, |r| {
                    let _block_stock = r.node_ref::<Arc<CollectorList>>()?;
                    let _challenge_parameters = r.node_ref::<Arc<ChallengeParameters>>()?;
                    let _map_kind = r.enum32::<MapKind>()?;

                    Ok(Chunk17)
                })?;
                let chunk_24 = r.chunk_skippable(0x03043018, |r| {
                    let _is_lap_race = r.bool32()?;
                    let _num_laps = r.u32()?;

                    Ok(Chunk24)
                })?;
                let chunk_25 = r.chunk_skippable(0x03043019, |r| {
                    let texture_mod = FileRef::read(r)?;

                    Ok(Chunk25 { texture_mod })
                })?;
                let mut chunk_31 = r.chunk(0x0304301f, |r| {
                    let _map_ident = Ident::read(r)?;
                    let _map_name = r.string()?;
                    let _deco = Ident::read(r)?;
                    let _size = r.vec3_u32()?;
                    let _need_unlock = r.bool32()?;

                    if r.u32()? != 6 {
                        return Err(Error::Internal("unknown blocks version".into()));
                    }

                    let blocks = r.list(Block::read)?;

                    Ok(Chunk31 { blocks })
                })?;
                let chunk_34 = r.chunk(0x03043022, |r| {
                    r.u32()?;

                    Ok(Chunk34)
                })?;
                let chunk_36 = r.chunk(0x03043024, |r| {
                    let music = FileRef::read(r)?;

                    Ok(Chunk36 { music })
                })?;
                let chunk_37 = r.chunk(0x03043025, |r| {
                    let _map_coord_origin = r.vec2_f32()?;
                    let _map_coord_target = r.vec2_f32()?;

                    Ok(Chunk37)
                })?;
                let chunk_38 = r.chunk_optional(0x03043026, |r| {
                    let _clip_global = r.node_ref::<Option<Arc<Clip>>>()?;

                    Ok(Chunk38)
                })?;
                let chunk_40 = r.chunk_optional(0x03043028, |r| {
                    if r.bool32()? {
                        r.u8()?;
                        let _thumbnail_position = r.iso4()?;
                        let _thumbnail_fov = r.f32()?;
                        let _thumbnail_near_clip_plane = r.f32()?;
                        let _thumbnail_far_clip_plane = r.f32()?;
                    }

                    let _comments = r.string()?;

                    Ok(Chunk40)
                })?;
                let chunk_41 = r.chunk_skippable(0x03043029, |r| {
                    let _hashed_password = r.u128()?;
                    let _crc32 = r.u32()?;

                    Ok(Chunk41)
                })?;
                let chunk_42 = r.chunk(0x0304302a, |r| {
                    let _created_with_simple_editor = r.bool32()?;

                    Ok(Chunk42)
                })?;
                let chunk_52 = r.chunk_skippable(0x03043034, |r| {
                    r.list_u8()?;

                    Ok(Chunk52)
                })?;
                let chunk_54 = r.chunk_skippable(0x03043036, |r| {
                    let _thumbnail_position = r.vec3_f32()?;
                    let _thumbnail_yaw_pitch_roll = r.yaw_pitch_roll()?;
                    let _thumbnail_fov = r.f32()?;
                    r.f32()?;
                    r.f32()?;
                    let _thumbnail_near_clip_plane = r.f32()?;
                    let _thumbnail_far_clip_plane = r.f32()?;
                    let _comments = r.string()?;

                    Ok(Chunk54)
                })?;
                let chunk_56 = r.chunk_skippable_optional(0x03043038, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    Ok(Chunk56)
                })?;
                let chunk_62 = r.chunk_skippable(0x0304303e, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _car_marks_buffer =
                        r.list_versioned(|r| r.node_ref::<Arc<VehicleCarMarksSamples>>())?;

                    Ok(Chunk62)
                })?;
                let mut chunk_64 = r.chunk_skippable(0x03043040, |r| {
                    let version = r.u32()?;

                    if !matches!(version, 5 | 8) {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    read_encapsulation(r, |r| {
                        let items = r.list_versioned(|r| r.node::<AnchoredObject>())?;
                        let _block_indices = r.list(|r| r.u32())?;

                        if version == 8 {
                            let _item_indices = r.list(|r| r.u32())?;
                        }

                        let _snap_item_groups = r.list(|r| r.u32())?;
                        r.list(|r| r.u32())?;
                        let _snapped_indices = r.list(|r| r.u32())?;

                        Ok(Chunk64 { items })
                    })
                })?;
                let chunk_66 = r.chunk_skippable(0x03043042, |r| {
                    if r.u32()? != 1 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown author version".into()));
                    }

                    let _author_login = r.string()?;
                    let _author_nickname = r.string()?;
                    let _author_zone = r.string()?;
                    let _author_extra_info = r.string()?;

                    Ok(Chunk66)
                })?;
                let chunk_67 = r.chunk_skippable(0x03043043, |r| {
                    read_encapsulation(r, |r| {
                        let _zone_genealogy = r.list(|r| r.node::<ZoneGenealogy>())?;

                        Ok(Chunk67)
                    })
                })?;
                let chunk_68 = r.chunk_skippable(0x03043044, |r| {
                    read_encapsulation(r, |r| {
                        let _script_metadata = TraitsMetadata::read_node(r)?;

                        Ok(Chunk68)
                    })
                })?;
                let mut chunk_72 = r.chunk_skippable(0x03043048, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    if r.u32()? != 6 {
                        return Err(Error::Internal("unknown blocks version".into()));
                    }

                    let baked_blocks = r.list(|r| Block::read(r))?;
                    r.u32()?;
                    r.list(|r| {
                        Ident::read(r)?;
                        Ident::read(r)?;
                        Ident::read(r)?;
                        Ident::read(r)?;
                        r.vec3_u32()?;

                        Ok(())
                    })?;

                    Ok(Chunk72 { baked_blocks })
                })?;
                let chunk_73 = r.chunk(0x03043049, |r| {
                    if r.u32()? != 2 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let intro_media = r.node_ref()?;
                    let podium_media = r.node_ref()?;
                    let in_game_media = r.node_ref()?;
                    let end_race_media = r.node_ref()?;
                    let ambiance_media = r.node_ref()?;
                    let _clip_trigger_size = r.vec3_u32()?;

                    Ok(Chunk73 {
                        intro_media,
                        podium_media,
                        in_game_media,
                        end_race_media,
                        ambiance_media,
                    })
                })?;
                let chunk_75 = r.chunk_skippable(0x0304304b, |r| {
                    let _objective_text_author = r.string()?;
                    let _objective_text_gold = r.string()?;
                    let _objective_text_silver = r.string()?;
                    let _objective_text_bronze = r.string()?;

                    Ok(Chunk75)
                })?;
                let chunk_79 = r.chunk_skippable(0x0304304f, |r| {
                    if r.u32()? != 3 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    r.u8()?;

                    Ok(Chunk79)
                })?;
                let chunk_80 = r.chunk_skippable(0x03043050, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _offzone_trigger_size = r.vec3_f32()?;
                    let _offzones = r.list(|r| r.box3_u32())?;

                    Ok(Chunk80)
                })?;
                let chunk_81 = r.chunk_skippable(0x03043051, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _title_id = r.string_ref::<Arc<str>>()?;
                    let _build_version = r.string()?;

                    Ok(Chunk81)
                })?;
                let chunk_82 = r.chunk_skippable(0x03043052, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _deco_height_base_offset = r.u32()?;

                    Ok(Chunk82)
                })?;
                let chunk_83 = r.chunk_skippable(0x03043053, |r| {
                    if r.u32()? != 3 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _bot_paths = r.list(|r| {
                        let _clan = r.u32()?;
                        let _path = r.list(|r| r.vec3_f32())?;
                        let _is_flying = r.bool32()?;
                        let _waypoint_special_property =
                            r.node_ref::<Arc<WaypointSpecialProperty>>()?;
                        let _is_autonomous = r.bool32()?;

                        Ok(())
                    })?;

                    Ok(Chunk83)
                })?;
                let chunk_84 = r.chunk_skippable(0x03043054, |r| {
                    if r.u32()? != 1 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    read_encapsulation(r, |r| {
                        let _embedded_models = r.list(|r| {
                            let _ident = Ident::read(r)?;

                            Ok(())
                        })?;
                        let zip_data = r.list_u8()?;
                        let _textures = r.list(|r| r.string())?;

                        let embedded_models = if zip_data.is_empty() {
                            None
                        } else {
                            Some(EmbeddedModels { zip_data })
                        };

                        Ok(Chunk84 { embedded_models })
                    })
                })?;
                let chunk_85 = r.chunk_skippable(0x03043055, |_| Ok(Chunk85))?;
                let chunk_86 = r.chunk_skippable(0x03043056, |r| {
                    if r.u32()? != 3 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    r.u32()?;
                    let _day_time = r.u32()?;
                    r.u32()?;
                    let _dynamic_daylight = r.bool32()?;
                    let _day_duration = r.u32()?;

                    Ok(Chunk86)
                })?;
                let chunk_87 = r.chunk_skippable(0x03043057, |r| {
                    let version = r.u32()?;

                    if !matches!(version, 4 | 5) {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    r.u32()?;

                    Ok(Chunk87)
                })?;
                let chunk_88 = r.chunk_skippable_optional(0x03043058, |r| {
                    if r.u32()? != 1 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    if r.u32()? > 0 {
                        todo!()
                    }

                    Ok(Chunk88)
                })?;
                let chunk_89 = r.chunk_skippable(0x03043059, |r| {
                    if r.u32()? != 3 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _world_distortion = r.vec3_f32()?;

                    if r.bool32()? {
                        todo!()
                    }

                    r.u32()?;
                    r.u32()?;

                    Ok(Chunk89)
                })?;
                let chunk_90 = r.chunk_skippable(0x0304305a, |r| {
                    r.u32()?;
                    r.u32()?;

                    Ok(Chunk90)
                })?;
                let chunk_91 = r.chunk_skippable(0x0304305b, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let has_lightmaps = r.bool32()?;
                    r.u32()?;
                    r.u32()?;

                    if has_lightmaps {
                        let lightmaps_version = r.u32()?;

                        if !matches!(lightmaps_version, 8 | 10) {
                            return Err(Error::Internal("unknown lightmaps version".into()));
                        }

                        let lightmap_frames = r.list(|r| {
                            r.list_u8()?;
                            r.list_u8()?;
                            r.list_u8()?;

                            Ok(())
                        })?;

                        if !lightmap_frames.is_empty() {
                            let _lightmap_cache_data_size = r.u32()?;
                            let _compressed_lightmap_cache_data = r.list_u8()?;
                        }
                    }

                    Ok(Chunk91)
                })?;
                let chunk_92 = r.chunk_skippable_optional(0x0304305c, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    r.u32()?;
                    r.u32()?;

                    Ok(Chunk92)
                })?;
                let chunk_93 = r.chunk_skippable(0x0304305d, |r| {
                    let version = r.u32()?;

                    if !matches!(version, 0 | 1) {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    r.u32()?;

                    Ok(Chunk93)
                })?;
                let chunk_94 = r.chunk_skippable(0x0304305e, |r| {
                    if r.u32()? != 1 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    r.u32()?;
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;

                    Ok(Chunk94)
                })?;
                let chunk_95 = r.chunk_skippable(0x0304305f, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    for block in &mut chunk_31.blocks {
                        if let block::Type::Free { position, rotation } = &mut block.ty {
                            *position = r.vec3_f32()?;
                            *rotation = r.yaw_pitch_roll()?;
                        }
                    }

                    for block in &mut chunk_72.baked_blocks {
                        if let block::Type::Free { position, rotation } = &mut block.ty {
                            *position = r.vec3_f32()?;
                            *rotation = r.yaw_pitch_roll()?;
                        }
                    }

                    Ok(Chunk95)
                })?;
                let chunk_96 = r.chunk_skippable(0x03043060, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    r.u32()?;

                    Ok(Chunk96)
                })?;
                let chunk_97 = r.chunk_skippable_optional(0x03043061, |r| {
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;

                    Ok(Chunk97)
                })?;
                let chunk_98 = r.chunk_skippable_optional(0x03043062, |r| {
                    let version = r.u32()?;

                    if version != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    for block in &mut chunk_31.blocks {
                        block.elem_color = r.enum8()?;
                    }

                    for block in &mut chunk_72.baked_blocks {
                        block.elem_color = r.enum8()?;
                    }

                    for item in &mut chunk_64.items {
                        item.elem_color = r.enum8()?;
                    }

                    Ok(Chunk98)
                })?;
                let chunk_99 = r.chunk_skippable_optional(0x03043063, |r| {
                    let version = r.u32()?;

                    if version != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    for item in &mut chunk_64.items {
                        item.anim_offset = r.enum8()?;
                    }

                    Ok(Chunk99)
                })?;
                let chunk_100 = r.chunk_skippable_optional(0x03043064, |r| {
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;

                    Ok(Chunk100)
                })?;
                let chunk_101 = r.chunk_skippable_optional(0x03043065, |r| {
                    let version = r.u32()?;

                    if version != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    for item in &mut chunk_64.items {
                        if r.bool8()? {
                            let _foreground_file_ref = FileRef::read(r)?;
                        }
                    }

                    Ok(Chunk101)
                })?;
                let chunk_103 = r.chunk_skippable_optional(0x03043067, |r| {
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;

                    Ok(Chunk103)
                })?;
                let chunk_104 = r.chunk_skippable_optional(0x03043068, |r| {
                    let version = r.u32()?;

                    if version != 1 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    for block in &mut chunk_31.blocks {
                        block.lightmap_quality = r.enum8()?;
                    }

                    for block in &mut chunk_72.baked_blocks {
                        block.lightmap_quality = r.enum8()?;
                    }

                    for item in &mut chunk_64.items {
                        item.lightmap_quality = r.enum8()?;
                    }

                    Ok(Chunk104)
                })?;
                let chunk_105 = r.chunk_skippable_optional(0x03043069, |r| {
                    let version = r.u32()?;

                    if version != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    for block in &mut chunk_31.blocks {
                        let _macroblock_index = r.u32()?;
                    }

                    for item in &mut chunk_64.items {
                        let _macroblock_index = r.u32()?;
                    }

                    let _flags = r.list(|r| {
                        let _index = r.u32()?;
                        let _flags = r.u32()?;

                        Ok(())
                    })?;

                    Ok(Chunk105)
                })?;
                let chunk_107 = r.chunk_skippable_optional(0x0304306b, |r| {
                    r.u32()?;
                    let _day_time = r.u32()?;
                    r.u32()?;
                    let _dynamic_daylight = r.bool32()?;
                    let _day_duration = r.u32()?;

                    Ok(Chunk107)
                })?;
                let chunk_108 = r.chunk_skippable_optional(0x0304306c, |r| {
                    let version = r.u32()?;

                    if version != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let color_palette = r.enum8()?;

                    Ok(Chunk108 { color_palette })
                })?;

                Ok(Self {
                    chunk_2,
                    chunk_3,
                    chunk_4,
                    chunk_5,
                    chunk_7,
                    chunk_8,
                    chunk_13,
                    chunk_17,
                    chunk_24,
                    chunk_25,
                    chunk_31,
                    chunk_34,
                    chunk_36,
                    chunk_37,
                    chunk_38,
                    chunk_40,
                    chunk_41,
                    chunk_42,
                    chunk_52,
                    chunk_54,
                    chunk_56,
                    chunk_62,
                    chunk_64,
                    chunk_66,
                    chunk_67,
                    chunk_68,
                    chunk_72,
                    chunk_73,
                    chunk_75,
                    chunk_79,
                    chunk_80,
                    chunk_81,
                    chunk_82,
                    chunk_83,
                    chunk_84,
                    chunk_85,
                    chunk_86,
                    chunk_87,
                    chunk_88,
                    chunk_89,
                    chunk_90,
                    chunk_91,
                    chunk_92,
                    chunk_93,
                    chunk_94,
                    chunk_95,
                    chunk_96,
                    chunk_97,
                    chunk_98,
                    chunk_99,
                    chunk_100,
                    chunk_101,
                    chunk_103,
                    chunk_104,
                    chunk_105,
                    chunk_107,
                    chunk_108,
                })
            })
        })
    }
}

impl Write for Challenge {}

impl write::sealed::Write for Challenge {
    const CLASS_ID: u32 = 0x03043000;

    fn write_header(&self, w: &mut impl write::HeaderWriter) -> io::Result<()> {
        todo!()
    }

    fn write_body(&self, w: &mut impl BodyWriter) -> io::Result<()> {
        todo!()
    }
}
