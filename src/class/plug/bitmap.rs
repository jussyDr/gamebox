//! Bitmap.

use crate::{ClassId, ExternalNodeRef, SubExtensions};

/// A bitmap.
#[derive(Default)]
pub struct Bitmap {
    image: ExternalNodeRef,
}

impl Bitmap {
    /// Image.
    pub fn image(&self) -> &ExternalNodeRef {
        &self.image
    }
}

impl SubExtensions for Bitmap {
    const SUB_EXTENSIONS: &[&str] = &["Texture"];
}

impl ClassId for Bitmap {
    const CLASS_ID: u32 = 0x09011000;
}

mod read {
    use crate::{
        class::plug::{bitmap::Bitmap, file_img::FileImg},
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            error_unknown_chunk_version, read_body_chunks,
            reader::{BodyReader, HeaderReader},
        },
    };

    impl Readable for Bitmap {}

    impl HeaderChunks for Bitmap {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            []
        }
    }

    impl ReadBody for Bitmap {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), crate::read::Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Bitmap {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(25, Self::read_chunk_25),
                BodyChunk::new(32, Self::read_chunk_32),
                BodyChunk::new(35, Self::read_chunk_35),
                BodyChunk::new(37, Self::read_chunk_37),
                BodyChunk::new(40, Self::read_chunk_40),
                BodyChunk::new(42, Self::read_chunk_42),
                BodyChunk::new(44, Self::read_chunk_44),
                BodyChunk::new(45, Self::read_chunk_45),
                BodyChunk::new(48, Self::read_chunk_48),
                BodyChunk::new(51, Self::read_chunk_51),
                BodyChunk::new(52, Self::read_chunk_52),
                BodyChunk::new(53, Self::read_chunk_53),
                BodyChunk::new(54, Self::read_chunk_54),
                BodyChunk::new(55, Self::read_chunk_55),
                BodyChunk::new(56, Self::read_chunk_56),
            ]
        }
    }

    impl Bitmap {
        fn read_chunk_25(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _bum_scale_mip_level = r.f32()?;

            Ok(())
        }

        fn read_chunk_32(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _mip_map_fade_alphas = r.list(|r| r.f32())?;

            Ok(())
        }

        fn read_chunk_35(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_37(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _default_texcoord_scale = r.vec2()?;
            let _default_texcoord_trans = r.vec2()?;
            let _default_texcoord_rotate = r.f32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_40(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_42(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _sprite_param = r.u32()?;

            Ok(())
        }

        fn read_chunk_44(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _decals = r.u32()?;

            Ok(())
        }

        fn read_chunk_45(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_48(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(error_unknown_chunk_version(version));
            }

            self.image = r.external_node_ref::<FileImg>()?;
            r.vec3()?;
            let _mip_map_lower_alpha = r.f32()?;
            let _bump_scale_factor = r.f32()?;
            let _mip_map_lod_bias_default = r.f32()?;
            let _border_rgb = r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_51(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _height_in_meters = r.f32()?;

            Ok(())
        }

        fn read_chunk_52(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(error_unknown_chunk_version(version));
            }

            let _image_array = r.u32()?;
            let _image_array_suffix = r.string()?;
            let _image_array_fids = r.list(|r| r.string())?;
            let _bitmap_array = r.u32()?;
            let _bitmap_array_elem_name = r.string()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_53(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u16()?;

            Ok(())
        }

        fn read_chunk_54(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.id_or_null()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_55(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
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

        fn read_chunk_56(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
