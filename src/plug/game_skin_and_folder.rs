//! Game skin and folder.

use crate::Class;

/// Game skin and folder.
#[derive(Default)]
pub struct GameSkinAndFolder;

impl Class for GameSkinAndFolder {
    const CLASS_ID: u32 = 0x0915d000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::GameSkin,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::GameSkinAndFolder;

    impl ReadBody for GameSkinAndFolder {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for GameSkinAndFolder {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::normal(1, Self::read_chunk_1),
            ]
            .into_iter()
        }
    }

    impl GameSkinAndFolder {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.internal_node_ref::<GameSkin>()?;
            r.string()?;

            Ok(())
        }

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }
    }
}
