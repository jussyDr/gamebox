//! Tree.

use crate::Class;

/// Tree.
#[derive(Default)]
pub struct Tree;

impl Class for Tree {
    const CLASS_ID: u32 = 0x0904f000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::Surface,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::Tree;

    impl ReadBody for Tree {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Tree {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(6, Self::read_chunk_6),
                BodyChunk::normal(13, Self::read_chunk_13),
                BodyChunk::normal(17, Self::read_chunk_17),
                BodyChunk::normal(22, Self::read_chunk_22),
                BodyChunk::normal(26, Self::read_chunk_26),
            ]
            .into_iter()
        }
    }

    impl Tree {
        fn read_chunk_6<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _children: Vec<()> = r.list_with_version(|_| todo!())?;

            Ok(())
        }

        fn read_chunk_13<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _name = r.id()?;
            r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_17<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_22(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            let _surface = r.internal_node_ref::<Surface>()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_26<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let flags = r.u32()?;

            if flags & 4 != 0 {
                r.vec3::<f32>()?;
                r.vec3::<f32>()?;
                r.vec3::<f32>()?;
                r.vec3::<f32>()?;
            }

            Ok(())
        }
    }
}