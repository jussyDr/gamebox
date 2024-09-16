use std::io::Read;

use crate::{
    engines::{
        game::{
            block::Block, challenge_parameters::ChallengeParameters, collector_list::CollectorList,
        },
        scene::VehicleCarMarksSamples,
        script::TraitsMetadata,
    },
    read::{
        readable::{self, BodyChunk, BodyChunks, UserDataChunk, UserDataChunks},
        IdState, IdStateMut, IdStateRef, NodeStateMut, NodeStateRef, Readable, Reader,
    },
    Error,
};

use super::{AnchoredObject, ZoneGenealogy};

/// A challenge.
#[derive(Default)]
pub struct Challenge;

impl Readable for Challenge {}

impl readable::Sealed for Challenge {}

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
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 18] = [
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
        let _map_coord_origin = r.vec2::<f32>()?;
        let _map_coord_target = r.vec2::<f32>()?;
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
        let version = r.u32()?;

        if version != 6 {
            return Err(Error);
        }

        let _blocks = r.list(|r| Block::read(r))?;

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
        let _map_coord_origin = r.vec2::<f32>()?;
        let _map_coord_target = r.vec2::<f32>()?;

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

        let list_version = r.u32()?;

        if list_version != 10 {
            return Err(Error);
        }

        let _car_marks_buffer = r.list(|r| r.node::<VehicleCarMarksSamples>())?;

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

            let list_version = r.u32()?;

            if list_version != 10 {
                return Err(Error);
            }

            let _anchored_objects = r.list(|r| r.node_inline::<AnchoredObject>())?;
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
        r: &mut Reader<impl Read, impl IdStateRef, impl NodeStateRef>,
    ) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error);
        }

        let blocks_version = r.u32()?;

        if blocks_version != 6 {
            return Err(Error);
        }

        r.list(|r| Block::read_inline(r))?;
        r.u32()?;

        Ok(())
    }
}
