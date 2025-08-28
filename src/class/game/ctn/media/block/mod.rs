mod camera_custom;
pub use camera_custom::CameraCustom;

mod camera_game;
pub use camera_game::CameraGame;

mod color_grading;
pub use color_grading::ColorGrading;

mod dof;
pub use dof::DOF;

mod entity;
pub use entity::Entity;

mod fog;
pub use fog::Fog;

mod image;
pub use image::Image;

mod text;
pub use text::Text;

mod time;
pub use time::Time;

mod transition_fade;
pub use transition_fade::TransitionFade;

use std::{any::Any, sync::Arc};

use crate::read::{BodyReader, Error, ReadNode, ReadNodeRef, Result};

pub enum Block {
    CameraCustom(Arc<CameraCustom>),
    CameraGame(Arc<CameraGame>),
    ColorGrading(Arc<ColorGrading>),
    DOF(Arc<DOF>),
    Entity(Arc<Entity>),
    Fog(Arc<Fog>),
    Image(Arc<Image>),
    Text(Arc<Text>),
    Time(Arc<Time>),
    TransitionFade(Arc<TransitionFade>),
}

impl ReadNodeRef for Block {
    fn read_node_ref(r: &mut impl BodyReader, class_id: u32) -> Result<Arc<dyn Any + Send + Sync>> {
        match class_id {
            CameraCustom::CLASS_ID => Ok(Arc::new(CameraCustom::read_node(r)?)),
            CameraGame::CLASS_ID => Ok(Arc::new(CameraGame::read_node(r)?)),
            ColorGrading::CLASS_ID => Ok(Arc::new(ColorGrading::read_node(r)?)),
            DOF::CLASS_ID => Ok(Arc::new(DOF::read_node(r)?)),
            Entity::CLASS_ID => Ok(Arc::new(Entity::read_node(r)?)),
            Fog::CLASS_ID => Ok(Arc::new(Fog::read_node(r)?)),
            Image::CLASS_ID => Ok(Arc::new(Image::read_node(r)?)),
            Text::CLASS_ID => Ok(Arc::new(Text::read_node(r)?)),
            Time::CLASS_ID => Ok(Arc::new(Time::read_node(r)?)),
            TransitionFade::CLASS_ID => Ok(Arc::new(TransitionFade::read_node(r)?)),
            _ => todo!("{class_id:08x?}"),
        }
    }

    fn from_option_any(node_ref: Option<Arc<dyn Any + Send + Sync>>) -> Result<Self> {
        match node_ref {
            None => todo!(),
            Some(node_ref) => node_ref
                .downcast()
                .map(Self::CameraCustom)
                .or_else(|node_ref| node_ref.downcast().map(Block::CameraGame))
                .or_else(|node_ref| node_ref.downcast().map(Block::ColorGrading))
                .or_else(|node_ref| node_ref.downcast().map(Block::DOF))
                .or_else(|node_ref| node_ref.downcast().map(Block::Entity))
                .or_else(|node_ref| node_ref.downcast().map(Block::Fog))
                .or_else(|node_ref| node_ref.downcast().map(Block::Image))
                .or_else(|node_ref| node_ref.downcast().map(Block::Text))
                .or_else(|node_ref| node_ref.downcast().map(Block::Time))
                .or_else(|node_ref| node_ref.downcast().map(Block::TransitionFade))
                .map_err(|_| Error::Internal("".into())),
        }
    }
}
