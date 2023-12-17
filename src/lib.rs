#![warn(missing_docs, clippy::unwrap_in_result)]

//! A (incomplete) GameBox (.Gbx) file reader and writer.
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
    //! GameBox classes that can be read and written.

    pub mod collector;
    pub mod color_table;
    pub mod item;
    pub mod material;
    pub mod texture;

    #[doc(inline)]
    pub use item::Item;
}

#[doc(inline)]
pub use read::{read, read_file};
#[doc(inline)]
pub use write::{write, write_file};

const MAGIC: [u8; 3] = [b'G', b'B', b'X'];

const SKIP: u32 = 0x534b4950;

const NODE_END: u32 = 0xfacade01;

mod class {
    pub trait Class {
        const ENGINE: u8;
        const CLASS: u16;

        fn class_id() -> u32 {
            ((Self::ENGINE as u32) << 24) | ((Self::CLASS as u32) << 12)
        }
    }
}

/// Color representation using red, green, and blue components.
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}
