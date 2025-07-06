//! Media track.

use std::{any::Any, sync::Arc};

use crate::{
    ClassId,
    class::game::ctn::{
        media_block_camera_custom::MediaBlockCameraCustom, media_block_fog::MediaBlockFog,
        media_block_image::MediaBlockImage,
    },
    read::reader::Downcast,
};

/// Media track.
#[derive(Default)]
pub struct MediaTrack;

impl ClassId for MediaTrack {
    const CLASS_ID: u32 = 0x03078000;
}

/// Media block.
#[derive(Clone)]
pub enum MediaBlock {
    /// Camera custom.
    CameraCustom(Arc<MediaBlockCameraCustom>),
    /// Image.
    Image(Arc<MediaBlockImage>),
    /// Fog.
    Fog(Arc<MediaBlockFog>),
}

impl Downcast for MediaBlock {
    fn downcast(value: Arc<dyn Any + Send + Sync>) -> Option<Self> {
        value
            .downcast()
            .map(Self::CameraCustom)
            .or_else(|value| value.downcast().map(Self::Image))
            .or_else(|value| value.downcast().map(Self::Fog))
            .ok()
    }
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::game::ctn::{
            media_block_camera_custom::MediaBlockCameraCustom,
            media_block_fog::MediaBlockFog,
            media_block_image::MediaBlockImage,
            media_track::{MediaBlock, MediaTrack},
        },
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            read_node_from_body, reader::BodyReader,
        },
    };

    impl ReadBody for MediaTrack {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaTrack {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::new(5, Self::read_chunk_5),
            ]
        }
    }

    impl MediaTrack {
        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _name = r.string()?;
            let _blocks: Vec<MediaBlock> = r.list_with_version(|r| {
                r.internal_node_ref_generic(|r, class_id| match class_id {
                    0x030a2000 => {
                        let node = read_node_from_body::<MediaBlockCameraCustom>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x030a5000 => {
                        let node = read_node_from_body::<MediaBlockImage>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x030ab000 => {
                        todo!()
                    }
                    0x03199000 => {
                        let node = read_node_from_body::<MediaBlockFog>(r)?;
                        Ok(Arc::new(node))
                    }
                    _ => todo!("0x{class_id:08x?}"),
                })
            })?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_5(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            let _is_keep_playing = r.bool32()?;
            let _is_read_only = r.bool32()?;
            let _is_cycling = r.bool32()?;
            let _repeating_segment_start = r.f32()?;
            let _repeating_segment_end = r.f32()?;

            Ok(())
        }
    }
}
