use std::sync::Arc;

use crate::{Class, class::plug::vertex_stream::VertexStream};

/// A visual.
#[derive(Default)]
pub struct Visual {
    vertex_streams: Vec<Arc<VertexStream>>,
}

impl Visual {
    pub fn vertex_streams(&self) -> &Vec<Arc<VertexStream>> {
        &self.vertex_streams
    }
}

impl Class for Visual {
    const CLASS_ID: u32 = 0x09006000;
}

mod read {
    use std::io::Read;

    use crate::{
        class::plug::visual::Visual,
        read::{
            BodyChunk, BodyChunks, Error,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl BodyChunks for Visual {
        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::new(5, Self::read_chunk_5),
                BodyChunk::new(9, Self::read_chunk_9),
                BodyChunk::new(11, Self::read_chunk_11),
                BodyChunk::new(15, Self::read_chunk_15),
                BodyChunk::new(16, Self::read_chunk_16),
            ]
        }
    }

    impl Visual {
        fn read_chunk_1<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, N>,
        ) -> Result<(), Error> {
            r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let sub_visuals = r.list(|r| {
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
            let splits = r.list(|r| {
                r.u32()?;
                r.u32()?;
                let bounding_box = r.box3d()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_15(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(Error("unknown chunk version".into()));
            }

            let flags = r.u32()?;
            let num_texcoord_sets = r.u32()?;
            let count = r.u32()?;
            self.vertex_streams = r.list(|r| r.internal_node_ref())?;
            let texcoord_sets: Vec<()> = r.repeat(num_texcoord_sets as usize, |r| todo!())?;
            let bounding_box = r.box3d()?;
            let bitmap_elem_to_packs: Vec<()> = r.list(|r| todo!())?;

            if version >= 5 {
                r.list(|r| r.u16())?;
            }

            if version >= 6 {
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_16<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error("unknown chunk version".into()));
            }

            let morph_count = r.u32()?;

            if morph_count > 0 {
                todo!()
            }

            Ok(())
        }
    }
}
