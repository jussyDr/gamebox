use std::{any::Any, sync::Arc};

use crate::{
    game::ctn::{
        MediaBlockCameraCustom, MediaBlockCameraGame, MediaBlockColorGrading, MediaBlockDOF,
        MediaBlockEntity, MediaBlockFog, MediaBlockImage, MediaBlockText, MediaBlockTime,
        MediaBlockTransitionFade,
    },
    read::{BodyReader, Error, ReadNode, ReadNodeRef, Result},
};

pub enum MediaBlock {
    CameraCustom(Arc<MediaBlockCameraCustom>),
    CameraGame(Arc<MediaBlockCameraGame>),
    ColorGrading(Arc<MediaBlockColorGrading>),
    DOF(Arc<MediaBlockDOF>),
    Entity(Arc<MediaBlockEntity>),
    Fog(Arc<MediaBlockFog>),
    Image(Arc<MediaBlockImage>),
    Text(Arc<MediaBlockText>),
    Time(Arc<MediaBlockTime>),
    TransitionFade(Arc<MediaBlockTransitionFade>),
}

impl ReadNodeRef for MediaBlock {
    fn read_node_ref(r: &mut impl BodyReader, class_id: u32) -> Result<Arc<dyn Any + Send + Sync>> {
        match class_id {
            MediaBlockCameraCustom::CLASS_ID => Ok(Arc::new(MediaBlockCameraCustom::read_node(r)?)),
            MediaBlockCameraGame::CLASS_ID => Ok(Arc::new(MediaBlockCameraGame::read_node(r)?)),
            MediaBlockColorGrading::CLASS_ID => Ok(Arc::new(MediaBlockColorGrading::read_node(r)?)),
            MediaBlockDOF::CLASS_ID => Ok(Arc::new(MediaBlockDOF::read_node(r)?)),
            MediaBlockEntity::CLASS_ID => Ok(Arc::new(MediaBlockEntity::read_node(r)?)),
            MediaBlockFog::CLASS_ID => Ok(Arc::new(MediaBlockFog::read_node(r)?)),
            MediaBlockImage::CLASS_ID => Ok(Arc::new(MediaBlockImage::read_node(r)?)),
            MediaBlockText::CLASS_ID => Ok(Arc::new(MediaBlockText::read_node(r)?)),
            MediaBlockTime::CLASS_ID => Ok(Arc::new(MediaBlockTime::read_node(r)?)),
            MediaBlockTransitionFade::CLASS_ID => {
                Ok(Arc::new(MediaBlockTransitionFade::read_node(r)?))
            }
            _ => todo!("{class_id:08x?}"),
        }
    }

    fn from_any(node_ref: Arc<dyn Any + Send + Sync>) -> Result<Self> {
        node_ref
            .downcast()
            .map(Self::CameraCustom)
            .or_else(|node_ref| node_ref.downcast().map(MediaBlock::CameraGame))
            .or_else(|node_ref| node_ref.downcast().map(MediaBlock::ColorGrading))
            .or_else(|node_ref| node_ref.downcast().map(MediaBlock::DOF))
            .or_else(|node_ref| node_ref.downcast().map(MediaBlock::Entity))
            .or_else(|node_ref| node_ref.downcast().map(MediaBlock::Fog))
            .or_else(|node_ref| node_ref.downcast().map(MediaBlock::Image))
            .or_else(|node_ref| node_ref.downcast().map(MediaBlock::Text))
            .or_else(|node_ref| node_ref.downcast().map(MediaBlock::Time))
            .or_else(|node_ref| node_ref.downcast().map(MediaBlock::TransitionFade))
            .map_err(|_| Error::Internal("".into()))
    }
}
