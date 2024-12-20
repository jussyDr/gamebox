//! Media block entity.

use crate::Class;

/// Entity media block.
#[derive(Default)]
pub struct MediaBlockEntity {
    keys: Vec<Key>,
}

impl Class for MediaBlockEntity {
    const CLASS_ID: u32 = 0x0329f000;
}

impl MediaBlockEntity {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Entity media block key.
pub struct Key {
    time: f32,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time
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

    use super::{Key, MediaBlockEntity};

    impl ReadBody for MediaBlockEntity {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockEntity {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::normal(2, Self::read_chunk_2),
            ]
            .into_iter()
        }
    }

    impl MediaBlockEntity {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 6 | 9 | 11) {
                return Err(Error::chunk_version(version));
            }

            let _record_data = r.internal_node_ref::<EntRecordData>()?;
            let _start_offset = r.f32()?;
            let _notice_records = r.list(|r| r.u32())?;
            let _no_damage = r.bool()?;
            r.bool()?;
            let _force_light = r.bool()?;
            let _force_hue = r.bool()?;

            if version >= 11 {
                r.u32()?;
            }

            let _player_model_id = r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.vec3::<f32>()?;
            let _skin_names = r.list(|r| r.pack_desc())?;
            let _has_badges = r.bool()?;

            if version >= 11 {
                let _skin_options = r.string()?;
            }

            self.keys = r.list(|r| {
                let time = r.f32()?;
                let _lights = r.u32()?;
                r.f32()?;
                r.u32()?;
                r.u32()?;
                let _trail_intensity = r.f32()?;

                if version >= 9 {
                    let _self_illum_intensity = r.f32()?;
                }

                Ok(Key { time })
            })?;

            if version >= 7 {
                let _ghost_name = r.string()?;
            }

            if version >= 8 {
                r.u32()?;
            }

            if version >= 11 {
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _skin_options = r.string()?;

            Ok(())
        }
    }
}
