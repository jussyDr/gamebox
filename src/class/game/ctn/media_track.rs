//! Media track.

use std::sync::Arc;

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
};

/// Media track.
#[derive(Default)]
pub struct MediaTrack {
    name: String,
    blocks: Vec<MediaBlock>,
}

impl MediaTrack {
    /// Name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Blocks.
    pub fn blocks(&self) -> &Vec<MediaBlock> {
        &self.blocks
    }
}

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

mod read {
    use std::{any::Any, sync::Arc};

    use crate::{
        ClassId, NodeRef,
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
            BodyChunk, BodyChunks, BodyReader, Error, ReadBody, ReadNodeRef,
            error_unknown_chunk_version, read_body_chunks, read_node_from_body,
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
            self.name = r.string()?;
            self.blocks = r.list_with_version(|r| r.node_ref())?;
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

    impl ReadNodeRef for MediaBlock {
        fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error> {
            match node_ref {
                NodeRef::Internal(node_ref) => node_ref
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
                    .map_err(|_| Error::new("")),
                _ => todo!(),
            }
        }

        fn read_internal_node_ref(
            r: &mut impl BodyReader,
            class_id: u32,
        ) -> Result<Arc<dyn Any + Send + Sync>, Error> {
            match class_id {
                MediaBlockCameraGame::CLASS_ID => {
                    Ok(Arc::new(read_node_from_body::<MediaBlockCameraGame>(r)?))
                }
                MediaBlockTime::CLASS_ID => Ok(Arc::new(read_node_from_body::<MediaBlockTime>(r)?)),
                MediaBlockCameraCustom::CLASS_ID => {
                    Ok(Arc::new(read_node_from_body::<MediaBlockCameraCustom>(r)?))
                }
                MediaBlockImage::CLASS_ID => {
                    Ok(Arc::new(read_node_from_body::<MediaBlockImage>(r)?))
                }
                MediaBlockText::CLASS_ID => Ok(Arc::new(read_node_from_body::<MediaBlockText>(r)?)),
                MediaBlockTransitionFade::CLASS_ID => Ok(Arc::new(read_node_from_body::<
                    MediaBlockTransitionFade,
                >(r)?)),
                MediaBlockDOF::CLASS_ID => Ok(Arc::new(read_node_from_body::<MediaBlockDOF>(r)?)),
                MediaBlockColorGrading::CLASS_ID => {
                    Ok(Arc::new(read_node_from_body::<MediaBlockColorGrading>(r)?))
                }
                MediaBlockFog::CLASS_ID => Ok(Arc::new(read_node_from_body::<MediaBlockFog>(r)?)),
                MediaBlockEntity::CLASS_ID => {
                    Ok(Arc::new(read_node_from_body::<MediaBlockEntity>(r)?))
                }
                _ => todo!("0x{class_id:08x?}"),
            }
        }
    }
}
