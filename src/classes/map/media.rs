//! Types used for reading and writing Media* nodes.

use std::{any::Any, io::Read, rc::Rc};

use crate::{
    classes::ent_record_data::EntRecordData,
    common::{Class, ClassId, EngineId, Vec3},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
    ExternalFileRef, FileRef, InternalFileRef,
};

/// A group of media clips.
#[derive(Default)]
pub struct MediaClipGroup {
    clips: Vec<MediaClipWithTrigger>,
}

impl Class for MediaClipGroup {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 122);
}

impl MediaClipGroup {
    /// The media clips in this group with their corresponding triggers.
    pub fn clips(&self) -> &[MediaClipWithTrigger] {
        &self.clips
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for MediaClipGroup {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for MediaClipGroup {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0307a003,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_3(n, d)),
        }]
        .into_iter()
    }
}

impl MediaClipGroup {
    fn read_chunk_3<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 10
        let clips = d.list(|d| d.internal_node_ref::<MediaClip>())?;
        self.clips = d.list_zipped_with(clips, |d, clip| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            let trigger_coords = d.list(|d| {
                let x = d.u32()?;
                let y = d.u32()?;
                let z = d.u32()?;

                Ok(Vec3 { x, y, z })
            })?;

            Ok(MediaClipWithTrigger {
                clip,
                trigger_coords,
            })
        })?;

        Ok(())
    }
}

/// A media clip and its corresponding trigger.
pub struct MediaClipWithTrigger {
    clip: Rc<MediaClip>,
    trigger_coords: Vec<Vec3<u32>>,
}

impl MediaClipWithTrigger {
    /// The media clip.
    pub fn clip(&self) -> &MediaClip {
        &self.clip
    }

    /// List of coordinates where this clip is triggered if its condition is met.
    pub fn trigger_coords(&self) -> &[Vec3<u32>] {
        &self.trigger_coords
    }
}

/// A media clip.
#[derive(Default)]
pub struct MediaClip {
    tracks: Vec<Rc<MediaTrack>>,
}

impl Class for MediaClip {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 121);
}

impl MediaClip {
    /// Media tracks in this clip.
    pub fn tracks(&self) -> &[Rc<MediaTrack>] {
        &self.tracks
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for MediaClip {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for MediaClip {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x0307900d,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0307900d(n, d)),
            },
            BodyChunkEntry {
                id: 0x0307900e,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0307900e(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MediaClip {
    fn read_chunk_0307900d<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 10
        self.tracks = d.list(|d| d.internal_node_ref::<MediaTrack>())?;
        d.string()?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?;
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0307900e<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0

        Ok(())
    }
}

/// A media track.
#[derive(Default)]
pub struct MediaTrack {
    blocks: Vec<MediaBlock>,
}

impl MediaTrack {
    /// Media blocks in this track.
    pub fn blocks(&self) -> &[MediaBlock] {
        &self.blocks
    }
}

impl Class for MediaTrack {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 120);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for MediaTrack {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for MediaTrack {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03078001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_03078001(n, d)),
            },
            BodyChunkEntry {
                id: 0x03078005,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_5(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MediaTrack {
    fn read_chunk_03078001<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.string()?;
        d.u32()?; // 10
        self.blocks = d.list(|d| {
            let node = d.any_internal_node_ref(|d, class_id| {
                let node: Rc<dyn Any> = match class_id {
                    0x0304b000 => {
                        let mut node = MediaBlockTriangles2D {
                            parent: MediaBlockTriangles,
                        };
                        MediaBlockTriangles2D::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x0304c000 => {
                        let mut node = MediaBlockTriangles3D {
                            parent: MediaBlockTriangles,
                        };
                        MediaBlockTriangles3D::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03080000 => {
                        let mut node = MediaBlockFxColors::default();
                        MediaBlockFxColors::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03084000 => {
                        let mut node = MediaBlockCameraGame;
                        MediaBlockCameraGame::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03085000 => {
                        let mut node = MediaBlockTime;
                        MediaBlockTime::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x030a2000 => {
                        let mut node = MediaBlockCameraCustom;
                        MediaBlockCameraCustom::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x030a4000 => {
                        let mut node = MediaBlockCameraEffectShake;
                        MediaBlockCameraEffectShake::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x030a5000 => {
                        let mut node = MediaBlockImage;
                        MediaBlockImage::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x030a6000 => {
                        let mut node = MediaBlockMusicEffect;
                        MediaBlockMusicEffect::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x030a7000 => {
                        let mut node = MediaBlockSound;
                        MediaBlockSound::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x030a8000 => {
                        let mut node = MediaBlockText;
                        MediaBlockText::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x030a9000 => {
                        let mut node = MediaBlockTrails;
                        MediaBlockTrails::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x030ab000 => {
                        let mut node = MediaBlockTransitionFade;
                        MediaBlockTransitionFade::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03126000 => {
                        let mut node = MediaBlockDOF;
                        MediaBlockDOF::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03127000 => {
                        let mut node = MediaBlockToneMapping;
                        MediaBlockToneMapping::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03128000 => {
                        let mut node = MediaBlockBloomHdr;
                        MediaBlockBloomHdr::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03129000 => {
                        let mut node = MediaBlockTimeSpeed;
                        MediaBlockTimeSpeed::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03133000 => {
                        let mut node = MediaBlockVehicleLight;
                        MediaBlockVehicleLight::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03145000 => {
                        let mut node = MediaBlockShoot;
                        MediaBlockShoot::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03165000 => {
                        let mut node = MediaBlockDirtyLens;
                        MediaBlockDirtyLens::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03186000 => {
                        let mut node = MediaBlockColorGrading;
                        MediaBlockColorGrading::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03195000 => {
                        let mut node = MediaBlockInterface;
                        MediaBlockInterface::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x03199000 => {
                        let mut node = MediaBlockFog;
                        MediaBlockFog::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    0x0329f000 => {
                        let mut node = MediaBlockEntity;
                        MediaBlockEntity::read_body(&mut node, d)?;
                        Rc::new(node)
                    }
                    _ => return Err("unknown media block".into()),
                };

                Ok(node)
            })?;

            Ok(node
                .downcast()
                .map(MediaBlock::Triangles2D)
                .or_else(|node| node.downcast().map(MediaBlock::Triangles3D))
                .or_else(|node| node.downcast().map(MediaBlock::FxColors))
                .or_else(|node| node.downcast().map(MediaBlock::CameraGame))
                .or_else(|node| node.downcast().map(MediaBlock::Time))
                .or_else(|node| node.downcast().map(MediaBlock::CameraCustom))
                .or_else(|node| node.downcast().map(MediaBlock::CameraEffectShake))
                .or_else(|node| node.downcast().map(MediaBlock::Image))
                .or_else(|node| node.downcast().map(MediaBlock::MusicEffect))
                .or_else(|node| node.downcast().map(MediaBlock::Sound))
                .or_else(|node| node.downcast().map(MediaBlock::Text))
                .or_else(|node| node.downcast().map(MediaBlock::Trails))
                .or_else(|node| node.downcast().map(MediaBlock::TransitionFade))
                .or_else(|node| node.downcast().map(MediaBlock::DOF))
                .or_else(|node| node.downcast().map(MediaBlock::ToneMapping))
                .or_else(|node| node.downcast().map(MediaBlock::BloomHdr))
                .or_else(|node| node.downcast().map(MediaBlock::TimeSpeed))
                .or_else(|node| node.downcast().map(MediaBlock::VehicleLight))
                .or_else(|node| node.downcast().map(MediaBlock::Shoot))
                .or_else(|node| node.downcast().map(MediaBlock::DirtyLens))
                .or_else(|node| node.downcast().map(MediaBlock::ColorGrading))
                .or_else(|node| node.downcast().map(MediaBlock::Interface))
                .or_else(|node| node.downcast().map(MediaBlock::Fog))
                .or_else(|node| node.downcast().map(MediaBlock::Entity))
                .map_err(|_| "")?)
        })?;
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_5<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // -1
        d.f32()?; // -1

        Ok(())
    }
}

/// A media block.
pub enum MediaBlock {
    /// 2D triangles media block.
    Triangles2D(Rc<MediaBlockTriangles2D>),
    /// 3D triangles media block.
    Triangles3D(Rc<MediaBlockTriangles3D>),
    /// Colors FX media block.
    FxColors(Rc<MediaBlockFxColors>),
    /// Player camera media block.
    CameraGame(Rc<MediaBlockCameraGame>),
    /// Time media block.
    Time(Rc<MediaBlockTime>),
    /// Custom camera media block.
    CameraCustom(Rc<MediaBlockCameraCustom>),
    /// Shake cam FX media block.
    CameraEffectShake(Rc<MediaBlockCameraEffectShake>),
    /// Image media block.
    Image(Rc<MediaBlockImage>),
    /// Music volume media block.
    MusicEffect(Rc<MediaBlockMusicEffect>),
    /// Sound FX media block.
    Sound(Rc<MediaBlockSound>),
    /// Text media block.
    Text(Rc<MediaBlockText>),
    /// Car trails media block.
    Trails(Rc<MediaBlockTrails>),
    /// Transition fade media block.
    TransitionFade(Rc<MediaBlockTransitionFade>),
    /// Depth of field media block.
    DOF(Rc<MediaBlockDOF>),
    /// Tone mapping media block.
    ToneMapping(Rc<MediaBlockToneMapping>),
    /// HDR bloom media block.
    BloomHdr(Rc<MediaBlockBloomHdr>),
    /// Time speed media block.
    TimeSpeed(Rc<MediaBlockTimeSpeed>),
    /// Vehicle lights media block.
    VehicleLight(Rc<MediaBlockVehicleLight>),
    /// Editing cut media block.
    Shoot(Rc<MediaBlockShoot>),
    /// Dirty lens media block.
    DirtyLens(Rc<MediaBlockDirtyLens>),
    /// Color grading media block.
    ColorGrading(Rc<MediaBlockColorGrading>),
    /// ManiaLink UI media block.
    Interface(Rc<MediaBlockInterface>),
    /// Fog media block.
    Fog(Rc<MediaBlockFog>),
    /// Ghost media block.
    Entity(Rc<MediaBlockEntity>),
}

struct MediaBlockTriangles;

impl MediaBlockTriangles {
    fn read_chunk_1<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;
        let num_keys = d.u32()?;
        let num_vertices = d.u32()?;
        d.repeat(num_keys as usize * num_vertices as usize, |d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }

    fn read_chunk_2<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;

        Ok(())
    }
}

/// 2D triangles media block.
pub struct MediaBlockTriangles2D {
    parent: MediaBlockTriangles,
}

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockTriangles2D {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockTriangles2D {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03029001,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    MediaBlockTriangles::read_chunk_1(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x03029002,
                read_fn: BodyChunkReadFn::Skippable(|n: &mut Self, d| {
                    MediaBlockTriangles::read_chunk_2(&mut n.parent, d)
                }),
            },
        ]
        .into_iter()
    }
}

/// 3D triangles media block.
pub struct MediaBlockTriangles3D {
    parent: MediaBlockTriangles,
}

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockTriangles3D {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockTriangles3D {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03029001,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    MediaBlockTriangles::read_chunk_1(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x03029002,
                read_fn: BodyChunkReadFn::Skippable(|n: &mut Self, d| {
                    MediaBlockTriangles::read_chunk_2(&mut n.parent, d)
                }),
            },
        ]
        .into_iter()
    }
}

/// Colors FX media block.
#[derive(Default)]
pub struct MediaBlockFxColors {
    keys: Vec<MediaBlockFxColorsKey>,
}

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockFxColors {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockFxColors {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03080003,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_3(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockFxColors {
    fn read_chunk_3<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        self.keys = d.list(|d| {
            d.u32()?;
            let global_intensity = d.f32()?;
            let far_blend = d.f32()?;
            let near_distance = d.f32()?;
            let far_distance = d.f32()?;
            let near_inverse = d.f32()?;
            let near_hue = d.f32()?;
            let near_saturation = d.f32()?;
            let near_brightness = d.f32()?;
            let near_contrast = d.f32()?;
            let near_red = d.f32()?;
            let near_green = d.f32()?;
            let near_blue = d.f32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            let far_inverse = d.f32()?;
            let far_hue = d.f32()?;
            let far_saturation = d.f32()?;
            let far_brightness = d.f32()?;
            let far_contrast = d.f32()?;
            let far_red = d.f32()?;
            let far_green = d.f32()?;
            let far_blue = d.f32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(MediaBlockFxColorsKey {
                global_intensity,
                far_blend,
                near_distance,
                far_distance,
                near_inverse,
                near_hue,
                near_saturation,
                near_brightness,
                near_contrast,
                near_red,
                near_green,
                near_blue,
                far_inverse,
                far_hue,
                far_saturation,
                far_brightness,
                far_contrast,
                far_red,
                far_green,
                far_blue,
            })
        })?;

        Ok(())
    }
}

/// Colors FX media block key.
pub struct MediaBlockFxColorsKey {
    global_intensity: f32,
    /// [0.0, 1.0].
    far_blend: f32,
    near_distance: f32,
    far_distance: f32,
    /// [0.0, 1.0].
    near_inverse: f32,
    /// [0.0, 1.0].
    near_hue: f32,
    /// [-1.0, 1.0].
    near_saturation: f32,
    /// [-1.0, 1.0].
    near_brightness: f32,
    /// [-1.0, 1.0].
    near_contrast: f32,
    /// [0.0, 1.0].
    near_red: f32,
    /// [0.0, 1.0].
    near_green: f32,
    /// [0.0, 1.0].
    near_blue: f32,
    /// [0.0, 1.0].
    far_inverse: f32,
    /// [0.0, 1.0].
    far_hue: f32,
    /// [-1.0, 1.0].
    far_saturation: f32,
    /// [-1.0, 1.0].
    far_brightness: f32,
    /// [-1.0, 1.0].
    far_contrast: f32,
    /// [0.0, 1.0].
    far_red: f32,
    /// [0.0, 1.0].
    far_green: f32,
    /// [0.0, 1.0].
    far_blue: f32,
}

impl MediaBlockFxColorsKey {
    /// [0.0, 1.0].
    pub fn global_intensity(&self) -> f32 {
        self.global_intensity
    }
}

/// Player camera media block.
pub struct MediaBlockCameraGame;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockCameraGame {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockCameraGame {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03084007,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_7(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockCameraGame {
    fn read_chunk_7<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 4
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }
}

/// Time media block.
pub struct MediaBlockTime;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockTime {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockTime {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03085000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockTime {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Custom camera media block.
pub struct MediaBlockCameraCustom;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockCameraCustom {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockCameraCustom {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030a2006,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_6(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockCameraCustom {
    fn read_chunk_6<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 3
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Shake cam FX media block.
pub struct MediaBlockCameraEffectShake;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockCameraEffectShake {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockCameraEffectShake {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030a4000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockCameraEffectShake {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Image media block.
pub struct MediaBlockImage;

impl<R: Read, I, N: NodeStateMut> ReadBody<R, I, N> for MediaBlockImage {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N: NodeStateMut> BodyChunks<R, I, N> for MediaBlockImage {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030a5000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockImage {
    fn read_chunk_0<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.internal_node_ref::<EffectSimi>()?;
        FileRef::read(d)?;

        Ok(())
    }
}

/// Music volume media block.
pub struct MediaBlockMusicEffect;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockMusicEffect {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockMusicEffect {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030a6001,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_1(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockMusicEffect {
    fn read_chunk_1<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Sound FX media block.
pub struct MediaBlockSound;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockSound {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockSound {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x030a7003,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_3(n, d)),
            },
            BodyChunkEntry {
                id: 0x030a7004,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_4(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MediaBlockSound {
    fn read_chunk_3<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }

    fn read_chunk_4<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        ExternalFileRef::read(d)?;
        d.u32()?; // 1
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Text media block.
pub struct MediaBlockText;

impl<R: Read, I, N: NodeStateMut> ReadBody<R, I, N> for MediaBlockText {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N: NodeStateMut> BodyChunks<R, I, N> for MediaBlockText {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x030a8001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_1(n, d)),
            },
            BodyChunkEntry {
                id: 0x030a8002,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MediaBlockText {
    fn read_chunk_1<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.string()?;
        d.internal_node_ref::<EffectSimi>()?;

        Ok(())
    }

    fn read_chunk_2<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }
}

/// Car trails media block.
pub struct MediaBlockTrails;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockTrails {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockTrails {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030a9000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockTrails {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?;

        Ok(())
    }
}

/// Transition fade media block.
pub struct MediaBlockTransitionFade;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockTransitionFade {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockTransitionFade {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030ab000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockTransitionFade {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }
}

/// Depth of field media block.
pub struct MediaBlockDOF;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockDOF {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockDOF {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03126002,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockDOF {
    fn read_chunk_2<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Tone mapping media block.
pub struct MediaBlockToneMapping;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockToneMapping {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockToneMapping {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03127004,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_4(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockToneMapping {
    fn read_chunk_4<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// HDR bloom media block.
pub struct MediaBlockBloomHdr;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockBloomHdr {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockBloomHdr {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03128002,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockBloomHdr {
    fn read_chunk_2<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Time speed media block.
pub struct MediaBlockTimeSpeed;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockTimeSpeed {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockTimeSpeed {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03129000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockTimeSpeed {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Vehicle lights media block.
pub struct MediaBlockVehicleLight;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockVehicleLight {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockVehicleLight {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03133000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
            },
            BodyChunkEntry {
                id: 0x03133001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_1(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MediaBlockVehicleLight {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?;

        Ok(())
    }

    fn read_chunk_1<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;

        Ok(())
    }
}

/// Editing cut media block.
pub struct MediaBlockShoot;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockShoot {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockShoot {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03145000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockShoot {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?;

        Ok(())
    }
}

/// Dirty lens media block.
pub struct MediaBlockDirtyLens;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockDirtyLens {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockDirtyLens {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03165000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockDirtyLens {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Color grading media block.
pub struct MediaBlockColorGrading;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockColorGrading {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockColorGrading {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x03186000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
            },
            BodyChunkEntry {
                id: 0x03186001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_1(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MediaBlockColorGrading {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        InternalFileRef::read(d)?;

        Ok(())
    }

    fn read_chunk_1<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// ManiaLink UI media block.
pub struct MediaBlockInterface;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockInterface {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockInterface {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03195000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockInterface {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.string()?;

        Ok(())
    }
}

/// Fog media block.
pub struct MediaBlockFog;

impl<R: Read, I, N> ReadBody<R, I, N> for MediaBlockFog {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for MediaBlockFog {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03199000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockFog {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

/// Ghost media block.
pub struct MediaBlockEntity;

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for MediaBlockEntity {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for MediaBlockEntity {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0329f000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockEntity {
    fn read_chunk_0<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?; // 6 | 9
        d.internal_node_ref::<EntRecordData>()?;
        d.u32()?; // 0
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.id()?;
        d.u32()?;
        d.id()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.list(|d| {
            FileRef::read(d)?;

            Ok(())
        })?;
        d.u32()?;
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            if version >= 9 {
                d.u32()?;
            }

            Ok(())
        })?;
        if version >= 7 {
            d.string()?; // "Guide"

            if version >= 8 {
                d.u32()?;
            }
        }

        Ok(())
    }
}

#[derive(Default)]
struct EffectSimi;

impl Class for EffectSimi {
    const CLASS_ID: ClassId = ClassId::new(EngineId::CONTROL, 16);
}

impl<R: Read, I, N> ReadBody<R, I, N> for EffectSimi {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for EffectSimi {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x07010005,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_5(n, d)),
        }]
        .into_iter()
    }
}

impl EffectSimi {
    fn read_chunk_5<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }
}
