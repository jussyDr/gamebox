//! Media block text.

use crate::Class;

/// Text media block.
#[derive(Default)]
pub struct MediaBlockText;

impl Class for MediaBlockText {
    const CLASS_ID: u32 = 0x030a8000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        control::EffectSimi,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::MediaBlockText;

    impl ReadBody for MediaBlockText {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockText {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::normal(2, Self::read_chunk_2),
            ]
            .into_iter()
        }
    }

    impl MediaBlockText {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _text = r.string()?;
            let _effect = r.internal_node_ref::<EffectSimi>()?;

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _color = r.vec3::<f32>()?;

            Ok(())
        }
    }
}
