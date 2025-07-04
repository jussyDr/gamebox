//! File image.

use crate::SubExtensions;

/// An image file.
pub struct FileImg;

impl SubExtensions for FileImg {
    const SUB_EXTENSIONS: &[&str] = &["dds"];
}
