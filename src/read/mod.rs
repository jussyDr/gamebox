mod error;
pub use error::{Error, Result};

mod reader;
pub(crate) use reader::{
    BodyReader, BodyReaderImpl, HeaderReader, ReadEnum, ReadNode, ReadNodeRef, Reader,
    read_body_chunks,
};

use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use crate::FILE_SIGNATURE;

pub fn read_file<T: Read>(path: impl AsRef<Path>) -> Result<T> {
    let file = File::open(path).map_err(Error::io)?;
    let reader = BufReader::new(file);

    read(reader)
}

pub fn read<T: Read>(reader: impl io::Read) -> Result<T> {
    let mut r = reader;

    if r.array_u8()? != FILE_SIGNATURE {
        return Err(Error::BadSignature);
    }

    if r.u16()? != 6 {
        return Err(Error::Internal("unknown file version".into()));
    }

    if r.u8()? != b'B' {
        return Err(Error::Internal("unknown file format".into()));
    }

    if r.u8()? != b'U' {
        return Err(Error::Internal(
            "unknown node reference table format".into(),
        ));
    }

    if r.u8()? != b'C' {
        return Err(Error::Internal("unknown body format".into()));
    }

    if r.u8()? != b'R' {
        return Err(Error::Internal("unknown file format".into()));
    }

    if r.u32()? != T::CLASS_ID {
        return Err(Error::ClassMismatch);
    }

    let header_data_size = r.u32()?;
    r.skip(header_data_size as usize)?;

    let num_node_refs = r
        .u32()?
        .checked_sub(1)
        .ok_or(Error::Internal("number of node references is zero".into()))?;

    let node_refs = vec![None; num_node_refs as usize].into_boxed_slice();

    let num_ext_node_refs = r.u32()?;

    if num_ext_node_refs > 0 {
        todo!()
    }

    let body_size = r.u32()?;
    let compressed_body = r.list_u8()?;

    let mut body = vec![0; body_size as usize].into_boxed_slice();
    lzo1x::decompress(&compressed_body, &mut body).map_err(|err| Error::Internal(err.into()))?;

    let mut body_reader = BodyReaderImpl::new(body.as_ref(), node_refs);

    T::read_body(&mut body_reader)
}

pub trait Read: sealed::Read {}

pub(crate) mod sealed {
    use crate::read::{Error, reader::BodyReader};

    pub trait Read: Sized {
        const CLASS_ID: u32;

        fn read_body(r: &mut impl BodyReader) -> Result<Self, Error>;
    }
}
