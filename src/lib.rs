#![warn(missing_docs, clippy::todo, clippy::unwrap_used, clippy::print_stdout)]

//! Gamebox

pub mod class;
pub mod read;

pub use read::read;

use std::{fmt::Debug, path::Path, sync::Arc};

pub trait Class {
    fn class_id(&self) -> u32;
}

impl Debug for dyn Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Reference to a node.
#[derive(Clone, Debug)]
pub enum NodeRef {
    Internal(Arc<dyn Class>),
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

const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];
const FILE_VERSION: u16 = 6;
