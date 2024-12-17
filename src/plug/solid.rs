//! Solid.

use crate::Class;

/// Solid.
#[derive(Default)]
pub struct Solid;

impl Class for Solid {
    const CLASS_ID: u32 = 0x09005000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::Tree,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::Solid;

    impl ReadBody for Solid {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Solid {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::normal(16, Self::read_chunk_16),
                BodyChunk::normal(17, Self::read_chunk_17),
                BodyChunk::normal(23, Self::read_chunk_23),
                BodyChunk::normal(25, Self::read_chunk_25),
                BodyChunk::skippable(26, Self::read_chunk_26),
            ]
            .into_iter()
        }
    }

    impl Solid {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _type_and_index = r.u32()?;

            Ok(())
        }

        fn read_chunk_16<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_17(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.bool()?;

            if r.bool()? {
                r.bool()?;
            }

            let _tree = r.internal_node_ref::<Tree>()?;

            Ok(())
        }

        fn read_chunk_23<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            if r.bool()? {
                let version = r.u32()?;

                if version != 1 {
                    return Err(Error::version("pre", version));
                }

                r.u32()?;
                r.f32()?;
                r.bool()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.u32()?;
                r.u32()?;
                r.list(|r| r.box3d())?;
                r.list(|r| {
                    r.f32()?;
                    r.f32()?;
                    r.f32()?;
                    r.f32()?;
                    r.f32()?;

                    Ok(())
                })?;
            }

            let _file_write_time = r.u64()?;

            Ok(())
        }

        fn read_chunk_25<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(Error::chunk_version(version));
            }

            r.list_with_version(|_| Ok(()))?;
            r.list_with_version(|_| Ok(()))?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_26<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
