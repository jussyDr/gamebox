//! Visual.

use std::sync::Arc;

use crate::{ClassId, class::plug::vertex_stream::VertexStream};

/// A visual.
#[derive(Default)]
pub struct Visual {
    vertex_streams: Vec<Arc<VertexStream>>,
}

impl Visual {
    /// Vertex streams.
    pub fn vertex_streams(&self) -> &Vec<Arc<VertexStream>> {
        &self.vertex_streams
    }
}

impl ClassId for Visual {
    const CLASS_ID: u32 = 0x09006000;
}

mod read {

    use std::sync::Arc;

    use crate::{
        class::plug::visual::Visual,
        read::{BodyChunk, BodyChunks, Error, error_unknown_chunk_version, reader::BodyReader},
    };

    impl BodyChunks for Visual {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
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
        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _: Option<Arc<str>> = r.id()?;

            Ok(())
        }

        fn read_chunk_5(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _sub_visuals = r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_9(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.f32()?;

            Ok(())
        }

        fn read_chunk_11(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _splits = r.list(|r| {
                r.u32()?;
                r.u32()?;
                let _bounding_box = r.box3d()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_15(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(error_unknown_chunk_version(version));
            }

            let _flags = r.u32()?;
            let num_texcoord_sets = r.u32()?;
            let _count = r.u32()?;
            self.vertex_streams = r.list(|r| r.internal_node_ref())?;
            let _texcoord_sets: Vec<()> = r.repeat(num_texcoord_sets as usize, |r| todo!())?;
            let _bounding_box = r.box3d()?;
            let _bitmap_elem_to_packs: Vec<()> = r.list(|r| todo!())?;

            if version >= 5 {
                r.list(|r| r.u16())?;
            }

            if version >= 6 {
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_16(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let morph_count = r.u32()?;

            if morph_count > 0 {
                todo!()
            }

            Ok(())
        }
    }
}
