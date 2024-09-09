#![warn(missing_docs)]

//! Reading and writing GameBox files.

use std::rc::Rc;

pub mod read;
pub mod write;

pub mod engines {
    //! GameBox engines.

    pub mod game {
        //! Game engine.

        pub mod challenge;
        pub mod collector_list;
    }

    pub mod game_data {
        //! GameData engine.
    }
}

pub use engines::game::challenge::Challenge;
pub use read::{read, read_file};

/// GameBox error.
#[derive(Debug)]
pub struct Error;

/// Identifier triple
pub struct Ident {
    /// The identifier.
    pub id: Option<Rc<str>>,
    /// The author.
    pub author: Option<Rc<str>>,
}

/// A 2-dimensional vector.
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}
