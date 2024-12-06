//! Bitmap.

use crate::{read::reader::ExternalNodeRef, Class};

/// Bitmap.
#[derive(Default)]
pub struct Bitmap {
    image: ExternalNodeRef,
}

impl Class for Bitmap {
    const CLASS_ID: u32 = 0x09011000;
}

impl Bitmap {
    /// Image of the bitmap.
    pub const fn image(&self) -> &ExternalNodeRef {
        &self.image
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        readable::Sealed,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody, Readable,
    };

    use super::Bitmap;

    impl Readable for Bitmap {}

    impl Sealed for Bitmap {}

    impl ReadBody for Bitmap {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Bitmap {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(25, Self::read_chunk_25),
                BodyChunk::normal(32, Self::read_chunk_32),
                BodyChunk::normal(35, Self::read_chunk_35),
                BodyChunk::normal(37, Self::read_chunk_37),
                BodyChunk::normal(40, Self::read_chunk_40),
                BodyChunk::normal(42, Self::read_chunk_42),
                BodyChunk::normal(44, Self::read_chunk_44),
                BodyChunk::normal(45, Self::read_chunk_45),
                BodyChunk::normal(48, Self::read_chunk_48),
                BodyChunk::normal(50, Self::read_chunk_50),
                BodyChunk::normal(51, Self::read_chunk_51),
                BodyChunk::normal(52, Self::read_chunk_52),
                BodyChunk::normal(53, Self::read_chunk_53),
                BodyChunk::normal(54, Self::read_chunk_54),
                BodyChunk::normal(55, Self::read_chunk_55),
                BodyChunk::normal(56, Self::read_chunk_56),
            ]
            .into_iter()
        }
    }

    impl Bitmap {
        fn read_chunk_25<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _bump_scale_mip_level = r.f32()?;

            Ok(())
        }

        fn read_chunk_32<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _mip_map_fade_alphas = r.list(|r| r.f32())?;

            Ok(())
        }

        fn read_chunk_35<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_37<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _default_texcoord_scale = r.f32()?;
            r.f32()?;
            let _default_texcoord_trans = r.f32()?;
            r.f32()?;
            let _default_texcoord_rotate = r.f32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_40<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_42<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_44<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_45<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_48(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(Error::chunk_version(version));
            }

            self.image = r.external_node_ref::<()>()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            let _mip_map_lower_alpha = r.f32()?;
            let _bump_scale_factor = r.f32()?;
            let _mip_map_lod_bias_default = r.f32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_50<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_51<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.f32()?;

            Ok(())
        }

        fn read_chunk_52<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_53<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u16()?;

            Ok(())
        }

        fn read_chunk_54<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
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
