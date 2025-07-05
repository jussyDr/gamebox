#![warn(
    missing_docs,
    clippy::todo,
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::undocumented_unsafe_blocks,
    clippy::panic,
    clippy::arithmetic_side_effects,
    clippy::or_fun_call // Remove if there is a valid or_fun_call.
)]

//! Gamebox file reading and writing.

pub mod class;
pub mod read;

// Re-export common class types.
pub use class::game::ctn::challenge::Challenge;
pub use read::{read, read_file};

use zerocopy::{FromBytes, Immutable, IntoBytes};

use std::{fmt::Debug, path::Path, sync::Arc};

use crate::read::byte_order::LeToNe;

/// GameBox class ID.
pub trait ClassId {
    /// GameBox class ID.
    const CLASS_ID: u32;
}

/// Sub extensions.
pub trait SubExtensions {
    /// GameBox sub-extensions that correspond to this type.
    ///
    /// Not case sensitive.
    const SUB_EXTENSIONS: &[&str];

    /// Returns `true` if the given `sub_extension` matches one of the `SUB_EXTENSIONS` associated with this type.
    fn has_sub_extension(sub_extension: &str) -> bool {
        Self::SUB_EXTENSIONS
            .iter()
            .any(|se| se.eq_ignore_ascii_case(sub_extension))
    }
}

/// Delme.
pub struct Delme;

impl SubExtensions for Delme {
    const SUB_EXTENSIONS: &[&str] = &[];
}

/// Reference to a node.
#[derive(Debug, Clone)]
pub enum NodeRef<T> {
    /// Reference to a node in memory.
    Internal(T),
    /// Reference to a node in an external file.
    External(ExternalNodeRef),
}

impl<T> NodeRef<T> {
    /// Internal.
    pub fn internal(&self) -> Option<&T> {
        match self {
            Self::Internal(value) => Some(value),
            Self::External(_) => None,
        }
    }

    /// External.
    pub fn external(&self) -> Option<&ExternalNodeRef> {
        match self {
            Self::Internal(_) => None,
            Self::External(value) => Some(value),
        }
    }
}

impl<T: Default> Default for NodeRef<T> {
    fn default() -> Self {
        Self::Internal(T::default())
    }
}

/// Reference to a node in an external file.
#[derive(Clone, Debug)]
pub struct ExternalNodeRef {
    /// Path.
    pub path: Arc<Path>,
    /// Ancestor level.
    pub ancestor_level: u32,
}

impl Default for ExternalNodeRef {
    fn default() -> Self {
        Self {
            path: Arc::from(Path::new("")),
            ancestor_level: 0,
        }
    }
}

/// A 2-dimensional vector.
#[derive(FromBytes, IntoBytes)]
#[repr(C)]
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

impl Vec2 {
    /// Convert to an array `[x, y]`.
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl LeToNe for Vec2 {
    fn le_to_ne(&mut self) {
        self.x.le_to_ne();
        self.y.le_to_ne();
    }
}

/// A 3-dimensional vector.
#[derive(PartialEq, Debug, Immutable, FromBytes, IntoBytes)]
#[repr(C)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl Vec3 {
    /// Create a new `Vec3`.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self { x, y, z }
    }

    /// Convert to an array `[x, y, z]`.
    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl LeToNe for Vec3 {
    fn le_to_ne(&mut self) {
        self.x.le_to_ne();
        self.y.le_to_ne();
        self.z.le_to_ne();
    }
}

/// A quaterion.
#[derive(FromBytes, Debug)]
#[repr(C)]
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

impl Quat {
    /// Convert to an array `[x, y, z, w]`.
    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl LeToNe for Quat {
    fn le_to_ne(&mut self) {
        self.x.le_to_ne();
        self.y.le_to_ne();
        self.z.le_to_ne();
        self.w.le_to_ne();
    }
}

/// Matrix with 4 rows and 3 columns.
pub struct Iso4(pub [f32; 12]);

const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];
const FILE_VERSION: u16 = 6;

const END_OF_BODY_MARKER: u32 = 0xfacade01;
const SKIPPABLE_CHUNK_MARKER: u32 = 0x534b4950;

/// Returns the sub-extension for the given `path`.
///
/// - If the path has the form `file_name.sub_extension.gbx` this function returns `Some(sub_extension)`.
/// - If the path has the form `file_name.extension` this function returns `Some(extension)`.
/// - Else this function returns `None`
fn sub_extension(path: &Path) -> Option<&str> {
    let parts: Vec<_> = path.to_str()?.split('.').collect();

    match parts.as_slice() {
        [_, extension] => Some(extension),
        [_, sub_extension, extension] if extension.eq_ignore_ascii_case("gbx") => {
            Some(sub_extension)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn sub_extension() {
        for (path, expected) in [
            (Path::new("image.dds"), "dds"),
            (Path::new("challenge.map.gbx"), "map"),
        ] {
            let se = super::sub_extension(path);
            assert_eq!(se, Some(expected));
        }
    }
}
