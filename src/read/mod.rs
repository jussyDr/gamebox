mod body;
mod error;

pub use body::{BodyChunksReader, BodyReader, IdRefs};
pub use error::Error;

use std::{
    fs::File,
    io::{BufReader, Read},
    iter,
    path::Path,
};

use crate::Challenge;

pub fn read_file(path: impl AsRef<Path>) -> Result<Challenge, Error> {
    let file = File::open(path).map_err(Error::new)?;
    let reader = BufReader::new(file);

    read(reader)
}

pub fn read(reader: impl Read) -> Result<Challenge, Error> {
    let mut r = Reader { inner: reader };

    if r.u8_array()? != [b'G', b'B', b'X'] {
        return Err(Error::unknown("file signature"));
    }

    let version = r.u16()?;

    if version != 6 {
        return Err(Error::unknown_version("file", version as u32));
    }

    if r.u8()? != b'B' {
        return Err(Error::unknown_file_format());
    }

    if r.u8()? != b'U' {
        return Err(Error::unknown("node reference table compression"));
    }

    let body_compressed = match r.u8()? {
        b'C' => true,
        b'U' => false,
        _ => return Err(Error::unknown("body compression")),
    };

    if r.u8()? != b'R' {
        return Err(Error::unknown_file_format());
    }

    if r.u32()? != Challenge::CLASS_ID {
        return Err(Error::new("class id mismatch"));
    }

    let header_data_size = r.u32()?;
    let header_data = r.repeat_u8(header_data_size as usize)?;

    let num_node_refs = r
        .u32()?
        .checked_sub(1)
        .ok_or_else(|| Error::zero("number of node references"))?;

    let node_refs = iter::repeat_with(|| None)
        .take(num_node_refs as usize)
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let num_external_node_refs = r.u32()?;

    if num_external_node_refs > 0 {
        todo!()
    }

    if body_compressed {
        let body_data_size = r.u32()?;
        let compressed_body_data_size = r.u32()?;
        let compressed_body_data = r.repeat_u8(compressed_body_data_size as usize)?;

        let mut body_data = vec![0; body_data_size as usize].into_boxed_slice();
        lzo1x::decompress(&compressed_body_data, &mut body_data).map_err(Error::new)?;

        Challenge::read_from_header_and_body(header_data, body_data, node_refs)
    } else {
        todo!()
    }
}

struct Reader<R> {
    inner: R,
}

impl<R: Read> Reader<R> {
    fn repeat_u8(&mut self, n: usize) -> Result<Box<[u8]>, Error> {
        let mut bytes = vec![0; n].into_boxed_slice();
        self.inner.read_exact(&mut bytes).map_err(Error::new)?;

        Ok(bytes)
    }

    fn u8_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut bytes = [0; N];
        self.inner.read_exact(&mut bytes).map_err(Error::new)?;

        Ok(bytes)
    }

    fn u8(&mut self) -> Result<u8, Error> {
        let bytes = self.u8_array()?;

        Ok(u8::from_le_bytes(bytes))
    }

    fn u16(&mut self) -> Result<u16, Error> {
        let bytes = self.u8_array()?;

        Ok(u16::from_le_bytes(bytes))
    }

    fn u32(&mut self) -> Result<u32, Error> {
        let bytes = self.u8_array()?;

        Ok(u32::from_le_bytes(bytes))
    }
}
