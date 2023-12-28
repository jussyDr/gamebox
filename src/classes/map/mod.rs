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
        self.id.as_str()
    }

    /// Identifier of the map author.
    pub fn author_id(&self) -> &str {
        self.author_id.as_str()
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
    id: RcStr,
    kind: BlockKind,
    color: Color,
    lightmap_quality: LightmapQuality,
}

impl Block {
    /// Identifier of the block.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Kind of block.
    pub const fn kind(&self) -> &BlockKind {
        &self.kind
    }

    pub const fn color(&self) -> Color {
        self.color
    }

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
    coordinate: Coordinate,
    is_ghost: bool,
}

impl NormalBlock {
    /// Cardinal direction of the block.
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    /// Coordinate of the block.
    pub const fn coordinate(&self) -> Coordinate {
        self.coordinate
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
        self.id.as_str()
    }

    pub const fn color(&self) -> Color {
        self.color
    }

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

#[derive(Clone, Copy)]
pub struct Coordinate {
    x: u8,
    y: u8,
    z: u8,
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

#[derive(Default)]
pub struct MediaClipGroup;

impl Class for MediaClipGroup {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x07a;
}

#[derive(Default)]
pub struct MediaClip;

impl Class for MediaClip {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x079;
}
