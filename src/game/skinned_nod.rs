//! Skinned nod.

use std::ops::Deref;

use crate::Class;

use super::Nod;

/// Skinned nod.
#[derive(Default)]
pub struct SkinnedNod {
    parent: Nod,
}

impl Class for SkinnedNod {
    const CLASS_ID: u32 = 0x0305f000;
}

impl Deref for SkinnedNod {
    type Target = Nod;

    fn deref(&self) -> &Nod {
        &self.parent
    }
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

    use super::SkinnedNod;

    impl ReadBody for SkinnedNod {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for SkinnedNod {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::normal(1, Self::read_chunk_1),
            ]
            .into_iter()
        }
    }

    impl SkinnedNod {
        fn read_chunk_0<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            let _title_logos_skin = r.external_node_ref::<GameSkin>()?;
            let _title_logos_control_style = r.external_node_ref::<()>()?;

            Ok(())
        }

        fn read_chunk_1<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.list(|r| {
                let _image = r.external_node_ref::<()>()?;
                r.string()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
