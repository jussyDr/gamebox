use std::io::Read;

use crate::{
    engines::{
        game::{
            block::Block, challenge_parameters::ChallengeParameters, collector_list::CollectorList,
        },
        game_data::WaypointSpecialProperty,
        scene::VehicleCarMarksSamples,
        script::TraitsMetadata,
    },
    read::{
        readable::{self, BodyChunk, BodyChunks, UserDataChunk, UserDataChunks},
        IdState, IdStateMut, NodeStateMut, Readable, Reader,
    },
    write::{writable, Writable},
    Error,
};

use super::{AnchoredObject, MediaClip, MediaClipGroup, ZoneGenealogy};

enum MapElemColor {
    Default,
    White,
    Green,
    Blue,
    Red,
    Black,
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

enum LightmapQuality {
    Normal,
    High,
    VeryHigh,
    Highest,
    Low,
    VeryLow,
    Lowest,
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

/// A challenge.
#[derive(Default)]
pub struct Challenge {
    blocks: Box<[Block]>,
    baked_blocks: Box<[Block]>,
    anchored_objects: Box<[Option<AnchoredObject>]>,
}

impl Readable for Challenge {}

impl readable::Sealed for Challenge {}

impl Writable for Challenge {}

impl writable::Sealed for Challenge {
    const CLASS_ID: u32 = 0x03043000;
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
        let _bronze_time = r.u32()?;
        let _silver_time = r.u32()?;
        let _gold_time = r.u32()?;
        let _author_time = r.u32()?;
        let _cost = r.u32()?;
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
            return Err(Error);
        }

        let _map_info = r.ident()?;
        let _map_name = r.string()?;
        let _kind_in_header = r.u8()?;
        r.u32()?;
        let _password = r.string()?;
        let _decoration = r.ident()?;
        read_map_origin_and_target(r)?;
        let _pack_mask = r.u128()?;
        let _map_type = r.string()?;
        let _map_style = r.string()?;
        let _lightmap_cache_uid = r.u64()?;
        let _lightmap_version = r.u8()?;
        let _title_id = r.id()?;

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
        let _xml = r.string()?;

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

        let _thumbnail = r.bytes(thumbnail_len as usize)?;

        if &r.byte_array()? != b"</Thumbnail.jpg>" {
            return Err(Error);
        }

        if &r.byte_array()? != b"<Comments>" {
            return Err(Error);
        }

        let _comments = r.string()?;

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

        read_author(r)?;

        Ok(())
    }

    fn read_chunk_13<N>(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, N>,
    ) -> Result<(), Error> {
        let _player_model = r.ident()?;

        Ok(())
    }

    fn read_chunk_17(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let _block_stock = r.node::<CollectorList>()?;
        let _challenge_parameter = r.node::<ChallengeParameters>()?;
        let _map_kind = r.u32()?;

        Ok(())
    }

    fn read_chunk_24<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _is_laps_race = r.bool()?;
        let _num_laps = r.u32()?;

        Ok(())
    }

    fn read_chunk_25<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _mod_pack_desc = r.file_ref()?;

        Ok(())
    }

    fn read_chunk_31(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let _map_info = r.ident()?;
        let _map_name = r.string()?;
        let _decoration = r.ident()?;
        let _size = r.vec3::<u32>()?;
        let _need_unlock = r.bool()?;
        self.blocks = read_blocks(r)?;

        Ok(())
    }

    fn read_chunk_34<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;

        Ok(())
    }

    fn read_chunk_36<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _custom_music_pack_desc = r.file_ref()?;

        Ok(())
    }

    fn read_chunk_37<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        read_map_origin_and_target(r)?;

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
        r.u32()?;

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

    fn read_chunk_62(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error);
        }

        let _car_marks_buffer = r.versioned_list(|r| r.node::<VehicleCarMarksSamples>())?;

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
            let _items_on_item = r.list(|r| r.vec2::<u32>())?;
            let _block_indexes = r.list(|r| r.u32())?;
            let _item_indexes = r.list(|r| r.u32())?;
            let _snap_item_groups = r.list(|r| r.u32())?;
            r.list(|r| r.u32())?;
            let _snapped_indexes = r.list(|r| r.u32())?;
        }

        Ok(())
    }

    fn read_chunk_66<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 1 {
            return Err(Error);
        }

        read_author(r)?;

        Ok(())
    }

    fn read_chunk_67<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        let len = r.u32()?;

        {
            let mut r = r.take_with(len as u64, IdState::new(), ());

            let _zone_genealogy = r.list(|r| r.node_inline::<ZoneGenealogy>())?;
        }

        Ok(())
    }

    fn read_chunk_68<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        let len = r.u32()?;

        {
            let mut r = r.take_with(len as u64, IdState::new(), ());

            let _script_metadata = r.node_inline_v2::<TraitsMetadata>()?;
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

        let _clip_intro = r.node::<MediaClip>()?;
        let _clip_podium = r.node::<MediaClip>()?;
        let _clip_group_in_game = r.node::<MediaClipGroup>()?;
        let _clip_group_end_race = r.node::<MediaClipGroup>()?;
        let _clip_ambiance = r.node::<MediaClip>()?;
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

        let _offzone_trigger_size = r.vec3::<u32>()?;
        let _offzones = r.list(|r| r.box3::<u32>())?;

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

        let _title_id = r.id()?;
        let _build_version = r.string()?;

        Ok(())
    }

    fn read_chunk_82<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error);
        }

        let _deco_base_height_offset = r.u32()?;

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

            let _embedded_item_models = r.list(|r| r.ident())?;
            let _embedded_zip_data = r.byte_buf()?;
            let _textures = r.list(|r| r.string())?;
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

        read_light_settings(r)?;

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

        let _world_distortion = r.vec3::<f32>()?;
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
                let _foreground_pack_desc = r.file_ref()?;
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
        read_light_settings(r)?;

        Ok(())
    }
}

fn read_map_origin_and_target<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
    let _map_coord_origin = r.vec2::<f32>()?;
    let _map_coord_target = r.vec2::<f32>()?;

    Ok(())
}

fn read_author<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
    let author_version = r.u32()?;

    if author_version != 0 {
        return Err(Error);
    }

    let _author_login = r.string()?;
    let _author_nickname = r.string()?;
    let _author_zone = r.string()?;
    let _author_extra_info = r.string()?;

    Ok(())
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

fn read_light_settings<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
    r.u32()?;
    let _day_time = r.f32()?;
    r.u32()?;
    let _dynamic_daylight = r.bool()?;
    let _day_duration = r.u32()?;

    Ok(())
}
