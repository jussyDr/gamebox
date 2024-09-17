//! Reading GameBox files.

mod file;
mod reader;

pub use file::File;
pub use reader::{
    IdState, IdStateMut, IdStateRef, NodeState, NodeStateMut, NodeStateRef, Reader, Take,
};

use std::{io::Read, path::Path};

use crate::Error;

/// Read a node of type `T` from the given `reader`.
///
/// # Examples
/// ``` no_run
/// use gamebox::Challenge;
///
/// # fn example(reader: impl std::io::Read) -> Result<(), gamebox::Error> {
/// let challenge: Challenge = gamebox::read(reader)?;
/// # Ok(())
/// # }
/// ```
pub fn read<T: Readable>(reader: impl Read) -> Result<T, Error> {
    let file = File::new(reader)?;

    file.read()
}

/// Read a node of type `T` from a file at the given `path`.
///
/// # Examples
/// ``` no_run
/// use gamebox::Challenge;
///
/// # fn example() -> Result<(), gamebox::Error> {
/// let challenge: Challenge = gamebox::read_file("MyMap.Map.Gbx")?;
/// # Ok(())
/// # }
/// ```
pub fn read_file<T: Readable>(path: impl AsRef<Path>) -> Result<T, Error> {
    let file = File::from_file(path)?;

    file.read()
}

/// Readable GameBox class.
///
/// Note that this trait is sealed and cannot be implemented for types outside of this crate.
pub trait Readable: readable::Sealed {}

pub(crate) mod readable {
    use std::io::Read;

    use crate::Error;

    use super::reader::{IdState, IdStateMut, NodeStateMut, Reader, Take};

    pub type UserDataChunk<T> = (
        u16,
        fn(&mut T, &mut Reader<Take<&mut &[u8]>, &mut IdState, ()>) -> Result<(), Error>,
    );

    pub trait UserDataChunks {
        /// The chunks numbers must not contain duplicates and must be increasing.
        fn user_data_chunks() -> impl Iterator<Item = UserDataChunk<Self>>;
    }

    pub type BodyChunk<T, R, I, N> = (
        u16,
        fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>,
        bool,
    );

    pub trait BodyChunks {
        /// The chunks numbers must not contain duplicates and must be increasing.
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>>;
    }

    pub trait BodyChunksInline {
        /// The chunks numbers must not contain duplicates and must be increasing.
        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>;
    }

    impl<T: BodyChunksInline> BodyChunks for T {
        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            <T as BodyChunksInline>::body_chunks()
        }
    }

    pub trait Class {
        fn is_instance_of(&self, id: u32) -> bool;
    }

    impl dyn Class {
        pub fn downcast<T>(&self) -> Option<T> {
            todo!()
        }
    }

    pub trait ReadAbstract {
        fn read_abstract<R, I, N>(r: &mut Reader<R, I, N>, class_id: u32) -> Result<Self, Error>
        where
            Self: Sized;
    }

    pub trait Sealed: Default + UserDataChunks + BodyChunks {}
}
