//! Block info clip vertical.

use std::ops::{Deref, DerefMut};

use crate::{SubExtensions, class::game::ctn::block_info_clip::BlockInfoClip};

/// Block info clip vertical.
pub struct BlockInfoClipVertical {
    parent: BlockInfoClip,
}

impl Deref for BlockInfoClipVertical {
    type Target = BlockInfoClip;

    fn deref(&self) -> &BlockInfoClip {
        &self.parent
    }
}

impl DerefMut for BlockInfoClipVertical {
    fn deref_mut(&mut self) -> &mut BlockInfoClip {
        &mut self.parent
    }
}

impl SubExtensions for BlockInfoClipVertical {
    const SUB_EXTENSIONS: &[&str] = &["EDVerticalClip"];
}
