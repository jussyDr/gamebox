//! Material.

use std::sync::Arc;

use crate::{ClassId, SubExtensions, class::plug::material_custom::MaterialCustom};

/// A material.
#[derive(Default)]
pub struct Material {
    custom_material: Arc<MaterialCustom>,
}

impl Material {
    /// Custom material.
    pub fn custom_material(&self) -> &Arc<MaterialCustom> {
        &self.custom_material
    }
}

impl ClassId for Material {
    const CLASS_ID: u32 = 0x09079000;
}

impl SubExtensions for Material {
    const SUB_EXTENSIONS: &[&str] = &["Material"];
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::material::Material,
        read::{
            BodyChunk, BodyChunks, BodyReader, Error, HeaderChunk, HeaderChunks, HeaderReader,
            ReadBody, Readable, read_body_chunks,
        },
    };

    impl Readable for Material {}

    impl HeaderChunks for Material {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            []
        }
    }

    impl ReadBody for Material {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Material {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::new(7, Self::read_chunk_7),
                BodyChunk::new(16, Self::read_chunk_16),
                BodyChunk::new(17, Self::read_chunk_17),
                BodyChunk::skippable(18, Self::read_chunk_18),
                BodyChunk::skippable(19, Self::read_chunk_19),
                BodyChunk::new(21, Self::read_chunk_21),
                BodyChunk::new(22, Self::read_chunk_22),
                BodyChunk::new(23, Self::read_chunk_23),
                BodyChunk::skippable(25, Self::read_chunk_25),
            ]
        }
    }

    impl Material {
        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_7(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            self.custom_material = r.node_ref()?;

            Ok(())
        }

        fn read_chunk_16(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.f32()?;

            Ok(())
        }

        fn read_chunk_17(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _: Vec<Arc<str>> = r.list(|r| r.id())?;

            Ok(())
        }

        fn read_chunk_18(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.string()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_19(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_21(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?; // 7
            r.u32()?;
            r.list(|r| r.u32())?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_22(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_23(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_25(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
