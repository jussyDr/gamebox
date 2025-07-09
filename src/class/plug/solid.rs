//! Solid.

use crate::ClassId;

/// Solid.
#[derive(Default)]
pub struct Solid;

impl ClassId for Solid {
    const CLASS_ID: u32 = 0x09005000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::{solid::Solid, tree::Tree},
        read::{
            BodyChunk, BodyChunks, BodyReader, Error, ReadBody, error_unknown_chunk_version,
            error_unknown_version, read_body_chunks,
        },
    };

    impl ReadBody for Solid {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Solid {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(0, Self::read_chunk_0),
                BodyChunk::new(16, Self::read_chunk_16),
                BodyChunk::new(17, Self::read_chunk_17),
                BodyChunk::new(23, Self::read_chunk_23),
                BodyChunk::new(25, Self::read_chunk_25),
                BodyChunk::skippable(26, Self::read_chunk_26),
            ]
        }
    }

    impl Solid {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _type_and_index = r.u32()?;

            Ok(())
        }

        fn read_chunk_16(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_17(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.bool32()?;

            if r.bool32()? {
                r.bool32()?;
            }

            let _tree: Arc<Tree> = r.node_ref()?;

            Ok(())
        }

        fn read_chunk_23(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            if r.bool32()? {
                let pre_light_gen_version = r.u32()?;

                if pre_light_gen_version != 1 {
                    return Err(error_unknown_version(
                        "pre light generator",
                        pre_light_gen_version,
                    ));
                }

                r.u32()?;
                r.f32()?;
                r.bool32()?;
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

        fn read_chunk_25(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(error_unknown_chunk_version(version));
            }

            let _: Vec<()> = r.list_with_version(|r| todo!())?;
            let _: Vec<()> = r.list_with_version(|r| todo!())?;
            r.list(|r| {
                r.u32()?;
                r.iso4()?;

                Ok(())
            })?;
            r.list(|r| {
                r.u32()?;
                r.iso4()?;

                Ok(())
            })?;
            r.u32()?;
            let _: Vec<Arc<str>> = r.list(|r| r.id())?;
            r.list(|r| r.iso4())?;
            r.string()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_26(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
