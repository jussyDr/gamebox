mod file_ref;
mod rc;

pub use file_ref::*;
pub use rc::*;

use std::io::{Read, Write};

use crate::{deserialize::Deserializer, read::Result, serialize::Serializer};

/// A 3-dimensional vector of type `T`.
#[derive(Clone, Copy, Default)]
pub struct Vec3<T> {
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T,
}

impl<T: Copy> Vec3<T> {
    // Convert to an array with of form `[x, y, z]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # |vec3: gamebox::Vec3<f32>| {
    /// let [x, y, z] = vec3.into_array();
    /// # };
    /// ```
    pub const fn into_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

/// Color representation using red, green, and blue components.
///
/// Each component is represented as an 8-bit unsigned integer.
#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Rgb {
    /// Convert to an array with of form `[r, g, b]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # |color: gamebox::Rgb| {
    /// let [r, g, b] = color.into_array();
    /// # };
    /// ```
    pub const fn into_array(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

pub enum FileFormat {
    Binary,
    Text,
}

impl FileFormat {
    pub fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Self> {
        match d.u8()? {
            b'B' => Ok(Self::Binary),
            b'T' => Ok(Self::Text),
            _ => Err("unknown gamebox file format".into()),
        }
    }

    pub fn write<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> crate::write::Result {
        let x = match *self {
            Self::Binary => b'B',
            Self::Text => b'T',
        };

        s.u8(x)?;

        Ok(())
    }
}

pub enum Compression {
    Compressed,
    Uncompressed,
}

impl Compression {
    pub fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Self> {
        match d.u8()? {
            b'C' => Ok(Self::Compressed),
            b'U' => Ok(Self::Uncompressed),
            _ => Err("unknown compression".into()),
        }
    }

    pub fn write<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> crate::write::Result {
        let x = match *self {
            Self::Compressed => b'C',
            Self::Uncompressed => b'U',
        };

        s.u8(x)?;

        Ok(())
    }
}

pub const GAMEBOX_FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];

pub const GAMEBOX_FILE_VERSION: u16 = 6;

pub const UNKNOWN_BYTE: u8 = b'R';

pub const HEAVY_CHUNK_MARKER_BIT: u32 = 0x80000000;

pub const SKIPPABLE_CHUNK_MARKER: u32 = 0x534b4950;

pub const END_OF_NODE_MARKER: u32 = 0xfacade01;

pub const ID_VERSION: u32 = 3;

pub const ID_INDEX_MASK: u32 = 0x00003fff;

pub const ID_MARKER_BIT: u32 = 0x40000000;

pub const NULL: u32 = 0xffffffff;

pub trait Class {
    const CLASS_ID: ClassId;
}

pub struct EngineId(u8);

impl EngineId {
    pub const GAME: Self = Self(3);
    pub const CONTROL: Self = Self(7);
    pub const PLUG: Self = Self(9);
    pub const SCRIPT: Self = Self(17);
    pub const GAME_DATA: Self = Self(46);
    pub const META: Self = Self(47);
}

pub struct ClassId(u32);

impl ClassId {
    pub const fn new(engine_id: EngineId, class: u16) -> Self {
        if class & 0xf000 != 0 {
            panic!()
        }

        Self((engine_id.0 as u32) << 24 | (class as u32) << 12)
    }

    pub const fn get(&self) -> u32 {
        self.0
    }
}

pub struct ChunkId(u32);

impl ChunkId {
    pub const fn new(class_id: ClassId, chunk: u16) -> Self {
        if chunk & 0xf000 != 0 {
            panic!()
        }

        Self(class_id.get() | chunk as u32)
    }

    pub const fn chunk(&self) -> u16 {
        (self.0 & 0x00000fff) as u16
    }
}
