//! Media block image.

use std::sync::Arc;

use crate::{control::EffectSimi, Class, FileRef};

/// Image media block.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockImage {
    effect: Arc<EffectSimi>,
    image: FileRef,
}

impl Class for MediaBlockImage {
    const CLASS_ID: u32 = 0x030a5000;
}

impl MediaBlockImage {
    /// Effect.
    pub const fn effect(&self) -> &Arc<EffectSimi> {
        &self.effect
    }

    /// Image.
    pub const fn image(&self) -> &FileRef {
        &self.image
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaBlockImage;

    impl ReadBody for MediaBlockImage {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockImage {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl MediaBlockImage {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.effect = r.internal_node_ref()?;
            self.image = r.file_ref()?;

            Ok(())
        }
    }
}
