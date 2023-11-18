#![warn(missing_docs)]

//! GameBox

pub mod read;
pub mod write;

pub mod item;

mod deserializer;
mod serializer;

#[doc(inline)]
pub use read::read;
#[doc(inline)]
pub use write::write;
