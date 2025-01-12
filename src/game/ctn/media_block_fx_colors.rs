//! Media block Fx colors.

use ordered_float::OrderedFloat;

use crate::Class;

/// Fx colors media block.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockFxColors {
    keys: Vec<Key>,
}

impl Class for MediaBlockFxColors {
    const CLASS_ID: u32 = 0x03080000;
}

impl MediaBlockFxColors {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Fx colors media block key.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Key {
    time: OrderedFloat<f32>,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time.0
    }
}

mod read {
    use std::io::{Read, Seek};

    use ordered_float::OrderedFloat;

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockFxColors};

    impl ReadBody for MediaBlockFxColors {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockFxColors {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(3, Self::read_chunk_3)].into_iter()
        }
    }

    impl MediaBlockFxColors {
        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.keys = r.list(|r| {
                let time = r.f32()?;
                let _intensity = r.f32()?;
                let _blend_z = r.f32()?;
                let _distance = r.f32()?;
                let _far_distance = r.f32()?;
                let _inverse = r.f32()?;
                let _hue = r.f32()?;
                let _saturation = r.f32()?;
                let _brightness = r.f32()?;
                let _contrast = r.f32()?;
                let _rgb = r.vec3()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                let _far_inverse = r.f32()?;
                let _far_hue = r.f32()?;
                let _far_saturation = r.f32()?;
                let _far_brightness = r.f32()?;
                let _far_contrast = r.f32()?;
                let _far_rgb = r.vec3()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;

                Ok(Key {
                    time: OrderedFloat(time),
                })
            })?;

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

    use super::MediaBlockFxColors;

    impl WriteBody for MediaBlockFxColors {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaBlockFxColors {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
