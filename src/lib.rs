#![warn(clippy::todo, clippy::unwrap_used)]

pub mod class;

mod read;

pub use class::game::ctn::Challenge;
pub use read::{read, read_file};
