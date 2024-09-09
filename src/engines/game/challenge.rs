//! Challenge class.

use std::io::Read;

use crate::{
    engines::game::collector_list::CollectorList,
    read::{
        readable::{self, BodyChunk, BodyChunks, UserDataChunk, UserDataChunks},
        reader::{IdStateMut, NodeStateMut, Reader},
        Readable,
    },
    Error,
};

/// Challenge class.
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
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (13, |n, r| Self::read_chunk_13(n, r)),
            (17, |n, r| Self::read_chunk_17(n, r)),
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

        todo!()
    }
}
