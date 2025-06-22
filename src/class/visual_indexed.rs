use crate::{
    Class,
    class::{index_buffer::IndexBuffer, visual_3d::Visual3D},
};

#[derive(Default)]
pub struct VisualIndexed {
    parent: Visual3D,
    index_buffer: IndexBuffer,
}

impl Class for VisualIndexed {
    fn class_id(&self) -> u32 {
        0x0906a000
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::{visual_3d::Visual3D, visual_indexed::VisualIndexed},
        read::{
            BodyChunk, BodyChunks, Error, read_node,
            reader::{IdsMut, NodesMut, Reader},
        },
    };

    impl BodyChunks for VisualIndexed {
        type Parent = Visual3D;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdsMut, N: NodesMut>()
        -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(0x0906a001, Self::read_chunk_1)].into_iter()
        }
    }

    impl VisualIndexed {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, impl NodesMut>,
        ) -> Result<(), Error> {
            if r.bool32()? {
                self.index_buffer = read_node(r)?;
            }

            Ok(())
        }
    }
}
