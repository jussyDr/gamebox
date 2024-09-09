//! Reading GameBox files.

pub mod file;
pub mod reader;

use std::{io::Read, path::Path};

use file::File;

use crate::Error;

/// Read a GameBox node of type `T` from the given `reader`.
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

/// Read a GameBox node of type `T` from a file at the given `path`.
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

/// Readable GameBox node.
///
/// Note that this trait is sealed and cannot be implemented for types outside of GameBox.
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

    pub type BodyChunk<T, R, I, N> = (u16, fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>);

    pub trait BodyChunks {
        /// The chunks numbers must not contain duplicates and must be increasing.
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>>;
    }

    pub trait Sealed: Default + UserDataChunks + BodyChunks {}
}
