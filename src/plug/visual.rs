use std::sync::Arc;

use crate::Class;

use super::vertex_stream::VertexStream;

/// A visual.
#[derive(Default)]
pub struct Visual {
    vertex_streams: Vec<Arc<VertexStream>>,
}

impl Class for Visual {
    const CLASS_ID: u32 = 0x09006000;
}

impl Visual {
    pub fn vertex_streams(&self) -> &[Arc<VertexStream>] {
        &self.vertex_streams
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::vertex_stream::VertexStream,
        read::{
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error,
        },
    };

    use super::Visual;

    impl BodyChunks for Visual {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::new(5, Self::read_chunk_5),
                BodyChunk::new(9, Self::read_chunk_9),
                BodyChunk::new(11, Self::read_chunk_11),
                BodyChunk::new(15, Self::read_chunk_15),
                BodyChunk::new(16, Self::read_chunk_16),
            ]
            .into_iter()
        }
    }

    impl Visual {
        fn read_chunk_1<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _sub_visuals = r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_9<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.f32()?;

            Ok(())
        }

        fn read_chunk_11<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _splits = r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.box3d()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_15(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(Error::chunk_version(version));
            }

            let _flags = r.u32()?;
            let _num_texcoord_sets = r.u32()?;
            let _count = r.u32()?;
            self.vertex_streams = r.list(|r| r.internal_node_ref::<VertexStream>())?;

            let _bounding_box = r.box3d()?;
            r.u32()?;
            r.list(|r| r.u16())?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_16<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _morph_count = r.u32()?;

            Ok(())
        }
    }
}
