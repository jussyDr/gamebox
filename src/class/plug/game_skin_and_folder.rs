//! Game skin and folder.

use crate::{ClassId, SubExtensions};

/// Game skin and folder.
#[derive(Default)]
pub struct GameSkinAndFolder;

impl ClassId for GameSkinAndFolder {
    const CLASS_ID: u32 = 0x0915d000;
}

impl SubExtensions for GameSkinAndFolder {
    const SUB_EXTENSIONS: &[&str] = &[
        "Gbx",
        "TerrainModifier",
        "TerrainModifier ", // Nice nadeo typo.
    ];
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::{game_skin::GameSkin, game_skin_and_folder::GameSkinAndFolder},
        read::{BodyChunk, BodyChunks, BodyReader, Error, ReadBody, read_body_chunks},
    };

    impl ReadBody for GameSkinAndFolder {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for GameSkinAndFolder {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(0, Self::read_chunk_0),
                BodyChunk::new(1, Self::read_chunk_1),
            ]
        }
    }

    impl GameSkinAndFolder {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _game_skin: Arc<GameSkin> = r.node_ref()?;
            let _folder = r.string()?;

            Ok(())
        }

        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }
    }
}
