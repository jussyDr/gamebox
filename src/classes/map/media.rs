//! Types used for reading and writing Media* nodes.

use std::{
    any::Any,
    io::{Read, Seek},
    rc::Rc,
};

use crate::{
    classes::ghost::EntRecordData,
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateRef, NodeStateRef},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
    FileRef, InternalFileRef,
};

use super::Coord;

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

impl ReadBody for MediaClipGroup {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaClipGroup {
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0307a003,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_3(n, d)),
        }]
        .into_iter()
    }
}

impl MediaClipGroup {
    fn read_chunk_3<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
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

                Ok(Coord { x, y, z })
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
    trigger_coords: Vec<Coord<u32>>,
}

impl MediaClipWithTrigger {
    /// The media clip.
    pub fn clip(&self) -> &MediaClip {
        &self.clip
    }

    /// List of coordinates where this clip is triggered if its condition is met.
    pub fn trigger_coords(&self) -> &[Coord<u32>] {
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

impl ReadBody for MediaClip {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaClip {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
    fn read_chunk_0307900d<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
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

impl ReadBody for MediaTrack {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaTrack {
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
    fn read_chunk_03078001<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.string()?;
        d.u32()?; // 10
        self.blocks = d.list(|d| {
            let node = d.any_internal_node_ref(|d, class_id| {
                let node: Rc<dyn Any> = match class_id {
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
                    0x030a8000 => {
                        let mut node = MediaBlockText;
                        MediaBlockText::read_body(&mut node, d)?;
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
                .map(MediaBlock::Triangles3D)
                .or_else(|node| node.downcast().map(MediaBlock::FxColors))
                .or_else(|node| node.downcast().map(MediaBlock::CameraGame))
                .or_else(|node| node.downcast().map(MediaBlock::CameraCustom))
                .or_else(|node| node.downcast().map(MediaBlock::CameraEffectShake))
                .or_else(|node| node.downcast().map(MediaBlock::Image))
                .or_else(|node| node.downcast().map(MediaBlock::Text))
                .or_else(|node| node.downcast().map(MediaBlock::TransitionFade))
                .or_else(|node| node.downcast().map(MediaBlock::DOF))
                .or_else(|node| node.downcast().map(MediaBlock::ToneMapping))
                .or_else(|node| node.downcast().map(MediaBlock::DirtyLens))
                .or_else(|node| node.downcast().map(MediaBlock::ColorGrading))
                .or_else(|node| node.downcast().map(MediaBlock::Fog))
                .or_else(|node| node.downcast().map(MediaBlock::Entity))
                .unwrap())
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
    Triangles3D(Rc<MediaBlockTriangles3D>),
    FxColors(Rc<MediaBlockFxColors>),
    CameraGame(Rc<MediaBlockCameraGame>),
    CameraCustom(Rc<MediaBlockCameraCustom>),
    CameraEffectShake(Rc<MediaBlockCameraEffectShake>),
    Image(Rc<MediaBlockImage>),
    Text(Rc<MediaBlockText>),
    TransitionFade(Rc<MediaBlockTransitionFade>),
    DOF(Rc<MediaBlockDOF>),
    ToneMapping(Rc<MediaBlockToneMapping>),
    DirtyLens(Rc<MediaBlockDirtyLens>),
    ColorGrading(Rc<MediaBlockColorGrading>),
    Fog(Rc<MediaBlockFog>),
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

pub struct MediaBlockTriangles3D {
    parent: MediaBlockTriangles,
}

impl ReadBody for MediaBlockTriangles3D {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockTriangles3D {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

#[derive(Default)]
pub struct MediaBlockFxColors {
    keys: Vec<MediaBlockFxColorsKey>,
}

impl ReadBody for MediaBlockFxColors {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockFxColors {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

pub struct MediaBlockCameraGame;

impl ReadBody for MediaBlockCameraGame {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockCameraGame {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

pub struct MediaBlockCameraCustom;

impl ReadBody for MediaBlockCameraCustom {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockCameraCustom {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

pub struct MediaBlockCameraEffectShake;

impl ReadBody for MediaBlockCameraEffectShake {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockCameraEffectShake {
    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

pub struct MediaBlockImage;

impl ReadBody for MediaBlockImage {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockImage {
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030a5000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockImage {
    fn read_chunk_0<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.internal_node_ref::<EffectSimi>()?;
        InternalFileRef::read(d)?;

        Ok(())
    }
}

pub struct MediaBlockText;

impl ReadBody for MediaBlockText {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockText {
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
    fn read_chunk_1<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.string()?;
        d.internal_node_ref::<EffectSimi>()?;

        Ok(())
    }

    fn read_chunk_2<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?;
        d.u32()?;
        d.u32()?;

        Ok(())
    }
}

pub struct MediaBlockTransitionFade;

impl ReadBody for MediaBlockTransitionFade {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockTransitionFade {
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x030ab000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockTransitionFade {
    fn read_chunk_0<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
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

pub struct MediaBlockDOF;

impl ReadBody for MediaBlockDOF {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockDOF {
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x03126002,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockDOF {
    fn read_chunk_2<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
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

pub struct MediaBlockToneMapping;

impl ReadBody for MediaBlockToneMapping {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockToneMapping {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

pub struct MediaBlockDirtyLens;

impl ReadBody for MediaBlockDirtyLens {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockDirtyLens {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

pub struct MediaBlockColorGrading;

impl ReadBody for MediaBlockColorGrading {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockColorGrading {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

pub struct MediaBlockFog;

impl ReadBody for MediaBlockFog {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockFog {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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

pub struct MediaBlockEntity;

impl ReadBody for MediaBlockEntity {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MediaBlockEntity {
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0329f000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl MediaBlockEntity {
    fn read_chunk_0<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 6
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

            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Default)]
struct EffectSimi;

impl Class for EffectSimi {
    const CLASS_ID: ClassId = ClassId::new(EngineId::CONTROL, 16);
}

impl ReadBody for EffectSimi {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for EffectSimi {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
