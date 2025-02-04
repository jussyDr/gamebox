//! Ghost.

use std::ops::Deref;

use crate::{game::ghost, Class};

/// Ghost.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct Ghost {
    parent: ghost::Ghost,
    events_duration: u32,
}

impl Class for Ghost {
    const CLASS_ID: u32 = 0x03092000;
}

impl Deref for Ghost {
    type Target = ghost::Ghost;

    fn deref(&self) -> &ghost::Ghost {
        &self.parent
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::ent_record_data::EntRecordData,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::Ghost;

    impl ReadBody for Ghost {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Ghost {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            #![allow(clippy::redundant_closure)]
            [
                BodyChunk::skippable(0, |s, r| Self::read_chunk_0(s, r)),
                BodyChunk::skippable(5, |s, r| Self::read_chunk_5(s, r)),
                BodyChunk::skippable(8, |s, r| Self::read_chunk_8(s, r)),
                BodyChunk::skippable(10, |s, r| Self::read_chunk_10(s, r)),
                BodyChunk::skippable(11, |s, r| Self::read_chunk_11(s, r)),
                BodyChunk::normal(12, Self::read_chunk_12),
                BodyChunk::normal(14, Self::read_chunk_14),
                BodyChunk::normal(15, Self::read_chunk_15),
                BodyChunk::normal(16, Self::read_chunk_16),
                BodyChunk::skippable(19, |s, r| Self::read_chunk_19(s, r)),
                BodyChunk::skippable(20, |s, r| Self::read_chunk_20(s, r)),
                BodyChunk::skippable(26, |s, r| Self::read_chunk_26(s, r)),
                BodyChunk::skippable(27, |s, r| Self::read_chunk_27(s, r)),
                BodyChunk::normal(28, Self::read_chunk_28),
                BodyChunk::skippable(29, |s, r| Self::read_chunk_29(s, r)),
                BodyChunk::skippable(34, |s, r| Self::read_chunk_34(s, r)),
                BodyChunk::skippable(35, |s, r| Self::read_chunk_35(s, r)),
                BodyChunk::skippable(36, |s, r| Self::read_chunk_36(s, r)),
                BodyChunk::skippable(37, |s, r| Self::read_chunk_37(s, r)),
                BodyChunk::skippable(38, |s, r| Self::read_chunk_38(s, r)),
                BodyChunk::skippable(39, |s, r| Self::read_chunk_39(s, r)),
                BodyChunk::skippable(40, |s, r| Self::read_chunk_40(s, r)),
                BodyChunk::skippable(41, |s, r| Self::read_chunk_41(s, r)),
                BodyChunk::skippable(42, |s, r| Self::read_chunk_42(s, r)),
            ]
            .into_iter()
        }
    }

    impl Ghost {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 7 {
                return Err(Error::chunk_version(version));
            }

            let _player_model_id = r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            let _light_trail_color = r.vec3()?;
            let _skin_pack_descs = r.list(|r| r.file_ref())?;
            let has_badges = r.bool()?;

            if has_badges {
                let _badge_version = r.u32()?;
                let _color = r.rgb_float()?;
                let _stickers = r.list(|r| {
                    r.string()?;
                    r.string()?;

                    Ok(())
                })?;
                let _layers = r.list(|r| r.string())?;
            }

            let _ghost_nickname = r.string()?;
            let _ghost_avatar_name = r.string()?;
            let _recording_context = r.string()?;
            r.bool()?;
            let _record_data = r.internal_node_ref::<EntRecordData>()?;
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
            r.id()?;

            Ok(())
        }

        fn read_chunk_19<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

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
            r.u32()?;
            r.u16()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_28<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_29<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(Error::chunk_version(version));
            }

            let _player_inputs = r.list(|r| {
                let version = r.u32()?;

                if version != 12 {
                    return Err(Error::version("player input data", version));
                }

                r.u32()?;
                let _start_offset = r.u32()?;
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
                return Err(Error::chunk_version(version));
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
                return Err(Error::chunk_version(version));
            }

            self.events_duration = r.u32()?;
            r.u32()?;
            let _input_names = r.list(|r| r.id())?;
            let num_entries = r.u32()?;
            r.u32()?;
            let _inputs = r.repeat(num_entries as usize, |r| {
                let _time = r.u32()?;
                let _input_name_index = r.u8()?;
                let _data = r.u32()?;

                Ok(())
            })?;
            let _validate_exe_version = r.string()?;
            let _validate_exe_checksum = r.u32()?;
            let _validate_os_kind = r.u32()?;
            let _validate_cpu_kind = r.u32()?;
            let _validate_race_settings = r.string()?;
            r.u32()?;
            let _steering_wheel_sensitivity = r.f32()?;

            Ok(())
        }

        fn read_chunk_38<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_39<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_40<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            if self.events_duration != 0 {
                let _validate_title_id = r.string()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
            }

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
}

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::Ghost;

    impl WriteBody for Ghost {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for Ghost {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
