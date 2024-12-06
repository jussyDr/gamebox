#![warn(missing_docs, clippy::print_stdout)]

//! GameBox file reading and writing.

pub mod control;
pub mod game;
pub mod plug;
pub mod read;
pub mod script;
pub mod write;

#[doc(inline)]
pub use read::{read, read_file};
#[doc(inline)]
pub use write::{write, write_file};

use std::path::PathBuf;

/// A 2-dimensional vector of type `T`.
pub struct Vec2<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
}

/// A 3-dimensional vector of type `T`.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Vec3<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
    /// Z component.
    pub z: T,
}

impl<T: Copy> Vec3<T> {
    pub const fn from_array(array: [T; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

/// A quaternion.
#[derive(Clone, Copy)]
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

/// A texture coordinate.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Texcoord {
    /// U component.
    pub u: f32,
    /// V component.
    pub v: f32,
}

impl Texcoord {
    pub const fn from_array(array: [f32; 2]) -> Self {
        Self {
            u: array[0],
            v: array[1],
        }
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

const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];

const SKIPPABLE_CHUNK_MARKER: u32 = 0x534B4950;

const END_OF_NODE_MARKER: u32 = 0xfacade01;

trait Class: Sized {
    const CLASS_ID: u32;
}
