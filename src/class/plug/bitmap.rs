use crate::{ClassId, ExternalNodeRef, SubExtensions};

/// A bitmap.
#[derive(Default)]
pub struct Bitmap {
    image: ExternalNodeRef,
}

impl Bitmap {
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
    use std::io::Read;

    use crate::{
        class::plug::{bitmap::Bitmap, file_img::FileImg},
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            error_unknown_chunk_version, read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl Readable for Bitmap {}

    impl HeaderChunks for Bitmap {
        fn header_chunks<R, I, N>() -> impl IntoIterator<Item = HeaderChunk<Self, R, I, N>> {
            []
        }
    }

    impl ReadBody for Bitmap {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), crate::read::Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Bitmap {
        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
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
        fn read_chunk_25<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let bum_scale_mip_level = r.f32()?;

            Ok(())
        }

        fn read_chunk_32<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let mip_map_fade_alphas = r.list(|r| r.f32())?;

            Ok(())
        }

        fn read_chunk_35<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_37<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let default_texcoord_scale = r.vec2()?;
            let default_texcoord_trans = r.vec2()?;
            let default_texcoord_rotate = r.f32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_40<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_42<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let sprite_param = r.u32()?;

            Ok(())
        }

        fn read_chunk_44<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let decals = r.u32()?;

            Ok(())
        }

        fn read_chunk_45<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_48(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(error_unknown_chunk_version(version));
            }

            self.image = r.external_node_ref::<FileImg>()?;
            r.vec3()?;
            let mip_map_lower_alpha = r.f32()?;
            let bump_scale_factor = r.f32()?;
            let mip_map_lod_bias_default = r.f32()?;
            let border_rgb = r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_51<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let height_in_meters = r.f32()?;

            Ok(())
        }

        fn read_chunk_52<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(error_unknown_chunk_version(version));
            }

            let image_array = r.u32()?;
            let image_array_suffix = r.string()?;
            let image_array_fids = r.list(|r| r.string())?;
            let bitmap_array = r.u32()?;
            let bitmap_array_elem_name = r.string()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_53<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u16()?;

            Ok(())
        }

        fn read_chunk_54<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, N>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.id_or_null()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_55<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
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

        fn read_chunk_56<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
