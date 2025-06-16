#![warn(missing_docs, clippy::todo, clippy::unwrap_used)]

//! Gamebox

pub mod class;
pub mod read;

pub use read::read;

use std::fmt::Debug;

pub trait Class {
    fn class_id(&self) -> u32;
}

impl Debug for dyn Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
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
