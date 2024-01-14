//! Types used for reading and writing [Map] nodes.

pub mod media;

mod read;
mod write;

use std::rc::Rc;

use crate::{
    common::{Class, ClassId, EngineId},
    Ghost, RcStr,
};

use self::media::{MediaClip, MediaClipGroup};

/// Node type corresponding to GameBox files with the extension `Map.Gbx`.
#[derive(Default)]
pub struct Map {
    /// (Display) cost of the map.
    pub cost: u32,
    id: RcStr,
    /// Identifier of the map author.
    pub author_id: RcStr,
    name: String,
    /// Name of the map author.
    pub author_name: String,
    /// Region of the map author.
    pub author_region: String,
    params: ChallengeParameters,
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
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 67);
}

impl Map {
    /// Validation of the map.
    pub fn validation(&self) -> Option<&Validation> {
        self.params.validation.as_ref()
    }

    /// Validation of the map.
    pub fn validation_mut(&mut self) -> Option<&mut Validation> {
        self.params.validation.as_mut()
    }

    /// Set the validation of the map.
    pub fn set_validation(&mut self, validation: Option<Validation>) {
        self.params.validation = validation
    }

    /// Identifier of the map.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Name of the map.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Type of the map.
    pub fn ty(&self) -> &MapType {
        &self.params.ty
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
        self.intro_media.as_ref().map(|x| x as _)
    }

    /// Podium media clip.
    pub fn podium_media(&self) -> Option<&MediaClip> {
        self.podium_media.as_ref().map(|x| x as _)
    }

    /// In game media clip group.
    pub fn in_game_media(&self) -> Option<&MediaClipGroup> {
        self.in_game_media.as_ref().map(|x| x as _)
    }

    /// End race media clip group.
    pub fn end_race_media(&self) -> Option<&MediaClipGroup> {
        self.end_race_media.as_ref().map(|x| x as _)
    }

    /// Ambiance media clip.
    pub fn ambiance_media(&self) -> Option<&MediaClip> {
        self.ambiance_media.as_ref().map(|x| x as _)
    }

    /// Embedded objects.
    pub fn embedded_objects(&self) -> Option<&EmbeddedObjects> {
        self.embedded_objects.as_ref()
    }
}

/// Validation of a map.
pub struct Validation {
    /// Bronze medal time objective.
    pub bronze_time: u32,
    /// Silver medal time objective.
    pub silver_time: u32,
    /// Gold medal time objective.
    pub gold_time: u32,
    /// Author medal time objective.
    pub author_time: u32,
    /// Validation ghost used to set the author time.
    pub ghost: Option<Rc<Ghost>>,
}

/// Type of a map.
pub enum MapType {
    /// Normal race.
    ///
    /// Corresponds to the script `TrackMania\\TM_Race`.
    Race,
    /// Custom script.
    Script {
        /// Game path to the script file, for example `TrackMania\\TM_Royal`.
        path: String,
    },
}

impl Default for MapType {
    fn default() -> Self {
        Self::Race
    }
}

/// Block placed inside of a `Map`.
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

/// Item placed inside of a `Map`.
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

impl<T> Coord<T> {
    /// Convert to an array.
    pub fn into_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
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

/// Position.
#[derive(Default)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    /// Convert to an array.
    pub fn into_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

/// Rotation.
#[derive(Default)]
pub struct Rotation {
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl Rotation {
    /// Convert to an array.
    pub fn into_array(self) -> [f32; 3] {
        [self.yaw, self.pitch, self.roll]
    }
}

/// Element color of a block or item.
#[derive(Clone, Copy, Default)]
pub enum Color {
    /// Default.
    #[default]
    Default,
    /// White.
    White,
    /// Green.
    Green,
    /// Blue.
    Blue,
    /// Red.
    Red,
    /// Black.
    Black,
}

/// Phase offset of an animated item.
#[derive(Clone, Copy, Default)]
pub enum PhaseOffset {
    /// None.
    #[default]
    None,
    /// One eighth.
    One8th,
    /// Two eighth.
    Two8th,
    /// Three eighth.
    Three8th,
    /// Four eighth.
    Four8th,
    /// Five eighth.
    Five8th,
    /// Six eighth.
    Six8th,
    /// Seven eighth.
    Seven8th,
}

/// Lightmap quality of a block or item.
#[derive(Clone, Copy, Default)]
pub enum LightmapQuality {
    /// Normal.
    #[default]
    Normal,
    /// High.
    High,
    /// Very high.
    VeryHigh,
    /// Highest.
    Highest,
    /// Low.
    Low,
    /// Very low.
    VeryLow,
    /// Lowest.
    Lowest,
}

#[derive(Default)]
struct CollectorList;

impl Class for CollectorList {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 27);
}

#[derive(Default)]
struct ChallengeParameters {
    validation: Option<Validation>,
    ty: MapType,
    style: String,
}

impl Class for ChallengeParameters {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 91);
}
