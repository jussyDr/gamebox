#![warn(missing_docs)]

//! Reading and writing GameBox files.
//!
//! GameBox class instances are called nodes.

pub mod engines;
pub mod read;
pub mod write;

#[doc(inline)]
pub use engines::game::challenge::Challenge;
#[doc(inline)]
pub use read::{read, read_file};
#[doc(inline)]
pub use write::{write, write_file};

use std::{
    io::{Read, Write},
    rc::Rc,
};

use read::Reader;
use write::Writer;

/// An error.
#[derive(Debug)]
pub struct Error;

/// A identifier, collection, author triple.
pub struct Ident {
    /// The identifier.
    pub id: Option<Rc<str>>,
    /// The author.
    pub author: Option<Rc<str>>,
}

/// A 2-dimensional vector.
pub struct Vec2<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
}

/// A 3-dimensional vector.
pub struct Vec3<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
    /// Z component.
    pub z: T,
}

/// A 4-dimensional vector.
pub struct Vec4<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
    /// Z component.
    pub z: T,
    /// W component.
    pub w: T,
}

/// A 3-dimensional box.
pub struct Box3<T>(Vec3<T>, Vec3<T>);

/// A cardinal direction.
pub enum Direction {
    /// North.
    North,
    /// East.
    East,
    /// South.
    South,
    /// West.
    West,
}

impl Direction {
    pub(crate) fn read_u8<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let direction = match r.u8()? {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => return Err(Error),
        };

        Ok(direction)
    }

    pub(crate) fn read_u32<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let direction = match r.u32()? {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => return Err(Error),
        };

        Ok(direction)
    }
}

const FILE_SIGNATURE: &[u8; 3] = b"GBX";

const FILE_VERSION: u16 = 6;

const UNKNOWN_BYTE: u8 = b'R';

enum FileFormat {
    Binary,
    Text,
}

impl FileFormat {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let format = match r.u8()? {
            b'B' => Self::Binary,
            b'T' => Self::Text,
            _ => return Err(Error),
        };

        Ok(format)
    }

    fn write<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
        let value = match self {
            Self::Binary => b'B',
            Self::Text => b'T',
        };

        w.u8(value)
    }
}

enum Compression {
    Compressed,
    Uncompressed,
}

impl Compression {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let compression = match r.u8()? {
            b'C' => Self::Compressed,
            b'U' => Self::Uncompressed,
            _ => return Err(Error),
        };

        Ok(compression)
    }

    fn write<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
        let value = match self {
            Self::Compressed => b'C',
            Self::Uncompressed => b'U',
        };

        w.u8(value)
    }
}
