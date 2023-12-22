use std::io::Read;

use crate::read::{deserialize::Deserializer, Result};

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
    /// Convert to an array with the form `[r, g, b]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # |color: gamebox::Rgb| {
    /// let array = color.into_array();
    /// # };
    /// ```
    pub const fn into_array(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

/// A 3-dimensional vector with components of type `T`.
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy> Vec3<T> {
    /// Convert to an array with the form `[x, y, z]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # |vec3: gamebox::Vec3<f32>| {
    /// let array = vec3.into_array();
    /// # };
    /// ```
    pub const fn into_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

pub(crate) const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];

pub(crate) const SKIP: u32 = 0x534b4950;

pub(crate) const NODE_END: u32 = 0xfacade01;

pub(crate) mod class {
    pub trait Class {
        const ENGINE: u8;
        const CLASS: u16;

        fn class_id() -> u32 {
            ((Self::ENGINE as u32) << 24) | ((Self::CLASS as u32) << 12)
        }
    }
}

pub(crate) fn read_compact_index<R: Read, I, N>(
    d: &mut Deserializer<R, I, N>,
    num_items: u32,
) -> Result<u32> {
    if num_items < u8::MAX as u32 {
        let index = d.u8()?;
        Ok(index as u32)
    } else if num_items < u16::MAX as u32 {
        let index = d.u16()?;
        Ok(index as u32)
    } else {
        d.u32()
    }
}

pub(crate) struct EngineId(u8);

impl EngineId {
    pub const GAME: u8 = 0x03;
    pub const PLUG: u8 = 0x09;
    pub const GAME_DATA: u8 = 0x2e;
    pub const META: u8 = 0x2f;
}

pub(crate) struct ClassId {
    engine: EngineId,
    class: u16,
}

pub(crate) struct ChunkId {
    class: ClassId,
    chunk: u16,
}
