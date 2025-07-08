//! Game skin and folder.

use crate::SubExtensions;

/// Game skin and folder.
pub struct GameSkinAndFolder;

impl SubExtensions for GameSkinAndFolder {
    const SUB_EXTENSIONS: &[&str] = &[
        "Gbx",
        "TerrainModifier",
        "TerrainModifier ", // Nice nadeo typo.
    ];
}
