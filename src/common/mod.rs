mod file_ref;
mod rc;

pub use file_ref::*;
pub use rc::*;

use std::io::{Read, Write};

use crate::{deserialize::Deserializer, read::Result, serialize::Serializer};

/// A 2-dimensional vector of type `T`.
#[derive(Clone, Copy, Default)]
pub struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T: Copy> Vec2<T> {
    // Convert to an array with of form `[x, y]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # |vec2: gamebox::Vec2<f32>| {
    /// let [x, y] = vec2.into_array();
    /// # };
    /// ```
    pub const fn into_array(self) -> [T; 2] {
        [self.x, self.y]
    }
}

/// A 3-dimensional vector of type `T`.
#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Copy for Vec3<u8> {}

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
    pub const GRAPHIC: Self = Self(4);
    pub const CONTROL: Self = Self(7);
    pub const PLUG: Self = Self(9);
    pub const SCRIPT: Self = Self(17);
    pub const GAME_DATA: Self = Self(46);
    pub const META: Self = Self(47);
}

#[derive(Clone, Copy, PartialEq)]
pub struct ClassId(u32);

impl ClassId {
    pub const fn new(engine: EngineId, class: u16) -> Self {
        if class & 0xf000 != 0 {
            panic!()
        }

        Self((engine.0 as u32) << 24 | (class as u32) << 12)
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    pub(crate) fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Self> {
        let value = d.u32()?;

        if !matches!((value & 0xff000000) >> 24, 3 | 4 | 7 | 9 | 17 | 46 | 47) {
            return Err("".into());
        }

        if value & 0x00000fff != 0 {
            return Err("".into());
        }

        Ok(Self(value))
    }

    pub(crate) fn read_or_null<R: Read, I, N>(
        d: &mut Deserializer<R, I, N>,
    ) -> Result<Option<Self>> {
        let value = d.u32()?;

        if value == NULL {
            return Ok(None);
        }

        if !matches!((value & 0xff000000) >> 24, 3 | 4 | 7 | 9 | 17 | 46 | 47) {
            return Err("".into());
        }

        if value & 0x00000fff != 0 {
            return Err("".into());
        }

        Ok(Some(Self(value)))
    }
}
