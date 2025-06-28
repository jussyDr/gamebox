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
use zerocopy::{Immutable, IntoBytes};

use std::{fmt::Debug, path::Path, sync::Arc};

/// A GameBox class.
pub trait Class {
    const CLASS_ID: u32;
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
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

/// A 3-dimensional vector.
#[derive(Immutable, IntoBytes)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

/// A quaterion.
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

/// Matrix with 4 rows and 3 columns.
pub struct Iso4([f32; 12]);

const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];
const FILE_VERSION: u16 = 6;

const END_OF_BODY_MARKER: u32 = 0xfacade01;
const SKIPPABLE_CHUNK_MARKER: u32 = 0x534b4950;
