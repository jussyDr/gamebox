use std::{io, sync::Arc};

use crate::{
    game::{
        WaypointSpecialProperty,
        ctn::{
            AnchoredObject, Block, ChallengeParameters, CollectorList, FileRef, MediaClip,
            MediaClipGroup, ZoneGenealogy,
        },
        read_encapsulation,
    },
    read::{
        self, BodyReader, Error, HeaderReader, Read, ReadEnum, ReadNode, Reader, Result,
        read_body_chunks,
    },
    scene::VehicleCarMarksSamples,
    script::TraitsMetadata,
    write::{self, BodyWriter, Write},
};

pub struct Challenge {
    chunk_13: Chunk13,
    chunk_17: Chunk17,
    chunk_24: Chunk24,
    chunk_25: Chunk25,
    chunk_31: Chunk31,
    chunk_34: Chunk34,
    chunk_36: Chunk36,
    chunk_37: Chunk37,
    chunk_38: Chunk38,
    chunk_40: Chunk40,
    chunk_41: Chunk41,
    chunk_42: Chunk42,
    chunk_52: Chunk52,
    chunk_54: Chunk54,
    chunk_56: Chunk56,
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
    chunk_88: Chunk88,
    chunk_89: Chunk89,
    chunk_90: Chunk90,
    chunk_91: Chunk91,
    chunk_92: Chunk92,
    chunk_93: Chunk93,
    chunk_94: Chunk94,
    chunk_95: Chunk95,
    chunk_96: Chunk96,
}

struct Chunk13;

struct Chunk17;

struct Chunk24;

struct Chunk25;

struct Chunk31 {
    blocks: Box<[Block]>,
}

struct Chunk34;

struct Chunk36;

struct Chunk37;

struct Chunk38;

struct Chunk40;

struct Chunk41;

struct Chunk42;

struct Chunk52;

struct Chunk54;

struct Chunk56;

struct Chunk62;

struct Chunk64;

struct Chunk66;

struct Chunk67;

struct Chunk68;

struct Chunk72 {
    baked_blocks: Box<[Block]>,
}

struct Chunk73;

struct Chunk75;

struct Chunk79;

struct Chunk80;

struct Chunk81;

struct Chunk82;

struct Chunk83;

struct Chunk84;

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

enum MapKind {
    InProgress,
}

impl ReadEnum for MapKind {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            6 => Ok(Self::InProgress),
            _ => Err(Error::Internal(
                "unknown variant index of enum MapKind".into(),
            )),
        }
    }
}

impl Read for Challenge {}

impl read::sealed::Read for Challenge {
    const CLASS_ID: u32 = 0x03043000;

    fn read_body(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            let chunk_13 = r.chunk(0x0304300d, |r| {
                let _player_model_id = r.string_ref()?;
                let _player_model_collection = r.string_ref()?;
                let _player_model_author = r.string_ref()?;

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
                let _texture_mod = FileRef::read(r)?;

                Ok(Chunk25)
            })?;
            let chunk_31 = r.chunk(0x0304301f, |r| {
                let _map_id = r.string_ref()?;
                let _map_collection = r.string_ref()?;
                let _map_author = r.string_ref()?;
                let _map_name = r.string()?;
                let _deco_id = r.string_ref()?;
                let _deco_collection = r.string_ref()?;
                let _deco_author = r.string_ref()?;
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
                let _music = FileRef::read(r)?;

                Ok(Chunk36)
            })?;
            let chunk_37 = r.chunk(0x03043025, |r| {
                let _map_coord_origin = r.vec2_f32()?;
                let _map_coord_target = r.vec2_f32()?;

                Ok(Chunk37)
            })?;
            let chunk_38 = r.chunk(0x03043026, |r| {
                let _clip_global = r.node_ref::<Arc<MediaClip>>()?;

                Ok(Chunk38)
            })?;
            let chunk_40 = r.chunk(0x03043028, |r| {
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
            let chunk_56 = r.chunk_skippable(0x03043038, |r| {
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
            let chunk_64 = r.chunk_skippable(0x03043040, |r| {
                if r.u32()? != 5 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                read_encapsulation(r, |r| {
                    let _anchored_objects = r.list_versioned(|r| r.node::<AnchoredObject>())?;
                    let _block_indices = r.list(|r| r.u32())?;
                    let _snap_item_groups = r.list(|r| r.u32())?;
                    r.list(|r| r.u32())?;
                    let _snapped_indices = r.list(|r| r.u32())?;

                    Ok(())
                })?;

                Ok(Chunk64)
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

                    Ok(())
                })?;

                Ok(Chunk67)
            })?;
            let chunk_68 = r.chunk_skippable(0x03043044, |r| {
                read_encapsulation(r, |r| {
                    let _script_metadata = TraitsMetadata::read_node(r)?;

                    Ok(())
                })?;

                Ok(Chunk68)
            })?;
            let chunk_72 = r.chunk_skippable(0x03043048, |r| {
                if r.u32()? != 0 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                if r.u32()? != 6 {
                    return Err(Error::Internal("unknown blocks version".into()));
                }

                let baked_blocks = r.list(|r| Block::read(r))?;
                r.u32()?;
                r.list(|r| {
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.string_ref()?;
                    r.vec3_u32()?;

                    Ok(())
                })?;

                Ok(Chunk72 { baked_blocks })
            })?;
            let chunk_73 = r.chunk(0x03043049, |r| {
                if r.u32()? != 2 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                let _intro_clip = r.node_ref::<Arc<MediaClip>>()?;
                let _podium_clip = r.node_ref::<Arc<MediaClip>>()?;
                let _in_game_clip_group = r.node_ref::<Arc<MediaClipGroup>>()?;
                let _end_race_clip_group = r.node_ref::<Arc<MediaClipGroup>>()?;
                let _ambiance_clip = r.node_ref::<Arc<MediaClip>>()?;
                let _clip_trigger_size = r.vec3_u32()?;

                Ok(Chunk73)
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

                let _title_id = r.string_ref()?;
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
                    let _embedded_item_models = r.list(|r| {
                        let _id = r.string_ref()?;
                        let _collection = r.string_ref()?;
                        let _author = r.string_ref()?;

                        Ok(())
                    })?;
                    let _embedded_zip_data = r.list_u8()?;
                    let _textures = r.list(|r| r.string())?;

                    Ok(())
                })?;

                Ok(Chunk84)
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
                if r.u32()? != 4 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                r.u32()?;

                Ok(Chunk87)
            })?;
            let chunk_88 = r.chunk_skippable(0x03043058, |r| {
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
                    if r.u32()? != 8 {
                        return Err(Error::Internal("unknown lightmaps version".into()));
                    }

                    r.list(|r| {
                        r.list_u8()?;
                        r.list_u8()?;
                        r.list_u8()?;

                        Ok(())
                    })?;

                    let _lightmap_cache_data_size = r.u32()?;
                    let _compressed_lightmap_cache_data = r.list_u8()?;
                }

                Ok(Chunk91)
            })?;
            let chunk_92 = r.chunk_skippable(0x0304305c, |r| {
                if r.u32()? != 0 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                r.u32()?;
                r.u32()?;

                Ok(Chunk92)
            })?;
            let chunk_93 = r.chunk_skippable(0x0304305d, |r| {
                if r.u32()? != 0 {
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

                for block in &chunk_31.blocks {
                    if block.is_free {
                        let _absolute_position_in_map = r.vec3_f32()?;
                        let _yaw_pitch_roll = r.yaw_pitch_roll()?;
                    }
                }

                for block in &chunk_72.baked_blocks {
                    if block.is_free {
                        let _absolute_position_in_map = r.vec3_f32()?;
                        let _yaw_pitch_roll = r.yaw_pitch_roll()?;
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

            Ok(Self {
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
