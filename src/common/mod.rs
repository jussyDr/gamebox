mod file_ref;
mod rc;

pub use file_ref::*;
pub use rc::*;

use std::io::{Read, Write};

use crate::{deserialize::Deserializer, read::Result, serialize::Serializer};

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

pub const GAMEBOX_VERSION: u16 = 6;

pub const SKIP: u32 = 0x534b4950;

pub const NODE_END: u32 = 0xfacade01;

pub const ID_VERSION: u32 = 3;

pub const ID_INDEX_MASK: u32 = 0x00003fff;

pub const ID_FLAG_BIT: u32 = 0x40000000;

pub const UNKNOWN_BYTE: u8 = b'R';

pub trait ClassId {
    const ENGINE: u8;
    const CLASS: u16;

    fn class_id() -> u32 {
        ((Self::ENGINE as u32) << 24) | ((Self::CLASS as u32) << 12)
    }
}

pub fn read_compact_index<R: Read, I, N>(
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

pub struct EngineId(u8);

impl EngineId {
    pub const GAME: u8 = 0x03;
    pub const CONTROL: u8 = 0x07;
    pub const PLUG: u8 = 0x09;
    pub const GAME_DATA: u8 = 0x2e;
    pub const META: u8 = 0x2f;
}
