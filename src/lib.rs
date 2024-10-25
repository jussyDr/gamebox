#![warn(missing_docs, clippy::print_stdout)]

//! GameBox file reading and writing for Trackmania (2020).

pub mod engines;
pub mod read;
pub mod write;

#[doc(inline)]
pub use engines::game::challenge::Challenge;
#[doc(inline)]
pub use engines::game::item_model::ItemModel;
#[doc(inline)]
pub use read::{read, read_file};
#[doc(inline)]
pub use write::{write, write_file};

use std::sync::Arc;

/// A identifier, collection, author triple.
#[derive(Default)]
pub struct Ident {
    /// The identifier.
    pub id: Option<Arc<str>>,
    /// The collection.
    pub collection: Option<()>,
    /// The author.
    pub author: Option<Arc<str>>,
}

/// A 2-dimensional vector.
#[derive(Default)]
pub struct Vec2<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
}

/// A 3-dimensional vector.
#[derive(Default)]
pub struct Vec3<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
    /// Z component.
    pub z: T,
}

/// A 4-dimensional vector.
#[derive(Default)]
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
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

const FILE_SIGNATURE: &[u8; 3] = b"GBX";

const FILE_VERSION: u16 = 6;

const UNKNOWN_BYTE: u8 = b'R';

enum FileFormat {
    Binary,
    Text,
}

enum Compression {
    Compressed,
    Uncompressed,
}

/// A pack descriptor.
pub struct PackDesc;

mod r {
    use std::io::Read;

    use crate::{
        read::{Error, Reader},
        Compression, Direction, FileFormat,
    };

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

    impl FileFormat {
        pub(crate) fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
            let format = match r.u8()? {
                b'B' => Self::Binary,
                b'T' => Self::Text,
                _ => return Err(Error),
            };

            Ok(format)
        }
    }

    impl Compression {
        pub(crate) fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
            let compression = match r.u8()? {
                b'C' => Self::Compressed,
                b'U' => Self::Uncompressed,
                _ => return Err(Error),
            };

            Ok(compression)
        }
    }
}

mod w {
    use std::io::Write;

    use crate::{
        write::{Error, Writer},
        Compression, FileFormat,
    };

    impl FileFormat {
        pub(crate) fn write<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let value = match self {
                Self::Binary => b'B',
                Self::Text => b'T',
            };

            w.u8(value)
        }
    }

    impl Compression {
        pub(crate) fn write<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            let value = match self {
                Self::Compressed => b'C',
                Self::Uncompressed => b'U',
            };

            w.u8(value)
        }
    }
}
