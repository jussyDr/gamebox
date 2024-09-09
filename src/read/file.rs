//! Reading GameBox files.

use std::{
    fs,
    io::{BufReader, Read},
    path::Path,
};

use crate::Error;

use super::{
    readable::BodyChunks,
    reader::{IdState, IdStateMut, NodeState, NodeStateMut, Reader},
    Readable,
};

const SIGNATURE: &[u8; 3] = b"GBX";

const VERSION: u16 = 6;

enum Format {
    Binary,
    Text,
}

impl Format {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let format = match r.u8()? {
            b'B' => Self::Binary,
            b'T' => Self::Text,
            _ => return Err(Error),
        };

        Ok(format)
    }
}

enum Compression {
    Compressed,
    Uncompressed,
}

impl Compression {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let compression = match r.u8()? {
            b'C' => Self::Compressed,
            b'U' => Self::Uncompressed,
            _ => return Err(Error),
        };

        Ok(compression)
    }
}

/// GameBox file.
pub struct File {
    class_id: u32,
    user_data: Box<[u8]>,
    num_nodes: u32,
    body: Box<[u8]>,
}

impl File {
    /// Create a new GameBox file from the given `reader`.
    pub fn new(reader: impl Read) -> Result<Self, Error> {
        let mut r = Reader::new(reader, (), ());

        if &r.byte_array()? != SIGNATURE {
            return Err(Error);
        }

        if r.u16()? != VERSION {
            return Err(Error);
        }

        if !matches!(Format::read(&mut r)?, Format::Binary) {
            return Err(Error);
        }

        if !matches!(Compression::read(&mut r)?, Compression::Uncompressed) {
            return Err(Error);
        }

        let body_compression = Compression::read(&mut r)?;

        if r.u8()? != b'R' {
            return Err(Error);
        }

        let class_id = r.u32()?;
        let user_data = r.byte_buf()?;
        let num_nodes = r.u32()?;

        if num_nodes == 0 {
            return Err(Error);
        }

        let num_external_node_refs = r.u32()?;

        if num_external_node_refs > 0 {
            todo!()
        }

        let body = match body_compression {
            Compression::Compressed => {
                let body_len = r.u32()?;
                let compressed_body = r.byte_buf()?;

                r.expect_eof()?;

                let mut body = vec![0; body_len as usize];
                lzo1x::decompress(&compressed_body, &mut body).map_err(|_| Error)?;

                body.into_boxed_slice()
            }
            Compression::Uncompressed => r.read_to_end()?,
        };

        Ok(Self {
            class_id,
            user_data,
            num_nodes,
            body,
        })
    }

    /// Create a new GameBox file from a file at the given `path`.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = fs::File::open(path).map_err(|_| Error)?;
        let reader = BufReader::new(file);

        Self::new(reader)
    }

    /// Read a GameBox node of type `T` from this file.
    pub fn read<T: Readable>(&self) -> Result<T, Error> {
        let mut node = T::default();

        read_user_data(&mut node, self.class_id, &self.user_data)?;
        read_body(&mut node, &self.body, self.num_nodes)?;

        Ok(node)
    }
}

fn read_user_data<T: Readable>(node: &mut T, class_id: u32, user_data: &[u8]) -> Result<(), Error> {
    let mut r = Reader::new(user_data, (), ());

    let chunk_entries = r.list(|r| {
        let chunk_id = r.u32()?;
        let chunk_len = r.u32()?;

        Ok((chunk_id, chunk_len))
    })?;

    let mut chunk_read_fns = T::user_data_chunks();

    let mut id_state = IdState::new();

    for (chunk_id, chunk_len) in chunk_entries {
        if chunk_class_id(chunk_id) != class_id {
            return Err(Error);
        }

        let chunk_num = chunk_num(chunk_id);

        let read_fn = chunk_read_fns
            .find(|(num, _)| *num == chunk_num)
            .map(|(_, read_fn)| read_fn)
            .ok_or(Error)?;

        let chunk_len = chunk_len & 0x7fffffff;

        let mut r = r.take(chunk_len as u64, &mut id_state, ());

        read_fn(node, &mut r)?;

        r.expect_eof()?;
    }

    r.expect_eof()?;

    Ok(())
}

fn read_body<T: Readable>(node: &mut T, body: &[u8], num_nodes: u32) -> Result<(), Error> {
    let mut r = Reader::new(body, IdState::new(), NodeState::new(num_nodes as usize));

    read_body_chunks(node, &mut r)?;

    r.expect_eof()?;

    Ok(())
}

pub(crate) fn read_body_chunks<T: BodyChunks>(
    node: &mut T,
    r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
) -> Result<(), Error> {
    let mut chunks = T::body_chunks();

    loop {
        let chunk_id = r.u32()?;

        if chunk_id == 0xfacade01 {
            break;
        }

        println!("{:08X?}", chunk_id);

        // if chunk_class_id(chunk_id) != class_id {
        //     return Err(Error);
        // }

        let chunk_num = chunk_num(chunk_id);

        let read_fn = chunks
            .find(|(num, _)| *num == chunk_num)
            .map(|(_, read_fn)| read_fn)
            .ok_or(Error)?;

        read_fn(node, r)?;
    }

    r.expect_eof()?;

    Ok(())
}

fn chunk_class_id(chunk_id: u32) -> u32 {
    chunk_id & 0xfffff000
}

fn chunk_num(chunk_id: u32) -> u16 {
    (chunk_id & 0x00000fff) as u16
}
