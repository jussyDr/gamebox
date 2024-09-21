//! Writing GameBox files.

mod writer;

pub use writer::{IdStateMut, Writer};

use lzo1x::CompressLevel;

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{Compression, Error, FileFormat, FILE_SIGNATURE, FILE_VERSION, UNKNOWN_BYTE};

/// Write a node to the given `writer`.
///
/// # Examples
/// ``` no_run
/// # fn example(writer: impl std::io::Write) -> Result<(), gamebox::Error> {
/// use gamebox::Challenge;
///
/// let challenge = Challenge::default();
///
/// gamebox::write(&challenge, writer)?;
/// # Ok(())
/// # }
/// ```
pub fn write<T: Writable>(node: &T, writer: impl Write) -> Result<(), Error> {
    let user_data = vec![];
    let body = vec![];
    let num_nodes = 0;

    let compressed_body = lzo1x::compress(&body, CompressLevel::default());

    let mut w = Writer::new(writer, (), ());

    w.bytes(FILE_SIGNATURE)?;
    w.u16(FILE_VERSION)?;
    FileFormat::Binary.write(&mut w)?;
    Compression::Uncompressed.write(&mut w)?;
    Compression::Compressed.write(&mut w)?;
    w.u8(UNKNOWN_BYTE)?;
    w.u32(T::CLASS_ID)?;
    w.u32(user_data.len() as u32)?;
    w.bytes(user_data)?;
    w.u32(num_nodes)?;
    w.u32(0)?;
    w.u32(body.len() as u32)?;
    w.u32(compressed_body.len() as u32)?;
    w.bytes(compressed_body)?;

    Ok(())
}

/// Write a node to a file at the given `path`.
///
/// # Examples
/// ``` no_run
/// # fn example(writer: impl std::io::Write) -> Result<(), gamebox::Error> {
/// use gamebox::Challenge;
///
/// let challenge = Challenge::default();
///
/// gamebox::write_file(&challenge, "MyMap.Map.Gbx")?;
/// # Ok(())
/// # }
/// ```
pub fn write_file<T: Writable>(node: &T, path: impl AsRef<Path>) -> Result<(), Error> {
    let file = File::create(path).map_err(|_| Error)?;
    let writer = BufWriter::new(file);

    write(node, writer)
}

/// Writable GameBox class.
///
/// Note that this trait is sealed and cannot be implemented for types outside of this crate.
pub trait Writable: writable::Sealed {}

pub(crate) mod writable {
    use std::io::Write;

    use crate::Error;

    use super::{IdStateMut, Writer};

    pub trait WriteUserData {
        fn write_user_data<W: Write, I: IdStateMut, N>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error>;
    }

    pub trait WriteBody {
        fn write_body<W: Write, I: IdStateMut, N>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error>;
    }

    pub trait Sealed: WriteUserData + WriteBody {
        const CLASS_ID: u32;
    }
}
