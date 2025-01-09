#![deny(unsafe_code)]
#![warn(
    missing_docs,
    clippy::unwrap_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::todo
)]

//! GameBox file reading and writing.
//!
//! #### Common files
//!
//! | Extension | Class | Readable | Writable |
//! | --- | --- | --- | --- |
//! | `Block.Gbx` | [game::ItemModel] | [x] | [ ] |
//! | `Item.Gbx` | [game::ItemModel] | [x] | [ ] |
//! | `Map.Gbx` | [game::ctn::Challenge] | [x] | [x] |
//!
//! # Complete file list
//!
//! | Extension | Class | Readable | Writable |
//! | --- | --- | --- | --- |
//! | `Block.Gbx` | [game::ItemModel] | [x] | [ ] |
//! | `Challenge.Gbx` | [game::ctn::Challenge] | [x] | [x] |
//! | `Collection.Gbx` | [game::ctn::Collection] | [x] | [ ] |
//! | `Decoration.Gbx` | [game::ctn::Decoration] | [x] | [ ] |
//! | `DecorationMood.Gbx` | [game::ctn::DecorationMood] | [x] | [ ] |
//! | `EDClassic.Gbx` | [game::ctn::BlockInfoClassic] | [x] | [ ] |
//! | `EDClip.Gbx` | [game::ctn::BlockInfoClip] | [x] | [ ] |
//! | `EDFlat.Gbx` | [game::ctn::BlockInfoFlat] | [x] | [ ] |
//! | `EDHorizontalClip.Gbx` | [game::ctn::BlockInfoClipHorizontal] | [x] | [ ] |
//! | `EDVerticalClip.Gbx` | [game::ctn::BlockInfoClipVertical] | [x] | [ ] |
//! | `Item.Gbx` | [game::ItemModel] | [x] | [ ] |
//! | `Map.Gbx` | [game::ctn::Challenge] | [x] | [x] |
//! | `Material.Gbx` | [plug::Material] | [x] | [ ] |
//! | `Mesh.Gbx` | [plug::Solid2Model] | [x] | [ ] |
//! | `Prefab.Gbx` | [plug::Prefab] | [x] | [ ] |
//! | `StaticObject.Gbx` | [plug::StaticObjectModel] | [x] | [ ] |
//! | `Texture.Gbx` | [plug::Bitmap] | [x] | [ ] |
//! | `Title.Gbx` | [game::ManiaTitle] | [x] | [ ] |
//! | `VegetTreeModel.Gbx` | [plug::VegetTreeModel] | [x] | [ ] |

pub mod control;
pub mod function;
pub mod game;
pub mod plug;
pub mod read;
pub mod script;
pub mod write;

mod node_ref;

use bytemuck::{Pod, Zeroable};
#[doc(inline)]
pub use read::{read, read_file};
#[doc(inline)]
pub use write::{write, write_file};

pub use node_ref::{ExternalNodeRef, NodeRef};

use gamebox_macros::{FromLe, ToLe};
use std::path::{Path, PathBuf};

/// Byte3.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Byte3 {
    /// X.
    pub x: u8,
    /// Y.
    pub y: u8,
    /// Z.
    pub z: u8,
}

impl Byte3 {
    /// New.
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }

    /// To array.
    pub const fn to_array(self) -> [u8; 3] {
        [self.x, self.y, self.z]
    }
}

/// Nat3.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Nat3 {
    /// X.
    pub x: u32,
    /// Y.
    pub y: u32,
    /// Z.
    pub z: u32,
}

impl Nat3 {
    /// New.
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    /// To array.
    pub const fn to_array(self) -> [u32; 3] {
        [self.x, self.y, self.z]
    }
}

/// Int2.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Int2 {
    /// X.
    pub x: i32,
    /// Y.
    pub y: i32,
}

impl Int2 {
    /// New.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// To array.
    pub const fn to_array(self) -> [i32; 2] {
        [self.x, self.y]
    }
}

/// Int3.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Int3 {
    /// X.
    pub x: i32,
    /// Y.
    pub y: i32,
    /// Z.
    pub z: i32,
}

impl Int3 {
    /// New.
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// To array.
    pub const fn to_array(self) -> [i32; 3] {
        [self.x, self.y, self.z]
    }
}

/// 2-dimensional vector.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Vec2 {
    /// X.
    pub x: f32,
    /// Y.
    pub y: f32,
}

impl Vec2 {
    /// New.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// To array.
    pub const fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

/// 3-dimensional vector.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Vec3 {
    /// X.
    pub x: f32,
    /// Y.
    pub y: f32,
    /// Z.
    pub z: f32,
}

impl Vec3 {
    /// New.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// To array.
    pub const fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

/// Rgba.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Rgba {
    /// Red.
    pub r: u8,
    /// Green.
    pub g: u8,
    /// Blue.
    pub b: u8,
    /// Alpha.
    pub a: u8,
}

impl Rgba {
    /// New.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// To array.
    pub const fn to_array(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

/// Rgb.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct RgbNat {
    /// Red.
    pub r: u32,
    /// Green.
    pub g: u32,
    /// Blue.
    pub b: u32,
}

impl RgbNat {
    /// New.
    pub const fn new(r: u32, g: u32, b: u32) -> Self {
        Self { r, g, b }
    }

    /// To array.
    pub const fn to_array(self) -> [u32; 3] {
        [self.r, self.g, self.b]
    }
}

/// Rgb.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct RgbFloat {
    /// Red.
    pub r: f32,
    /// Green.
    pub g: f32,
    /// Blue.
    pub b: f32,
}

impl RgbFloat {
    /// New.
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// To array.
    pub const fn to_array(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

/// Rotation represented as yaw, pitch, and roll angles.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct YawPitchRoll {
    /// Yaw angle.
    pub yaw: f32,
    /// Yaw angle.
    pub pitch: f32,
    /// Roll angle.
    pub roll: f32,
}

impl YawPitchRoll {
    /// New.
    pub const fn new(yaw: f32, pitch: f32, roll: f32) -> Self {
        Self { yaw, pitch, roll }
    }

    /// To array.
    pub const fn to_array(self) -> [f32; 3] {
        [self.yaw, self.pitch, self.roll]
    }
}

/// Rotation represented as pitch, yaw, and roll angles.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct PitchYawRoll {
    /// Pitch angle.
    pub pitch: f32,
    /// Yaw angle.
    pub yaw: f32,
    /// Roll angle.
    pub roll: f32,
}

/// Quaternion.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Quat {
    /// X.
    pub x: f32,
    /// Y.
    pub y: f32,
    /// Z.
    pub z: f32,
    /// W.
    pub w: f32,
}

impl Quat {
    /// New.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// To array.
    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

/// Box 3D.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Box3d {
    /// A.
    a: Vec3,
    /// B.
    b: Vec3,
}

/// Iso.
#[derive(Clone, Copy, Zeroable, Pod, Default, FromLe, ToLe, Debug)]
#[repr(C)]
pub struct Iso4 {
    /// X.
    pub x: Vec3,
    /// Y.
    pub y: Vec3,
    /// Z.
    pub z: Vec3,
    /// T.
    pub t: Vec3,
}

/// Reference to a file.
#[derive(Debug)]
pub enum FileRef {
    /// Reference to an internal game file.
    Internal {
        /// Path.
        path: PathBuf,
    },
    /// Reference to an external file.
    External {
        /// Checksum.
        checksum: [u8; 32],
        /// Path.
        path: PathBuf,
        /// Locator URL.
        locator_url: String,
    },
}

impl Default for FileRef {
    fn default() -> Self {
        Self::Internal {
            path: PathBuf::default(),
        }
    }
}

const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];

const SKIPPABLE_CHUNK_MARKER: u32 = 0x534B4950;

const END_OF_NODE_MARKER: u32 = 0xfacade01;

const ID_MARKER_BIT: u32 = 0x40000000;

const HEAVY_CHUNK_MARKER_BIT: u32 = 0x80000000;

trait Class: Sized {
    const CLASS_ID: u32;
}

/// Extract the sub-extension of `path.file_name`, if possible.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// assert_eq!("Map", gamebox::sub_extension(Path::new("MyMap.Map.Gbx")).unwrap());
/// ```
pub fn sub_extension(path: &Path) -> Option<&str> {
    let mut parts = path.file_name()?.to_str()?.split('.');

    parts.next()?;
    let sub_extension = parts.next()?;
    parts.next()?;

    if parts.next().is_some() {
        return None;
    }

    Some(sub_extension)
}
