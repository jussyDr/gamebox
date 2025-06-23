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

use std::{fmt::Debug, path::Path, sync::Arc};

/// A GameBox class.
pub trait Class {
    const CLASS_ID: u32;
}

pub trait DynClass {
    fn class_id(&self) -> u32;
}

impl<T: Class> DynClass for T {
    fn class_id(&self) -> u32 {
        Self::CLASS_ID
    }
}

impl Debug for dyn DynClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Reference to a node.
#[derive(Clone, Debug)]
pub enum NodeRef {
    Internal(Arc<dyn DynClass>),
    /// Reference to a node in an external file.
    External(ExternalNodeRef),
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
