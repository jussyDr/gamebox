//! Types used for reading and writing [Map] nodes.

use crate::class::Class;

mod read;

/// Node type corresponding to GameBox files with the extension `Map.Gbx`.
#[derive(Default)]
pub struct Map;

impl Class for Map {
    const ENGINE: u8 = 0x03;
    const CLASS: u16 = 0x043;
}
