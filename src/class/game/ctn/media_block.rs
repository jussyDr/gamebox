use std::{any::Any, cell::OnceCell, sync::Arc};

use crate::{
    game::ctn::{
        MediaBlockCameraCustom, MediaBlockCameraGame, MediaBlockColorGrading, MediaBlockDOF,
        MediaBlockEntity, MediaBlockFog, MediaBlockImage, MediaBlockText, MediaBlockTime,
        MediaBlockTransitionFade,
    },
    read::{ClassId, Error, ReadNodeRef},
};

pub enum MediaBlock<'a> {
    CameraCustom(&'a MediaBlockCameraCustom),
    CameraGame(&'a MediaBlockCameraGame),
    ColorGrading(&'a MediaBlockColorGrading),
    DOF(&'a MediaBlockDOF),
    Entity(&'a MediaBlockEntity),
    Fog(&'a MediaBlockFog),
    Image(&'a MediaBlockImage),
    Text(&'a MediaBlockText),
    Time(&'a MediaBlockTime),
    TransitionFade(&'a MediaBlockTransitionFade),
}

impl<'a> ReadNodeRef<'a> for MediaBlock<'a> {
    fn read(
        body_data: Arc<[u8]>,
        body_data_offset: &mut usize,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
        seen_id: &mut bool,
        ids: &mut Vec<(usize, usize)>,
        class_id: u32,
    ) -> Result<Box<dyn Any>, Error> {
        let node: Box<dyn Any> = match class_id {
            MediaBlockCameraCustom::CLASS_ID => Box::new(MediaBlockCameraCustom::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            MediaBlockCameraGame::CLASS_ID => Box::new(MediaBlockCameraGame::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            MediaBlockColorGrading::CLASS_ID => Box::new(MediaBlockColorGrading::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            MediaBlockDOF::CLASS_ID => Box::new(MediaBlockDOF::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            MediaBlockEntity::CLASS_ID => Box::new(MediaBlockEntity::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            MediaBlockFog::CLASS_ID => Box::new(MediaBlockFog::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            MediaBlockImage::CLASS_ID => Box::new(MediaBlockImage::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            MediaBlockText::CLASS_ID => Box::new(MediaBlockText::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            MediaBlockTime::CLASS_ID => Box::new(MediaBlockTime::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            MediaBlockTransitionFade::CLASS_ID => Box::new(MediaBlockTransitionFade::read(
                body_data,
                body_data_offset,
                node_refs,
                seen_id,
                ids,
            )?),
            _ => todo!(),
        };

        Ok(node)
    }

    fn upcast(node_ref: &'a dyn Any) -> Result<Self, Error> {
        node_ref
            .downcast_ref()
            .map(Self::CameraCustom)
            .or_else(|| node_ref.downcast_ref().map(Self::CameraGame))
            .or_else(|| node_ref.downcast_ref().map(Self::ColorGrading))
            .or_else(|| node_ref.downcast_ref().map(Self::DOF))
            .or_else(|| node_ref.downcast_ref().map(Self::Entity))
            .or_else(|| node_ref.downcast_ref().map(Self::Fog))
            .or_else(|| node_ref.downcast_ref().map(Self::Image))
            .or_else(|| node_ref.downcast_ref().map(Self::Text))
            .or_else(|| node_ref.downcast_ref().map(Self::Time))
            .or_else(|| node_ref.downcast_ref().map(Self::TransitionFade))
            .ok_or_else(|| Error::new("upcast"))
    }
}
