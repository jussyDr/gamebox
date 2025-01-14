//! Media track.

use std::{any::Any, sync::Arc};

use crate::Class;

use super::{
    media_block_camera_game::MediaBlockCameraGame, media_block_dof::MediaBlockDof,
    media_block_entity::MediaBlockEntity, media_block_image::MediaBlockImage,
    media_block_interface::MediaBlockInterface, media_block_text::MediaBlockText,
    media_block_trails::MediaBlockTrails, MediaBlockCameraCustom, MediaBlockCameraEffectShake,
    MediaBlockCameraPath, MediaBlockColorGrading, MediaBlockDirtyLens, MediaBlockFog,
    MediaBlockFxColors, MediaBlockManialink, MediaBlockSound, MediaBlockToneMapping,
    MediaBlockTransitionFade, MediaBlockTriangles2D, MediaBlockTriangles3D,
};

/// Media track.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaTrack {
    name: String,
    blocks: Vec<MediaBlockType>,
    is_keep_playing: bool,
    is_read_only: bool,
    is_cycling: bool,
}

impl Class for MediaTrack {
    const CLASS_ID: u32 = 0x03078000;
}

impl MediaTrack {
    /// Name of the media track.
    pub const fn name(&self) -> &String {
        &self.name
    }

    /// Media blocks of the media track.
    pub const fn blocks(&self) -> &Vec<MediaBlockType> {
        &self.blocks
    }
}

/// Media block type.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MediaBlockType {
    /// 2D triangles.
    Triangles2D(Arc<MediaBlockTriangles2D>),
    /// 3D triangles.
    Triangles3D(Arc<MediaBlockTriangles3D>),
    /// Fx colors.
    FxColors(Arc<MediaBlockFxColors>),
    /// Camera game.
    CameraGame(Arc<MediaBlockCameraGame>),
    /// Camera path.
    CameraPath(Arc<MediaBlockCameraPath>),
    /// Custom camera.
    CameraCustom(Arc<MediaBlockCameraCustom>),
    /// Camera effect shake.
    CameraEffectShake(Arc<MediaBlockCameraEffectShake>),
    /// Image.
    Image(Arc<MediaBlockImage>),
    /// Sound.
    Sound(Arc<MediaBlockSound>),
    /// Text.
    Text(Arc<MediaBlockText>),
    /// Trails
    Trails(Arc<MediaBlockTrails>),
    /// Transition fade.
    TransitionFade(Arc<MediaBlockTransitionFade>),
    /// DOF.
    Dof(Arc<MediaBlockDof>),
    /// Tone mapping.
    ToneMapping(Arc<MediaBlockToneMapping>),
    /// Manialink.
    Manialink(Arc<MediaBlockManialink>),
    /// Dirty lens.
    DirtyLens(Arc<MediaBlockDirtyLens>),
    /// Color grading.
    ColorGrading(Arc<MediaBlockColorGrading>),
    /// Interface
    Interface(Arc<MediaBlockInterface>),
    /// Fog.
    Fog(Arc<MediaBlockFog>),
    /// Entity.
    Entity(Arc<MediaBlockEntity>),
}

impl TryFrom<Arc<dyn Any + Send + Sync>> for MediaBlockType {
    type Error = ();

    fn try_from(value: Arc<dyn Any + Send + Sync>) -> Result<Self, ()> {
        value
            .downcast()
            .map(Self::CameraCustom)
            .or_else(|value| value.downcast().map(Self::CameraEffectShake))
            .or_else(|value| value.downcast().map(Self::CameraGame))
            .or_else(|value| value.downcast().map(Self::CameraPath))
            .or_else(|value| value.downcast().map(Self::ColorGrading))
            .or_else(|value| value.downcast().map(Self::DirtyLens))
            .or_else(|value| value.downcast().map(Self::Dof))
            .or_else(|value| value.downcast().map(Self::Entity))
            .or_else(|value| value.downcast().map(Self::Fog))
            .or_else(|value| value.downcast().map(Self::FxColors))
            .or_else(|value| value.downcast().map(Self::Image))
            .or_else(|value| value.downcast().map(Self::Interface))
            .or_else(|value| value.downcast().map(Self::Manialink))
            .or_else(|value| value.downcast().map(Self::Sound))
            .or_else(|value| value.downcast().map(Self::Text))
            .or_else(|value| value.downcast().map(Self::ToneMapping))
            .or_else(|value| value.downcast().map(Self::Trails))
            .or_else(|value| value.downcast().map(Self::TransitionFade))
            .or_else(|value| value.downcast().map(Self::Triangles2D))
            .or_else(|value| value.downcast().map(Self::Triangles3D))
            .map_err(|_| ())
    }
}

mod read {
    use std::{
        io::{Read, Seek},
        sync::Arc,
    };

    use crate::{
        game::ctn::{
            media_block_camera_game::MediaBlockCameraGame, media_block_dof::MediaBlockDof,
            media_block_entity::MediaBlockEntity, media_block_image::MediaBlockImage,
            media_block_interface::MediaBlockInterface, media_block_text::MediaBlockText,
            media_block_trails::MediaBlockTrails, MediaBlockCameraCustom,
            MediaBlockCameraEffectShake, MediaBlockCameraPath, MediaBlockColorGrading,
            MediaBlockDirtyLens, MediaBlockFog, MediaBlockFxColors, MediaBlockManialink,
            MediaBlockSound, MediaBlockToneMapping, MediaBlockTransitionFade,
            MediaBlockTriangles2D, MediaBlockTriangles3D,
        },
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ErrorKind, ReadBody,
        },
    };

    use super::MediaTrack;

    impl ReadBody for MediaTrack {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaTrack {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::normal(5, Self::read_chunk_5),
            ]
            .into_iter()
        }
    }

    impl MediaTrack {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.name = r.string()?;
            self.blocks = r.list_with_version(|r| {
                r.internal_node_ref_any(|r, class_id| match class_id {
                    0x0304b000 => {
                        let mut triangles_2d = MediaBlockTriangles2D::default();
                        read_body_chunks(&mut triangles_2d, r)?;

                        Ok(Arc::new(triangles_2d))
                    }
                    0x0304c000 => {
                        let mut triangles_3d = MediaBlockTriangles3D::default();
                        read_body_chunks(&mut triangles_3d, r)?;

                        Ok(Arc::new(triangles_3d))
                    }
                    0x03080000 => {
                        let mut fx_colors = MediaBlockFxColors::default();
                        read_body_chunks(&mut fx_colors, r)?;

                        Ok(Arc::new(fx_colors))
                    }
                    0x03084000 => {
                        let mut camera_game = MediaBlockCameraGame::default();
                        read_body_chunks(&mut camera_game, r)?;

                        Ok(Arc::new(camera_game))
                    }
                    0x030a1000 => {
                        let mut camera_path = MediaBlockCameraPath::default();
                        read_body_chunks(&mut camera_path, r)?;

                        Ok(Arc::new(camera_path))
                    }
                    0x030a2000 => {
                        let mut camera_custom = MediaBlockCameraCustom::default();
                        read_body_chunks(&mut camera_custom, r)?;

                        Ok(Arc::new(camera_custom))
                    }
                    0x030a4000 => {
                        let mut camera_effect_shake = MediaBlockCameraEffectShake::default();
                        read_body_chunks(&mut camera_effect_shake, r)?;

                        Ok(Arc::new(camera_effect_shake))
                    }
                    0x030a5000 => {
                        let mut image = MediaBlockImage::default();
                        read_body_chunks(&mut image, r)?;

                        Ok(Arc::new(image))
                    }
                    0x030a7000 => {
                        let mut sound = MediaBlockSound::default();
                        read_body_chunks(&mut sound, r)?;

                        Ok(Arc::new(sound))
                    }
                    0x030a8000 => {
                        let mut text = MediaBlockText::default();
                        read_body_chunks(&mut text, r)?;

                        Ok(Arc::new(text))
                    }
                    0x030a9000 => {
                        let mut trails = MediaBlockTrails::default();
                        read_body_chunks(&mut trails, r)?;

                        Ok(Arc::new(trails))
                    }
                    0x030ab000 => {
                        let mut transition_fade = MediaBlockTransitionFade::default();
                        read_body_chunks(&mut transition_fade, r)?;

                        Ok(Arc::new(transition_fade))
                    }
                    0x03126000 => {
                        let mut dof = MediaBlockDof::default();
                        read_body_chunks(&mut dof, r)?;

                        Ok(Arc::new(dof))
                    }
                    0x03127000 => {
                        let mut tone_mapping = MediaBlockToneMapping::default();
                        read_body_chunks(&mut tone_mapping, r)?;

                        Ok(Arc::new(tone_mapping))
                    }
                    0x0312a000 => {
                        let mut manialink = MediaBlockManialink::default();
                        read_body_chunks(&mut manialink, r)?;

                        Ok(Arc::new(manialink))
                    }
                    0x03165000 => {
                        let mut dirty_lens = MediaBlockDirtyLens::default();
                        read_body_chunks(&mut dirty_lens, r)?;

                        Ok(Arc::new(dirty_lens))
                    }
                    0x03186000 => {
                        let mut color_grading = MediaBlockColorGrading::default();
                        read_body_chunks(&mut color_grading, r)?;

                        Ok(Arc::new(color_grading))
                    }
                    0x03195000 => {
                        let mut interface = MediaBlockInterface::default();
                        read_body_chunks(&mut interface, r)?;

                        Ok(Arc::new(interface))
                    }
                    0x03199000 => {
                        let mut fog = MediaBlockFog::default();
                        read_body_chunks(&mut fog, r)?;

                        Ok(Arc::new(fog))
                    }
                    0x0329f000 => {
                        let mut entity = MediaBlockEntity::default();
                        read_body_chunks(&mut entity, r)?;

                        Ok(Arc::new(entity))
                    }
                    _ => Err(Error::new(ErrorKind::Unsupported(format!(
                        "{class_id:08x?}"
                    )))),
                })
            })?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            self.is_keep_playing = r.bool()?;
            self.is_read_only = r.bool()?;
            self.is_cycling = r.bool()?;
            r.f32()?;
            r.f32()?;

            Ok(())
        }
    }
}

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::{MediaBlockType, MediaTrack};

    impl WriteBody for MediaTrack {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaTrack {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [
                BodyChunk::normal(1, Self::write_chunk_1),
                BodyChunk::normal(5, Self::write_chunk_5),
            ]
            .into_iter()
        }
    }

    impl MediaTrack {
        fn write_chunk_1(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            w.string(&self.name)?;
            w.list_with_version(&self.blocks, |w, block| match block {
                MediaBlockType::CameraCustom(camera_custom) => w.internal_node_ref(camera_custom),
                MediaBlockType::CameraEffectShake(camera_effect_shake) => {
                    w.internal_node_ref(camera_effect_shake)
                }
                MediaBlockType::CameraGame(camera_game) => w.internal_node_ref(camera_game),
                MediaBlockType::CameraPath(camera_path) => w.internal_node_ref(camera_path),
                MediaBlockType::ColorGrading(color_grading) => w.internal_node_ref(color_grading),
                MediaBlockType::DirtyLens(dirty_lens) => w.internal_node_ref(dirty_lens),
                MediaBlockType::Dof(dof) => w.internal_node_ref(dof),
                MediaBlockType::Entity(entity) => w.internal_node_ref(entity),
                MediaBlockType::Fog(fog) => w.internal_node_ref(fog),
                MediaBlockType::FxColors(fx_colors) => w.internal_node_ref(fx_colors),
                MediaBlockType::Image(image) => w.internal_node_ref(image),
                MediaBlockType::Interface(interface) => w.internal_node_ref(interface),
                MediaBlockType::Manialink(manialink) => w.internal_node_ref(manialink),
                MediaBlockType::Sound(sound) => w.internal_node_ref(sound),
                MediaBlockType::Text(text) => w.internal_node_ref(text),
                MediaBlockType::ToneMapping(tone_mapping) => w.internal_node_ref(tone_mapping),
                MediaBlockType::Trails(trails) => w.internal_node_ref(trails),
                MediaBlockType::TransitionFade(transition_fade) => {
                    w.internal_node_ref(transition_fade)
                }
                MediaBlockType::Triangles2D(triangles_2d) => w.internal_node_ref(triangles_2d),
                MediaBlockType::Triangles3D(triangles_3d) => w.internal_node_ref(triangles_3d),
            })?;
            w.u32(0xffffffff)?;

            Ok(())
        }

        fn write_chunk_5(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            w.u32(1)?;
            w.bool(self.is_keep_playing)?;
            w.bool(self.is_read_only)?;
            w.bool(self.is_cycling)?;
            w.f32(-1.0)?;
            w.f32(-1.0)?;

            Ok(())
        }
    }
}
