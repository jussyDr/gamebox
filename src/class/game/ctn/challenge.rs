use std::{any::Any, cell::OnceCell, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    game::ctn::{
        AnchoredObject, Block, ChallengeParameters, CollectorList, FileRef, MediaClip,
        MediaClipGroup, ZoneGenealogy,
    },
    read::{BodyChunksReader, BodyReader, ClassId, Error, Readable},
    script::TraitsMetadata,
};

pub struct Challenge(Inner);

#[self_referencing]
struct Inner {
    header_data: Box<[u8]>,
    body_data: Arc<[u8]>,
    node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    #[borrows(header_data, body_data, node_refs)]
    #[covariant]
    chunks: Chunks<'this>,
}

struct Chunks<'a> {
    chunk_2: Chunk2,
    chunk_3: Chunk3,
    chunk_4: Chunk4,
    chunk_5: Chunk5,
    chunk_7: Chunk7,
    chunk_8: Chunk8,
    chunk_13: Chunk13,
    chunk_17: Chunk17<'a>,
    chunk_24: Chunk24,
    chunk_25: Chunk25<'a>,
    chunk_31: Chunk31<'a>,
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
    chunk_72: Chunk72<'a>,
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

struct Chunk2;

struct Chunk3;

struct Chunk4;

struct Chunk5;

struct Chunk7;

struct Chunk8;

struct Chunk13;

struct Chunk17<'a> {
    parameters: &'a ChallengeParameters,
}

struct Chunk24;

struct Chunk25<'a> {
    texture_mod: Option<FileRef<'a>>,
}

struct Chunk31<'a> {
    blocks: Box<[Block<'a>]>,
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

struct Chunk64 {
    anchored_objects: Box<[AnchoredObject]>,
}

struct Chunk66;

struct Chunk67;

struct Chunk68;

struct Chunk72<'a> {
    baked_blocks: Box<[Block<'a>]>,
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

impl Challenge {
    pub fn parameters(&self) -> &ChallengeParameters {
        self.0.borrow_chunks().chunk_17.parameters
    }

    pub fn texture_mod(&self) -> Option<&FileRef> {
        self.0.borrow_chunks().chunk_25.texture_mod.as_ref()
    }

    pub fn blocks(&self) -> &[Block] {
        &self.0.borrow_chunks().chunk_31.blocks
    }

    pub fn anchored_objects(&self) -> &[AnchoredObject] {
        &self.0.borrow_chunks().chunk_64.anchored_objects
    }
}

impl ClassId for Challenge {
    const CLASS_ID: u32 = 0x03043000;
}

impl Readable for Challenge {
    fn read_from_header_and_body(
        header_data: Box<[u8]>,
        body_data: Arc<[u8]>,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    ) -> Result<Self, Error> {
        let builder = InnerTryBuilder {
            header_data,
            body_data,
            node_refs,
            chunks_builder: |_header_data, body_data, node_refs| {
                let mut body_data_offset = 0;
                let mut seen_id = false;
                let mut ids = vec![];

                let mut br = BodyReader::new(
                    body_data,
                    &mut body_data_offset,
                    node_refs,
                    &mut seen_id,
                    &mut ids,
                );
                let mut r = BodyChunksReader(&mut br);

                let chunk_13 = r.chunk(0x0304300d, |r| {
                    let _player_model = r.id_or_null()?;
                    let _player_model_collection = r.id_or_null()?;
                    let _player_model_author = r.id_or_null()?;

                    Ok(Chunk13)
                })?;

                let chunk_17 = r.chunk(0x03043011, |r| {
                    let _block_stock = r.node_ref::<CollectorList>()?;
                    let parameters = r.node_ref::<ChallengeParameters>()?;
                    let _kind = match r.u32()? {
                        6 => MapKind::InProgress,
                        index => {
                            return Err(Error::new(format!(
                                "unknown map kind enum variant index: {index}"
                            )));
                        }
                    };

                    Ok(Chunk17 { parameters })
                })?;

                let chunk_24 = r.skippable_chunk(0x03043018, |r| {
                    let _is_lap_race = r.bool32()?;
                    let _num_laps = r.u32()?;

                    Ok(Chunk24)
                })?;

                let chunk_25 = r.skippable_chunk(0x03043019, |r| {
                    let texture_mod = FileRef::read(r)?;

                    Ok(Chunk25 { texture_mod })
                })?;

                let chunk_31 = r.chunk(0x0304301f, |r| {
                    let _map_id = r.id()?;
                    let _map_collection = r.id()?;
                    let _map_author = r.id()?;
                    let _map_name = r.string()?;
                    let _decoration_id = r.id()?;
                    let _decoration_collection = r.id()?;
                    let _decoration_author = r.id()?;
                    let _size = r.vec3_u32()?;
                    let _need_unlock = r.bool32()?;
                    let blocks_version = r.u32()?;

                    if blocks_version != 6 {
                        return Err(Error::new(format!(
                            "unknown blocks version: {blocks_version}"
                        )));
                    }

                    let blocks = r.list(|r| Block::read(r))?;

                    Ok(Chunk31 { blocks })
                })?;

                let chunk_34 = r.chunk(0x03043022, |r| {
                    r.u32()?;

                    Ok(Chunk34)
                })?;

                let chunk_36 = r.chunk(0x03043024, |r| {
                    let _music_file_ref = FileRef::read(r)?;

                    Ok(Chunk36)
                })?;

                let chunk_37 = r.chunk(0x03043025, |r| {
                    let _map_coord_origin = r.vec2_f32()?;
                    let _map_coord_target = r.vec2_f32()?;

                    Ok(Chunk37)
                })?;

                let chunk_38 = r.chunk(0x03043026, |r| {
                    r.u32()?;

                    Ok(Chunk38)
                })?;

                let chunk_40 = r.chunk(0x03043028, |r| {
                    if r.bool32()? {
                        todo!();
                    }

                    let _comments = r.string()?;

                    Ok(Chunk40)
                })?;

                let chunk_41 = r.skippable_chunk(0x03043029, |r| {
                    let _password_hash = r.u128()?;
                    let _crc32 = r.u32()?;

                    Ok(Chunk41)
                })?;

                let chunk_42 = r.chunk(0x0304302a, |r| {
                    let _created_with_simple_editor = r.bool32()?;

                    Ok(Chunk42)
                })?;

                let chunk_52 = r.skippable_chunk(0x03043034, |r| {
                    r.list_u8()?;

                    Ok(Chunk52)
                })?;

                let chunk_54 = r.skippable_chunk(0x03043036, |r| {
                    let _thumbnail_position = r.vec3_f32()?;
                    let _thumbnail_pitch_yaw_roll = r.vec3_f32()?;
                    let _thumbnail_fov = r.f32()?;
                    r.f32()?;
                    r.f32()?;
                    let _thumbnail_near_clip_plane = r.f32()?;
                    let _thumbnail_far_clip_plane = r.f32()?;
                    let _comments = r.string()?;

                    Ok(Chunk54)
                })?;

                let chunk_56 = r.skippable_chunk(0x03043038, |r| {
                    r.u32()?;

                    Ok(Chunk56)
                })?;

                let chunk_62 = r.skippable_chunk(0x0304303e, |r| {
                    let version = r.u32()?;

                    if version != 0 {
                        return Err(Error::new(format!("unknown chunk version: {version}")));
                    }

                    let _car_marks_buffer: Box<[()]> = r.list_with_version(|_| todo!())?;

                    Ok(Chunk62)
                })?;

                let chunk_64 = r.skippable_chunk(0x03043040, |r| {
                    let version = r.u32()?;

                    if version != 5 {
                        return Err(Error::new(format!("unknown chunk version: {version}")));
                    }

                    let encapsulation_version = r.u32()?;

                    if encapsulation_version != 0 {
                        return Err(Error::new(format!(
                            "unknown encapsulation version: {encapsulation_version}"
                        )));
                    }

                    let _size = r.u32()?;

                    let mut seen_id = false;
                    let mut ids = vec![];
                    let node_refs = Arc::from(vec![]);
                    let mut r = BodyReader::new(
                        body_data,
                        r.data_offset,
                        &node_refs,
                        &mut seen_id,
                        &mut ids,
                    );

                    let anchored_objects = r.list_with_version(|r| r.node::<AnchoredObject>())?;
                    let _block_indices = r.list(|r| r.u32())?;
                    let _snap_item_groups = r.list(|r| r.u32())?;
                    r.list(|r| r.u32())?;
                    let _snapped_indices = r.list(|r| r.u32())?;

                    Ok(Chunk64 { anchored_objects })
                })?;

                let chunk_66 = r.skippable_chunk(0x03043042, |r| {
                    let version = r.u32()?;

                    if version != 1 {
                        return Err(Error::new(format!("unknown chunk version: {version}")));
                    }

                    let author_version = r.u32()?;

                    if author_version != 0 {
                        return Err(Error::new(format!(
                            "unknown author version: {author_version}"
                        )));
                    }

                    let _author_login = r.string()?;
                    let _author_nickname = r.string()?;
                    let _author_zone = r.string()?;
                    let _author_extra_info = r.string()?;

                    Ok(Chunk66)
                })?;

                let chunk_67 = r.skippable_chunk(0x03043043, |r| {
                    let encapsulation_version = r.u32()?;

                    if encapsulation_version != 0 {
                        return Err(Error::new(format!(
                            "unknown encapsulation version: {encapsulation_version}"
                        )));
                    }

                    let _size = r.u32()?;

                    let mut seen_id = false;
                    let mut ids = vec![];
                    let node_refs = Arc::from(vec![]);
                    let mut r = BodyReader::new(
                        body_data,
                        r.data_offset,
                        &node_refs,
                        &mut seen_id,
                        &mut ids,
                    );

                    let _zone_genealogy = r.list(|r| r.node::<ZoneGenealogy>())?;

                    Ok(Chunk67)
                })?;

                let chunk_68 = r.skippable_chunk(0x03043044, |r| {
                    let encapsulation_version = r.u32()?;

                    if encapsulation_version != 0 {
                        return Err(Error::new(format!(
                            "unknown encapsulation version: {encapsulation_version}"
                        )));
                    }

                    let _size = r.u32()?;

                    let mut seen_id = false;
                    let mut ids = vec![];
                    let node_refs = Arc::from(vec![]);
                    let mut r = BodyReader::new(
                        body_data,
                        r.data_offset,
                        &node_refs,
                        &mut seen_id,
                        &mut ids,
                    );

                    let _script_metadata = TraitsMetadata::read(&mut r)?;

                    Ok(Chunk68)
                })?;

                let chunk_72 = r.skippable_chunk(0x03043048, |r| {
                    let version = r.u32()?;

                    if version != 0 {
                        return Err(Error::new(format!("unknown chunk version: {version}")));
                    }

                    let blocks_version = r.u32()?;

                    if blocks_version != 6 {
                        return Err(Error::new(format!(
                            "unknown blocks version: {blocks_version}"
                        )));
                    }

                    let baked_blocks = r.list(|r| Block::read(r))?;
                    r.u32()?;
                    let _baked_clips_additional_data = r.u32()?;

                    Ok(Chunk72 { baked_blocks })
                })?;

                let chunk_73 = r.chunk(0x03043049, |r| {
                    let version = r.u32()?;

                    if version != 2 {
                        return Err(Error::new(format!("unknown chunk version: {version}")));
                    }

                    let _intro_clip = r.node_ref::<MediaClip>()?;
                    let _podium_clip = r.node_ref_or_null::<MediaClip>()?;
                    let _in_game_clips = r.node_ref::<MediaClipGroup>()?;
                    let _end_race_clips = r.node_ref_or_null::<MediaClipGroup>()?;
                    let _ambiance_clip = r.node_ref::<MediaClip>()?;
                    let _clip_trigger_size = r.vec3_u32()?;

                    Ok(Chunk73)
                })?;

                r.end()?;

                Ok(Chunks {
                    chunk_2: Chunk2,
                    chunk_3: Chunk3,
                    chunk_4: Chunk4,
                    chunk_5: Chunk5,
                    chunk_7: Chunk7,
                    chunk_8: Chunk8,
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
                })
            },
        };

        builder.try_build().map(Self)
    }
}
