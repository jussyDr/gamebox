#![warn(
    missing_debug_implementations,
    missing_docs,
    clippy::get_unwrap,
    clippy::panic,
    clippy::print_stdout,
    clippy::todo,
    clippy::unwrap_in_result,
    clippy::unwrap_used
)]
#![forbid(unsafe_code)]

//! A (incomplete) GameBox (.Gbx) file reader and writer for Trackmania (2020).
//!
//! # Examples
//!
//! Reading nodes.
//!
//! ``` no_run
//! use gamebox::{read, read_file};
//! use gamebox::Item;
//!
//! # |reader: std::io::Cursor<&[u8]>| {
//! let item1: Item = read(reader)?;
//! let item2: Item = read_file("MyItem.Item.Gbx")?;
//! # Ok::<(), gamebox::read::Error>(()) };
//! ```
//!
//! Writing nodes.
//!
//! ``` no_run
//! use gamebox::{write, write_file};
//!
//! # |map: gamebox::Map, writer: std::io::Cursor<&mut [u8]>| {
//! write(&map, writer)?;
//! write_file(&map, "MyMap.Item.Gbx")?;
//! # Ok::<(), gamebox::write::Error>(()) };
//! ```

pub mod deserialize;
pub mod engines;
pub mod read;
pub mod serialize;
pub mod write;

mod common;

pub use common::{ExternalFileRef, FileRef, InternalFileRef, RcPath, RcStr, Rgb, Vec2, Vec3};
pub use engines::{
    game::{ghost::Ghost, macroblock::Macroblock, map::Map},
    game_data::item::Item,
};
pub use read::{read, read_file, Reader};
pub use write::{write, write_file, Writer};
