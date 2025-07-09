//! Tree.

use crate::ClassId;

/// Tree.
#[derive(Default)]
pub struct Tree;

impl ClassId for Tree {
    const CLASS_ID: u32 = 0x0904f000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::{surface::Surface, tree::Tree},
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for Tree {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Tree {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(6, Self::read_chunk_6),
                BodyChunk::new(13, Self::read_chunk_13),
                BodyChunk::new(17, Self::read_chunk_17),
                BodyChunk::new(22, Self::read_chunk_22),
                BodyChunk::new(26, Self::read_chunk_26),
            ]
        }
    }

    impl Tree {
        fn read_chunk_6(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _children: Vec<Arc<Tree>> = r.list_with_version(|r| r.node_ref())?;

            Ok(())
        }

        fn read_chunk_13(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _name: Arc<str> = r.id()?;
            let _: Option<Arc<str>> = r.id()?;

            Ok(())
        }

        fn read_chunk_17(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _func_tree = r.u32()?;

            Ok(())
        }

        fn read_chunk_22(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _visual = r.u32()?;
            let _shader = r.u32()?;
            let _surface: Arc<Surface> = r.node_ref()?;
            let _generator = r.u32()?;

            Ok(())
        }

        fn read_chunk_26(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let flags = r.u32()?;

            if flags & 0x00000004 != 0 {
                let _location = r.iso4()?;
            }

            Ok(())
        }
    }
}
