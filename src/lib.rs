//! GameBox file reading and writing.

#![warn(clippy::todo, clippy::print_stdout, missing_docs)]

mod class;
pub use class::{control, game, plug, scene, script};

pub mod read;
pub use read::{read, read_file};

pub mod write;
pub use write::{write, write_file};

pub use game::ctn::Challenge;

/// 3-dimensional vector.
#[derive(Clone, Copy, Default)]
pub struct Vec3<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
    /// Z component
    pub z: T,
}

/// Color represented as red, green, and blue channels.
#[derive(Clone, Copy, Default, Debug)]
pub struct Rgb {
    /// Red channel. [0.0, 1.0]
    pub red: f32,
    /// Green channel. [0.0, 1.0]
    pub green: f32,
    /// Blue channel. [0.0, 1.0]
    pub blue: f32,
}

/// Rotation represented as yaw, pitch, and roll angles.
#[derive(Clone, Copy, Default)]
pub struct YawPitchRoll {
    /// Yaw angle in radians.
    pub yaw: f32,
    /// Pitch angle in radians.
    pub pitch: f32,
    /// Roll angle in radians.
    pub roll: f32,
}

const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];
