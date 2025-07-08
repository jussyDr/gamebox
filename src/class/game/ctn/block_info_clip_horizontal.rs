//! Block info clip horizontal.

use std::ops::{Deref, DerefMut};

use crate::{SubExtensions, class::game::ctn::block_info_clip::BlockInfoClip};

/// Block info clip horizontal.
pub struct BlockInfoClipHorizontal {
    parent: BlockInfoClip,
}

impl Deref for BlockInfoClipHorizontal {
    type Target = BlockInfoClip;

    fn deref(&self) -> &BlockInfoClip {
        &self.parent
    }
}

impl DerefMut for BlockInfoClipHorizontal {
    fn deref_mut(&mut self) -> &mut BlockInfoClip {
        &mut self.parent
    }
}

impl SubExtensions for BlockInfoClipHorizontal {
    const SUB_EXTENSIONS: &[&str] = &["EDHorizontalClip"];
}
