//! Media block entity.

use crate::ClassId;

/// Media block entity.
#[derive(Default)]
pub struct MediaBlockEntity;

impl ClassId for MediaBlockEntity {
    const CLASS_ID: u32 = 0x0329f000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::{
            game::ctn::{media_block_entity::MediaBlockEntity, read_file_ref},
            plug::ent_record_data::EntRecordData,
        },
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for MediaBlockEntity {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockEntity {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl MediaBlockEntity {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(error_unknown_chunk_version(version));
            }

            let _record_data = r.internal_node_ref::<EntRecordData>()?;
            let _start_offset = r.f32()?;
            let _notice_records = r.list(|r| r.u32())?;
            let _no_damage = r.bool32()?;
            r.bool32()?;
            let _force_light = r.u32()?;
            let _force_hue = r.bool32()?;
            let _player_model: Vec<Option<Arc<str>>> = r.repeat(3, |r| r.id())?;
            r.vec3()?;
            let _skin_names = r.list(|r| read_file_ref(r))?;

            if r.bool32()? {
                todo!()
            }

            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _lights = r.u32()?;
                r.f32()?;
                r.u32()?;
                r.u32()?;
                let _trail_intensity = r.f32()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
