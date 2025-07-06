//! Media track.

use std::{any::Any, sync::Arc};

use crate::{
    ClassId,
    class::game::ctn::{
        media_block_camera_custom::MediaBlockCameraCustom,
        media_block_camera_game::MediaBlockCameraGame,
        media_block_color_grading::MediaBlockColorGrading, media_block_dof::MediaBlockDOF,
        media_block_entity::MediaBlockEntity, media_block_fog::MediaBlockFog,
        media_block_image::MediaBlockImage, media_block_text::MediaBlockText,
        media_block_time::MediaBlockTime, media_block_transition_fade::MediaBlockTransitionFade,
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
    /// Camera game.
    CameraGame(Arc<MediaBlockCameraGame>),
    /// Time.
    Time(Arc<MediaBlockTime>),
    /// Camera custom.
    CameraCustom(Arc<MediaBlockCameraCustom>),
    /// Image.
    Image(Arc<MediaBlockImage>),
    /// Text.
    Text(Arc<MediaBlockText>),
    /// Transition fade.
    TransitionFade(Arc<MediaBlockTransitionFade>),
    /// DOF.
    DOF(Arc<MediaBlockDOF>),
    /// Color grading.
    ColorGrading(Arc<MediaBlockColorGrading>),
    /// Fog.
    Fog(Arc<MediaBlockFog>),
    /// Entity.
    Entity(Arc<MediaBlockEntity>),
}

impl Downcast for MediaBlock {
    fn downcast(value: Arc<dyn Any + Send + Sync>) -> Option<Self> {
        value
            .downcast()
            .map(Self::CameraGame)
            .or_else(|value| value.downcast().map(Self::Time))
            .or_else(|value| value.downcast().map(Self::CameraCustom))
            .or_else(|value| value.downcast().map(Self::Image))
            .or_else(|value| value.downcast().map(Self::Text))
            .or_else(|value| value.downcast().map(Self::TransitionFade))
            .or_else(|value| value.downcast().map(Self::DOF))
            .or_else(|value| value.downcast().map(Self::ColorGrading))
            .or_else(|value| value.downcast().map(Self::Fog))
            .or_else(|value| value.downcast().map(Self::Entity))
            .ok()
    }
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::game::ctn::{
            media_block_camera_custom::MediaBlockCameraCustom,
            media_block_camera_game::MediaBlockCameraGame,
            media_block_color_grading::MediaBlockColorGrading,
            media_block_dof::MediaBlockDOF,
            media_block_entity::MediaBlockEntity,
            media_block_fog::MediaBlockFog,
            media_block_image::MediaBlockImage,
            media_block_text::MediaBlockText,
            media_block_time::MediaBlockTime,
            media_block_transition_fade::MediaBlockTransitionFade,
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
                    0x03084000 => {
                        let node = read_node_from_body::<MediaBlockCameraGame>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x03085000 => {
                        let node = read_node_from_body::<MediaBlockTime>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x030a2000 => {
                        let node = read_node_from_body::<MediaBlockCameraCustom>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x030a5000 => {
                        let node = read_node_from_body::<MediaBlockImage>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x030a8000 => {
                        let node = read_node_from_body::<MediaBlockText>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x030ab000 => {
                        let node = read_node_from_body::<MediaBlockTransitionFade>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x03126000 => {
                        let node = read_node_from_body::<MediaBlockDOF>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x03186000 => {
                        let node = read_node_from_body::<MediaBlockColorGrading>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x03199000 => {
                        let node = read_node_from_body::<MediaBlockFog>(r)?;
                        Ok(Arc::new(node))
                    }
                    0x0329f000 => {
                        let node = read_node_from_body::<MediaBlockEntity>(r)?;
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
