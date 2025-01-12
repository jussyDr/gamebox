//! Media block text.

use std::sync::Arc;

use bytemuck::cast;

use crate::{control::EffectSimi, Class, OrderedRgbFloat, RgbFloat};

/// Text media block.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockText {
    text: String,
    effect: Arc<EffectSimi>,
    color: OrderedRgbFloat,
}

impl Class for MediaBlockText {
    const CLASS_ID: u32 = 0x030a8000;
}

impl MediaBlockText {
    /// Text.
    pub const fn text(&self) -> &String {
        &self.text
    }

    /// Effect.
    pub const fn effect(&self) -> &Arc<EffectSimi> {
        &self.effect
    }

    /// Color.
    pub fn color(&self) -> RgbFloat {
        cast(self.color)
    }
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
            self.text = r.string()?;
            self.effect = r.internal_node_ref::<EffectSimi>()?;

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.color = r.rgb_float_ordered()?;

            Ok(())
        }
    }
}

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::MediaBlockText;

    impl WriteBody for MediaBlockText {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaBlockText {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
