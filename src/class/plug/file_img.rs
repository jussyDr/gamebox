use crate::Extensions;

/// An image file.
pub struct FileImg;

impl Extensions for FileImg {
    const EXTENSIONS: &[&str] = &["dds"];
}
