#![warn(
    missing_docs,
    clippy::get_unwrap,
    clippy::todo,
    clippy::unwrap_in_result
)]

//! A (incomplete) GameBox (.Gbx) file reader and writer for Trackmania (2020).
//!
//! # Examples
//!
//! Reading nodes.
//!
//! ``` no_run
//! use gamebox::{read, read_file};
//! use gamebox::classes::Item;
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
//! # |map: gamebox::classes::Map, writer: std::io::Cursor<&mut [u8]>| {
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
    pub mod map;
    pub mod material;
    pub mod prefab;
    pub mod texture;
    pub mod veget_tree_model;

    mod static_object_model;
    mod visual_indexed_triangles;

    #[doc(inline)]
    pub use ghost::Ghost;
    #[doc(inline)]
    pub use item::Item;
    #[doc(inline)]
    pub use map::Map;
}

mod common;

pub use common::{ExternalFileRef, FileRef, InternalFileRef, RcPath, RcStr, Rgb};
#[doc(inline)]
pub use read::{read, read_file, Reader};
#[doc(inline)]
pub use write::{write, write_file, Writer};
