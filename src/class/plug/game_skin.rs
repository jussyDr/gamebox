//! Game skin.

use crate::ClassId;

/// Game skin.
#[derive(Default)]
pub struct GameSkin;

impl ClassId for GameSkin {
    const CLASS_ID: u32 = 0x090f4000;
}

mod read {
    use crate::{
        class::plug::game_skin::GameSkin,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for GameSkin {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for GameSkin {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(3, Self::read_chunk_3),
                BodyChunk::skippable(5, Self::read_chunk_5),
            ]
        }
    }

    impl GameSkin {
        fn read_chunk_3(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _painted_texture_name = r.string()?;
            let _painer_scene_id = r.string()?;

            Ok(())
        }

        fn read_chunk_5(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 8 {
                return Err(error_unknown_chunk_version(version as u32));
            }

            let _dir_name = r.string()?;
            let _painted_texture_name = r.string()?;
            let _painer_scene_id = r.string()?;
            let num_fids = r.u8()?;
            let _fids = r.repeat(num_fids as usize, |r| {
                let _class_id = r.u32()?;
                let _name = r.string()?;
                r.string()?;
                r.bool32()?;

                Ok(())
            })?;
            let _dir_name_alt = r.string()?;
            r.bool32()?;
            r.string()?;
            r.u32()?;
            r.u8()?;

            Ok(())
        }
    }
}
