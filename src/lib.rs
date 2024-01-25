#![warn(
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
pub mod read;
pub mod serialize;
pub mod write;

pub mod classes {
    //! GameBox classes that can be read and (optionally) written.

    pub mod collector;
    pub mod color_table;
    pub mod ghost;
    pub mod item;
    pub mod macroblock;
    pub mod map;
    pub mod material;
    pub mod prefab;
    pub mod texture;
    pub mod veget_tree_model;

    mod ent_record_data;
    mod light_user_model;
    mod material_user_inst;
    mod static_object_model;
    mod surface;
    mod traits_metadata;
    mod visual_indexed_triangles;
    mod waypoint_special_property;
    mod zone_genealogy;
}

mod common;

pub use classes::{ghost::Ghost, item::Item, macroblock::Macroblock, map::Map};
pub use common::{ExternalFileRef, FileRef, InternalFileRef, RcPath, RcStr, Rgb, Vec3};
pub use read::{read, read_file, Reader};
pub use write::{write, write_file, Writer};
