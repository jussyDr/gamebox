#![warn(missing_docs, clippy::print_stdout)]

//! A GameBox file reader and writer.

pub mod game;
pub mod plug;
pub mod read;
pub mod script;

use std::{path::PathBuf, sync::Arc};

pub use read::{read, read_file};

pub trait Class: Sized {
    const CLASS_ID: u32;
}

/// A quaternion.
#[derive(Clone, Copy)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// A 2-dimensional vector.
#[derive(Clone, Copy)]
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

impl Vec2 {
    /// `[x, y, z]`.
    pub const fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

/// A 3-dimensional vector.
#[derive(Clone, Copy)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl Vec3 {
    /// `[x, y, z]`.
    pub const fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

/// A 3-dimensional vector.
#[derive(Clone, Copy, Default)]
pub struct Byte3 {
    /// X component.
    pub x: u8,
    /// Y component.
    pub y: u8,
    /// Z component.
    pub z: u8,
}

impl Byte3 {
    /// `[x, y, z]`.
    pub const fn to_array(&self) -> [u8; 3] {
        [self.x, self.y, self.z]
    }
}

/// A 3-dimensional vector.
#[derive(Clone, Copy, Default)]
pub struct Nat3 {
    /// X component.
    pub x: u32,
    /// Y component.
    pub y: u32,
    /// Z component.
    pub z: u32,
}

impl Nat3 {
    /// `[x, y, z]`.
    pub const fn to_array(&self) -> [u32; 3] {
        [self.x, self.y, self.z]
    }
}

/// A pack desc.
pub enum PackDesc {
    Internal {
        path: PathBuf,
    },
    External {
        path: PathBuf,
        locator_url: String,
        checksum: [u8; 32],
    },
}
