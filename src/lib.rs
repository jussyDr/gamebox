#![warn(
    missing_docs,
    clippy::todo,
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::undocumented_unsafe_blocks,
    clippy::panic
)]

//! Gamebox

pub mod class;
pub mod read;

pub use read::{read, read_file};
use zerocopy::{FromBytes, IntoBytes};

use std::{fmt::Debug, path::Path, sync::Arc};

use crate::read::byte_order::LeToNe;

/// GameBox class ID.
pub trait ClassId {
    const CLASS_ID: u32;
}

pub trait Extensions {
    const EXTENSIONS: &[&str];
}

pub struct Delme;

impl Extensions for Delme {
    const EXTENSIONS: &[&str] = &[];
}

/// Reference to a node.
#[derive(Debug, Clone)]
pub enum NodeRef<T> {
    Internal(T),
    /// Reference to a node in an external file.
    External(ExternalNodeRef),
}

impl<T> NodeRef<T> {
    pub fn internal(&self) -> Option<&T> {
        match self {
            Self::Internal(value) => Some(value),
            Self::External(_) => None,
        }
    }

    pub fn external(&self) -> Option<&ExternalNodeRef> {
        match self {
            Self::Internal(value) => None,
            Self::External(value) => Some(value),
        }
    }
}

impl<T: Default> Default for NodeRef<T> {
    fn default() -> Self {
        Self::Internal(T::default())
    }
}

/// Reference to a node in an external file.
#[derive(Clone, Debug)]
pub struct ExternalNodeRef {
    pub path: Arc<Path>,
    pub ancestor_level: u32,
}

impl Default for ExternalNodeRef {
    fn default() -> Self {
        Self {
            path: Arc::from(Path::new("")),
            ancestor_level: 0,
        }
    }
}

/// A 2-dimensional vector.
#[derive(FromBytes, IntoBytes)]
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

impl LeToNe for Vec2 {
    fn le_to_ne(&mut self) {
        self.x.le_to_ne();
        self.y.le_to_ne();
    }
}

/// A 3-dimensional vector.
#[derive(PartialEq, Debug, FromBytes, IntoBytes)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self { x, y, z }
    }
}

impl LeToNe for Vec3 {
    fn le_to_ne(&mut self) {
        self.x.le_to_ne();
        self.y.le_to_ne();
        self.z.le_to_ne();
    }
}

/// A quaterion.
#[derive(FromBytes)]
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

impl LeToNe for Quat {
    fn le_to_ne(&mut self) {
        self.x.le_to_ne();
        self.y.le_to_ne();
        self.z.le_to_ne();
        self.w.le_to_ne();
    }
}

/// Matrix with 4 rows and 3 columns.
pub struct Iso4([f32; 12]);

const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];
const FILE_VERSION: u16 = 6;

const END_OF_BODY_MARKER: u32 = 0xfacade01;
const SKIPPABLE_CHUNK_MARKER: u32 = 0x534b4950;

fn full_extension(path: &Path) -> Option<&str> {
    let file_name = path.file_name()?.to_str()?;

    file_name.find('.').map(|index| &file_name[index + 1..])
}
