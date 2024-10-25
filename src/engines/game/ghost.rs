use std::io::Read;

use crate::{
    engines::plug::EntRecordData,
    read::{
        readable::{BodyChunk, BodyChunks},
        Error, IdStateMut, NodeStateMut, Reader,
    },
};

#[derive(Default)]
pub struct GhostInner;

impl BodyChunks for GhostInner {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (6, |n, r| Self::read_chunk_6(n, r), false),
            (7, |n, r| Self::read_chunk_7(n, r), true),
        ];

        chunks.into_iter()
    }
}

impl GhostInner {
    fn read_chunk_6<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _is_replaying = r.bool()?;
        let _uncompressed_size = r.u32()?;
        let _data = r.byte_buf()?;

        Ok(())
    }

    fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;

        Ok(())
    }
}

/// A ghost.
#[derive(Default)]
pub struct Ghost {
    inner: GhostInner,
}

impl BodyChunks for Ghost {
    type Parent = GhostInner;

    fn parent(&mut self) -> Option<&mut GhostInner> {
        Some(&mut self.inner)
    }

    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 24] = [
            (0, |n, r| Self::read_chunk_0(n, r), true),
            (5, |n, r| Self::read_chunk_5(n, r), true),
            (8, |n, r| Self::read_chunk_8(n, r), true),
            (10, |n, r| Self::read_chunk_10(n, r), true),
            (11, |n, r| Self::read_chunk_11(n, r), true),
            (12, |n, r| Self::read_chunk_12(n, r), false),
            (14, |n, r| Self::read_chunk_14(n, r), false),
            (15, |n, r| Self::read_chunk_15(n, r), false),
            (16, |n, r| Self::read_chunk_16(n, r), false),
            (19, |n, r| Self::read_chunk_19(n, r), true),
            (20, |n, r| Self::read_chunk_20(n, r), true),
            (26, |n, r| Self::read_chunk_26(n, r), true),
            (27, |n, r| Self::read_chunk_27(n, r), true),
            (28, |n, r| Self::read_chunk_28(n, r), false),
            (29, |n, r| Self::read_chunk_29(n, r), true),
            (34, |n, r| Self::read_chunk_34(n, r), true),
            (35, |n, r| Self::read_chunk_35(n, r), true),
            (36, |n, r| Self::read_chunk_36(n, r), true),
            (37, |n, r| Self::read_chunk_37(n, r), true),
            (38, |n, r| Self::read_chunk_38(n, r), true),
            (39, |n, r| Self::read_chunk_39(n, r), true),
            (40, |n, r| Self::read_chunk_40(n, r), true),
            (41, |n, r| Self::read_chunk_41(n, r), true),
            (42, |n, r| Self::read_chunk_42(n, r), true),
        ];

        chunks.into_iter()
    }
}

impl Ghost {
    fn read_chunk_0(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 7 {
            return Err(Error);
        }

        let _player_model = r.ident()?;
        let _light_trail_color = r.vec3::<f32>()?;
        let _skin_pack_descs = r.list(|r| r.pack_desc())?;
        let has_badges = r.bool()?;

        if has_badges {
            todo!()
        }

        let _ghost_nickname = r.string()?;
        let _ghost_avatar_name = r.string()?;
        let _recording_context = r.string()?;
        r.bool()?;
        let _record_data = r.node::<EntRecordData>()?;
        r.list(|r| r.u32())?;
        let _ghost_trigram = r.string()?;
        let _ghost_zone = r.string()?;

        Ok(())
    }

    fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _race_time = r.u32()?;

        Ok(())
    }

    fn read_chunk_8<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _respawns = r.u32()?;

        Ok(())
    }

    fn read_chunk_10<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _stunt_score = r.u32()?;

        Ok(())
    }

    fn read_chunk_11<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _checkpoints = r.list(|r| {
            let _time = r.u32()?;
            let _stunts_score = r.u32()?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_12<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;

        Ok(())
    }

    fn read_chunk_14<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;

        Ok(())
    }

    fn read_chunk_15<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _ghost_login = r.string()?;

        Ok(())
    }

    fn read_chunk_16<N>(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, N>,
    ) -> Result<(), Error> {
        let _validate_challenge_uid = r.id()?;

        Ok(())
    }

    fn read_chunk_19<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        r.i32()?;

        Ok(())
    }

    fn read_chunk_20<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;

        Ok(())
    }

    fn read_chunk_26<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;

        Ok(())
    }

    fn read_chunk_27<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        r.list(|r| r.u32())?;
        r.u16()?;
        r.u32()?;
        r.u32()?;

        Ok(())
    }

    fn read_chunk_28<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.byte_array::<32>()?;

        Ok(())
    }

    fn read_chunk_29<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 4 {
            return Err(Error);
        }

        let _player_inputs = r.list(|r| {
            let version = r.u32()?;

            if version != 12 {
                return Err(Error);
            }

            r.u32()?;
            let _start_offset = r.f32()?;
            let _ticks = r.u32()?;
            let _data = r.byte_buf()?;

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_34<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        r.u32()?;

        Ok(())
    }

    fn read_chunk_35<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 3 {
            return Err(Error);
        }

        r.string()?;
        r.u32()?;
        r.string()?;
        r.u32()?;
        r.u32()?;
        r.string()?;
        r.u32()?;
        r.string()?;
        r.u8()?;
        r.u32()?;
        r.u32()?;
        r.u8()?;
        r.u8()?;

        Ok(())
    }

    fn read_chunk_36<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        r.u32()?;

        Ok(())
    }

    fn read_chunk_37<N>(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, N>,
    ) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 1 {
            return Err(Error);
        }

        let _events_duration = r.u32()?;
        r.u32()?;
        let _input_names = r.list(|r| r.id())?;
        let num_entries = r.u32()?;
        r.u32()?;
        r.repeat(num_entries as usize, |r| {
            let _time = r.u32()?;
            let _input_name_index = r.u8()?;
            let _data = r.u32()?;

            Ok(())
        })?;
        let _exe_version = r.string()?;
        let _checksum = r.u32()?;
        let _os_kind = r.u32()?;
        let _cpu_kind = r.u32()?;
        let _race_settings = r.string()?;
        r.u32()?;
        let _steering_wheel_sensitivity = r.bool()?;

        Ok(())
    }

    fn read_chunk_38<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u128()?;

        Ok(())
    }

    fn read_chunk_39<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;

        Ok(())
    }

    fn read_chunk_40<I, N>(&mut self, _: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        Ok(())
    }

    fn read_chunk_41<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;

        Ok(())
    }

    fn read_chunk_42<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        r.u32()?;

        Ok(())
    }
}
