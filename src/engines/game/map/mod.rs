//! Types used for reading and writing [Map] nodes.

pub mod media;

mod read;
mod write;

use std::rc::Rc;

use crate::{
    common::{Class, ClassId, EngineId, Vec3},
    RcStr,
};

use self::media::{MediaClip, MediaClipGroup};

use super::ghost::Ghost;

/// A map / challenge.
///
/// Corresponds to the file extension `Map.Gbx` or `Challenge.Gbx`.
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
    thumbnail: Vec<u8>,
    comments: String,
    size: Vec3<u32>,
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

impl Default for Map {
    fn default() -> Self {
        Self {
            cost: 0,
            id: RcStr::default(),
            author_id: RcStr::default(),
            name: String::new(),
            author_name: String::new(),
            author_region: String::default(),
            params: ChallengeParameters::default(),
            thumbnail: vec![],
            comments: String::new(),
            size: Vec3 {
                x: 48,
                y: 40,
                z: 48,
            },
            blocks: vec![],
            items: vec![],
            baked_blocks: vec![],
            intro_media: None,
            podium_media: None,
            in_game_media: None,
            end_race_media: None,
            ambiance_media: None,
            embedded_objects: None,
        }
    }
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

/// Block placed inside of a map.
pub struct Block {
    id: Rc<str>,
    kind: BlockKind,
    elem_color: ElemColor,
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
    pub const fn elem_color(&self) -> ElemColor {
        self.elem_color
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
    coord: Vec3<u8>,
    is_ground: bool,
    is_ghost: bool,
}

impl NormalBlock {
    /// Cardinal direction of the block.
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    /// Coordinate of the block.
    pub const fn coord(&self) -> Vec3<u8> {
        self.coord
    }

    /// Returns `true` if this block is a ground block.
    pub const fn is_ground(&self) -> bool {
        self.is_ground
    }

    /// Returns `true` if this block is a ghost block.
    pub const fn is_ghost(&self) -> bool {
        self.is_ghost
    }
}

/// A free block.
#[derive(Default)]
pub struct FreeBlock {
    position: Vec3<f32>,
    rotation: Rotation,
}

impl FreeBlock {
    /// Position of the free block.
    pub const fn position(&self) -> &Vec3<f32> {
        &self.position
    }

    /// Rotation of the free block.
    pub const fn rotation(&self) -> &Rotation {
        &self.rotation
    }
}

/// Item placed inside of a map.
#[derive(Default)]
pub struct Item {
    id: RcStr,
    rotation: Rotation,
    position: Vec3<f32>,
    pivot_position: Vec3<f32>,
    elem_color: ElemColor,
    animation_offset: PhaseOffset,
}

impl Item {
    /// Identifier of the item.
    pub fn id(&self) -> &str {
        &self.id
    }

    pub const fn rotation(&self) -> &Rotation {
        &self.rotation
    }

    pub const fn position(&self) -> &Vec3<f32> {
        &self.position
    }

    pub const fn pivot_position(&self) -> &Vec3<f32> {
        &self.pivot_position
    }

    /// Element color of the item.
    pub const fn elem_color(&self) -> ElemColor {
        self.elem_color
    }

    /// Animation phase offset of the item.
    pub const fn animation_offset(&self) -> PhaseOffset {
        self.animation_offset
    }
}

/// Cardinal direction.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// Objects embedded in a map.
pub struct EmbeddedObjects {
    ids: Vec<Rc<str>>,
    data: Vec<u8>,
}

impl EmbeddedObjects {
    /// Identifiers of the embedded objects.
    ///
    /// The identifier at the i-th index corresponds to the i-th file in the ZIP archive.
    pub fn ids(&self) -> &[Rc<str>] {
        &self.ids
    }

    /// Embedded object data encoded as a ZIP archive.
    pub fn data(&self) -> &[u8] {
        &self.data
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
    pub const fn as_array(&self) -> [f32; 3] {
        [self.yaw, self.pitch, self.roll]
    }
}

/// Element color of a block or item.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElemColor {
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
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
