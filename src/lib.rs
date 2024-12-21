#![warn(
    missing_docs,
    clippy::unwrap_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::todo,
    clippy::undocumented_unsafe_blocks
)]

//! GameBox file reading and writing.
//!
//! #### Common files
//!
//! | Extension | Class | Readable | Writable |
//! | --- | --- | --- | --- |
//! | `Block.Gbx` | [game::ItemModel] | [x] | [ ] |
//! | `Item.Gbx` | [game::ItemModel] | [x] | [ ] |
//! | `Map.Gbx` | [game::ctn::Challenge] | [x] | [ ] |
//!
//! # Complete file list
//!
//! | Extension | Class | Readable | Writable |
//! | --- | --- | --- | --- |
//! | `Block.Gbx` | [game::ItemModel] | [x] | [ ] |
//! | `Challenge.Gbx` | [game::ctn::Challenge] | [x] | [ ] |
//! | `Collection.Gbx` | [game::ctn::Collection] | [x] | [ ] |
//! | `Decoration.Gbx` | [game::ctn::Decoration] | [x] | [ ] |
//! | `DecorationMood.Gbx` | [game::ctn::DecorationMood] | [x] | [ ] |
//! | `EDClassic.Gbx` | [game::ctn::BlockInfoClassic] | [x] | [ ] |
//! | `EDClip.Gbx` | [game::ctn::BlockInfoClip] | [x] | [ ] |
//! | `EDFlat.Gbx` | [game::ctn::BlockInfoFlat] | [x] | [ ] |
//! | `EDHorizontalClip.Gbx` | [game::ctn::BlockInfoClipHorizontal] | [x] | [ ] |
//! | `EDVerticalClip.Gbx` | [game::ctn::BlockInfoClipVertical] | [x] | [ ] |
//! | `Item.Gbx` | [game::ItemModel] | [x] | [ ] |
//! | `Map.Gbx` | [game::ctn::Challenge] | [x] | [ ] |
//! | `Material.Gbx` | [plug::Material] | [x] | [ ] |
//! | `Prefab.Gbx` | [plug::Prefab] | [x] | [ ] |
//! | `Texture.Gbx` | [plug::Bitmap] | [x] | [ ] |
//! | `Title.Gbx` | [game::ManiaTitle] | [x] | [ ] |

pub mod control;
pub mod game;
pub mod plug;
pub mod read;
pub mod script;
pub mod write;

mod node_ref;

#[doc(inline)]
pub use read::{read, read_file};
#[doc(inline)]
pub use write::{write, write_file};

pub use node_ref::{ExternalNodeRef, NodeRef};

use std::path::PathBuf;

/// 2-dimensional vector.
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Vec2<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
}

/// 3-dimensional vector.
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Vec3<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
    /// Z component.
    pub z: T,
}

impl<T: Copy> Vec3<T> {
    /// From array.
    pub const fn from_array(array: [T; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }

    /// To array `[x, y, z]`.
    pub const fn to_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

/// 4-dimensional vector.
#[derive(Clone, Copy, Default, Debug)]
pub struct Vec4<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
    /// Z component.
    pub z: T,
    /// W component.
    pub w: T,
}

impl<T: Copy> Vec4<T> {
    /// From array.
    pub const fn from_array(array: [T; 4]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
            w: array[3],
        }
    }
}

/// Texture coordinate.
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Texcoord {
    /// U component.
    pub u: f32,
    /// V component.
    pub v: f32,
}

impl Texcoord {
    /// From array.
    pub const fn from_array(array: [f32; 2]) -> Self {
        Self {
            u: array[0],
            v: array[1],
        }
    }
}

/// Color represented by red, green, and blue components.
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Rgb<T> {
    /// Red component.
    pub r: T,
    /// Green component.
    pub g: T,
    /// Blue component.
    pub b: T,
}

/// Color represented by red, green, and blue components.
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Rgba<T> {
    /// Red component.
    pub r: T,
    /// Green component.
    pub g: T,
    /// Blue component.
    pub b: T,
    /// Alpha component.
    pub a: T,
}

impl<T: Copy> Rgba<T> {
    /// From array.
    pub const fn from_array(array: [T; 4]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
            a: array[3],
        }
    }
}

/// Rotation represented as pitch, yaw, and roll angles.
#[derive(Clone, Copy, Default, Debug)]
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
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Quat {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
    /// W component.
    pub w: f32,
}

/// Iso.
#[derive(Default, Debug)]
pub struct Iso4 {
    /// X.
    pub x: Vec4<f32>,
    /// Y.
    pub y: Vec4<f32>,
    /// Z.
    pub z: Vec4<f32>,
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
