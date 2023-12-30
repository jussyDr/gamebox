mod rc;

pub use rc::{RcPath, RcStr};

use std::{
    io::Read,
    path::{Path, PathBuf},
};

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

/// Reference to a file.
pub enum FileRef {
    /// Reference to an internal game file.
    Internal(InternalFileRef),
    /// Reference to an external file.
    External(ExternalFileRef),
}

impl FileRef {
    pub(crate) fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Option<Self>> {
        if d.u8()? != 3 {
            todo!()
        }

        let checksum = d.byte_array::<32>()?;
        let path = PathBuf::from(d.string()?);
        let url = d.string()?;

        if path.as_os_str().is_empty() {
            return Ok(None);
        }

        if url.is_empty() {
            Ok(Some(Self::Internal(InternalFileRef { path })))
        } else {
            Ok(Some(Self::External(ExternalFileRef {
                checksum,
                path,
                url,
            })))
        }
    }
}

/// Reference to an internal game file.
pub struct InternalFileRef {
    path: PathBuf,
}

impl InternalFileRef {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Option<Self>> {
        match FileRef::read(d)? {
            None => Ok(None),
            Some(FileRef::Internal(file_ref)) => Ok(Some(file_ref)),
            Some(FileRef::External(_)) => todo!(),
        }
    }
}

/// Reference to an external file.
pub struct ExternalFileRef {
    checksum: [u8; 32],
    path: PathBuf,
    url: String,
}

impl ExternalFileRef {
    pub fn checksum(&self) -> &[u8; 32] {
        &self.checksum
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub(crate) fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Option<Self>> {
        match FileRef::read(d)? {
            None => Ok(None),
            Some(FileRef::Internal(_)) => todo!(),
            Some(FileRef::External(file_ref)) => Ok(Some(file_ref)),
        }
    }
}

pub(crate) const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];

pub(crate) const SKIP: u32 = 0x534b4950;

pub(crate) const NODE_END: u32 = 0xfacade01;

pub(crate) mod class {
    pub trait ClassId {
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
    pub const CONTROL: u8 = 0x07;
    pub const PLUG: u8 = 0x09;
    pub const GAME_DATA: u8 = 0x2e;
    pub const META: u8 = 0x2f;
}
