//! Types used for reading and writing [Map] nodes.

mod read;

use std::rc::Rc;

use crate::{class::Class, EngineId, RcStr};

/// Node type corresponding to GameBox files with the extension `Map.Gbx`.
#[derive(Default)]
pub struct Map {
    id: RcStr,
    author_id: RcStr,
    name: String,
    author_name: String,
    author_region: String,
    blocks: Vec<Block>,
    items: Vec<Item>,
    baked_blocks: Vec<Block>,
    intro_media: Option<Rc<MediaClip>>,
    podium_media: Option<Rc<MediaClip>>,
    in_game_media: Option<Rc<MediaClipGroup>>,
    end_race_media: Option<Rc<MediaClipGroup>>,
    ambiance_media: Option<Rc<MediaClip>>,
    embedded_objects: Option<EmbeddedObjects>,
}

impl Class for Map {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x043;
}

impl Map {
    /// Identifier of the map.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Identifier of the map author.
    pub fn author_id(&self) -> &str {
        &self.author_id
    }

    /// Name of the map.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Name of the map author.
    pub fn author_name(&self) -> &str {
        &self.author_name
    }

    /// Region of the map author.
    pub fn author_region(&self) -> &str {
        &self.author_region
    }

    /// List of blocks placed inside of this map.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// List of items placed inside of this map.
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    /// Intro media clip.
    pub fn intro_media(&self) -> Option<&MediaClip> {
        match self.intro_media {
            None => None,
            Some(ref intro_media) => Some(intro_media),
        }
    }

    /// Podium media clip.
    pub fn podium_media(&self) -> Option<&MediaClip> {
        match self.podium_media {
            None => None,
            Some(ref podium_media) => Some(podium_media),
        }
    }

    /// In game media clip group.
    pub fn in_game_media(&self) -> Option<&MediaClipGroup> {
        match self.in_game_media {
            None => None,
            Some(ref in_game_media) => Some(in_game_media),
        }
    }

    /// End race media clip group.
    pub fn end_race_media(&self) -> Option<&MediaClipGroup> {
        match self.end_race_media {
            None => None,
            Some(ref end_race_media) => Some(end_race_media),
        }
    }

    /// Ambiance media clip.
    pub fn ambiance_media(&self) -> Option<&MediaClip> {
        match self.ambiance_media {
            None => None,
            Some(ref ambiance_media) => Some(ambiance_media),
        }
    }

    /// Embedded objects.
    pub fn embedded_objects(&self) -> Option<&EmbeddedObjects> {
        self.embedded_objects.as_ref()
    }
}

/// Block placed inside of a [Map].
pub struct Block {
    id: Rc<str>,
    kind: BlockKind,
    color: Color,
    lightmap_quality: LightmapQuality,
}

impl Block {
    /// Identifier of the block.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Kind of block.
    pub const fn kind(&self) -> &BlockKind {
        &self.kind
    }

    /// Element color of the block.
    pub const fn color(&self) -> Color {
        self.color
    }

    /// Lightmap quality of the block.
    pub const fn lightmap_quality(&self) -> LightmapQuality {
        self.lightmap_quality
    }
}

/// Either a normal or a free block.
pub enum BlockKind {
    /// A normal block.
    Normal(NormalBlock),
    /// A free block.
    Free(FreeBlock),
}

/// A normal block.
pub struct NormalBlock {
    direction: Direction,
    coord: Coord<u8>,
    is_ghost: bool,
}

impl NormalBlock {
    /// Cardinal direction of the block.
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    /// Coordinate of the block.
    pub const fn coord(&self) -> Coord<u8> {
        self.coord
    }

    /// Returns `true` if this block is a ghost block.
    pub const fn is_ghost_block(&self) -> bool {
        self.is_ghost
    }
}

/// A free block.
#[derive(Default)]
pub struct FreeBlock {
    position: Position,
    rotation: Rotation,
}

impl FreeBlock {
    /// Position of the free block.
    pub const fn position(&self) -> &Position {
        &self.position
    }

    /// Rotation of the free block.
    pub const fn rotation(&self) -> &Rotation {
        &self.rotation
    }
}

/// Item placed inside of a [Map].
#[derive(Default)]
pub struct Item {
    id: RcStr,
    color: Color,
    animation_offset: PhaseOffset,
}

impl Item {
    /// Identifier of the item.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Element color of the item.
    pub const fn color(&self) -> Color {
        self.color
    }

    /// Animation phase offset of the item.
    pub const fn animation_offset(&self) -> PhaseOffset {
        self.animation_offset
    }
}

/// Cardinal direction.
#[derive(Clone, Copy)]
pub enum Direction {
    /// North.
    North,
    /// East.
    East,
    /// South.
    South,
    /// West.
    West,
}

/// A coordinate.
#[derive(Clone, Copy)]
pub struct Coord<T> {
    x: T,
    y: T,
    z: T,
}

/// Objects embedded in a map.
pub struct EmbeddedObjects {
    object_ids: Vec<RcStr>,
    data: Vec<u8>,
}

impl EmbeddedObjects {
    /// Identifiers of the embedded objects.
    pub fn object_ids(&self) -> &[RcStr] {
        &self.object_ids
    }

    /// Embedded object data encoded as a ZIP archive.
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Default)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default)]
pub struct Rotation {
    yaw: f32,
    pitch: f32,
    roll: f32,
}

#[derive(Clone, Copy, Default)]
pub enum Color {
    #[default]
    Default,
    White,
    Green,
    Blue,
    Red,
    Black,
}

/// Phase offset.
#[derive(Clone, Copy, Default)]
pub enum PhaseOffset {
    #[default]
    None,
    One8th,
    Two8th,
    Three8th,
    Four8th,
    Five8th,
    Six8th,
    Seven8th,
}

/// Lightmap quality of a block or item.
#[derive(Clone, Copy, Default)]
pub enum LightmapQuality {
    #[default]
    Normal,
    High,
    VeryHigh,
    Highest,
    Low,
    VeryLow,
    Lowest,
}

/// A group of media clips.
#[derive(Default)]
pub struct MediaClipGroup {
    clips: Vec<MediaClipWithTrigger>,
}

impl Class for MediaClipGroup {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x07a;
}

impl MediaClipGroup {
    /// The media clips in this group with their corresponding triggers.
    pub fn clips(&self) -> &[MediaClipWithTrigger] {
        &self.clips
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
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x079;
}

impl MediaClip {
    /// Media tracks in this clip.
    pub fn tracks(&self) -> &[Rc<MediaTrack>] {
        &self.tracks
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
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x078;
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

pub struct MediaBlockTriangles3D {
    parent: MediaBlockTriangles,
}

pub struct MediaBlockFxColors;

pub struct MediaBlockCameraGame;

pub struct MediaBlockCameraCustom;

pub struct MediaBlockCameraEffectShake;

pub struct MediaBlockImage;

pub struct MediaBlockText;

pub struct MediaBlockTransitionFade;

pub struct MediaBlockDOF;

pub struct MediaBlockToneMapping;

pub struct MediaBlockDirtyLens;

pub struct MediaBlockColorGrading;

pub struct MediaBlockFog;

pub struct MediaBlockEntity;
