#![warn(missing_docs, clippy::unwrap_in_result)]

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
//! # |item: gamebox::classes::Item, writer: std::io::Cursor<&mut [u8]>| {
//! write(&item, writer)?;
//! write_file(&item, "MyItem.Item.Gbx")?;
//! # Ok::<(), gamebox::write::Error>(()) };
//! ```

pub mod read;
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
    pub mod static_object_model;
    pub mod texture;
    pub mod veget_tree_model;
    pub mod visual_indexed_triangles;

    #[doc(inline)]
    pub use ghost::Ghost;
    #[doc(inline)]
    pub use item::Item;
    #[doc(inline)]
    pub use map::Map;
}

mod common;

pub use common::*;
#[doc(inline)]
pub use read::{read, read_file, Reader};
#[doc(inline)]
pub use write::{write, write_file, Writer};
