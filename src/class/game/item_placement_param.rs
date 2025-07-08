//! Item placement param.

use crate::SubExtensions;

/// Item placement param.
pub struct ItemPlacementParam;

impl SubExtensions for ItemPlacementParam {
    const SUB_EXTENSIONS: &[&str] = &["PlaceParam"];
}
