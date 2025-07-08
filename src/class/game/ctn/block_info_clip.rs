//! Block info clip.

use crate::SubExtensions;

/// Block info clip.
pub struct BlockInfoClip;

impl SubExtensions for BlockInfoClip {
    const SUB_EXTENSIONS: &[&str] = &["EDClip", "EDHorizontalClip", "EDVerticalClip"];
}
