//! Material custom.

use std::sync::Arc;

use crate::{ClassId, ExternalNodeRef};

/// A custom material.
#[derive(Default)]
pub struct MaterialCustom {
    textures: Vec<MaterialCustomTexture>,
}

impl MaterialCustom {
    /// Textures.
    pub fn textures(&self) -> &Vec<MaterialCustomTexture> {
        &self.textures
    }
}

impl ClassId for MaterialCustom {
    const CLASS_ID: u32 = 0x0903a000;
}

#[derive(Debug)]
pub struct MaterialCustomTexture {
    name: Arc<str>,
    texture: ExternalNodeRef,
}

impl MaterialCustomTexture {
    /// Name.
    pub fn name(&self) -> &Arc<str> {
        &self.name
    }

    /// Texture.
    pub fn texture(&self) -> &ExternalNodeRef {
        &self.texture
    }
}

mod read {
    use crate::{
        class::plug::{
            bitmap::Bitmap,
            material_custom::{MaterialCustom, MaterialCustomTexture},
        },
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for MaterialCustom {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), crate::read::Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MaterialCustom {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(4, Self::read_chunk_4),
                BodyChunk::new(10, Self::read_chunk_10),
                BodyChunk::new(12, Self::read_chunk_12),
                BodyChunk::skippable(15, Self::read_chunk_15),
                BodyChunk::skippable(17, Self::read_chunk_17),
                BodyChunk::new(18, Self::read_chunk_18),
                BodyChunk::new(19, Self::read_chunk_19),
                BodyChunk::new(20, Self::read_chunk_20),
                BodyChunk::new(21, Self::read_chunk_21),
                BodyChunk::new(22, Self::read_chunk_22),
            ]
        }
    }

    impl MaterialCustom {
        fn read_chunk_4(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.list(|r| r.u32())?;

            Ok(())
        }

        fn read_chunk_10(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let gpu_fxs1 = r.list(|r| {
                r.id()?;
                let count1 = r.u32()?;
                let count2 = r.u32()?;
                r.bool32()?;

                for _ in 0..count2 {
                    r.repeat(count1 as usize, |r| r.f32())?;
                }

                Ok(())
            })?;
            let gpu_fxs2 = r.list(|r| {
                r.id()?;
                let count1 = r.u32()?;
                let count2 = r.u32()?;
                r.bool32()?;

                for _ in 0..count2 {
                    r.repeat(count1 as usize, |r| r.f32())?;
                }

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_12(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let skip_samplers = r.list(|r| {
                let name = r.id()?;
                r.bool32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_15(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_chunk_version(version));
            }

            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.list(|r| {
                r.id()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_17(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_18(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_19(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            self.textures = r.list(|r| {
                let name = r.id()?;
                r.u32()?; // 0
                let texture = r.external_node_ref::<Bitmap>()?;
                r.u32()?; // 4
                r.u32()?; // 4

                Ok(MaterialCustomTexture { name, texture })
            })?;

            Ok(())
        }

        fn read_chunk_20(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            r.list(|r| {
                r.u32()?;
                r.byte_buf()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_21(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_chunk_version(version));
            }

            if r.u32()? == 0 {
                r.string()?;
                r.string()?;
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_22(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
