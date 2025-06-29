use std::io::Read;

use crate::{
    Class, END_OF_BODY_MARKER, SKIPPABLE_CHUNK_MARKER,
    read::{
        Error,
        reader::{IdTableRef, NodeTableRef, Reader},
    },
};

pub trait ReadBody {
    fn read_body(
        &mut self,
        r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
    ) -> Result<(), Error>;
}

pub trait BodyChunks: Class {
    fn parent(&mut self) -> Option<&mut impl BodyChunks>
    where
        Self: Sized,
    {
        None::<&mut Self>
    }

    fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
    -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>>;
}

pub struct BodyChunk<T: ?Sized, R, I, N> {
    num: u8,
    read_fn: BodyChunkReadFn<T, R, I, N>,
    skippable: bool,
}

impl<T, R, I, N> BodyChunk<T, R, I, N> {
    pub fn new(num: u8, read_fn: fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>) -> Self {
        Self {
            num,
            read_fn,
            skippable: false,
        }
    }

    pub fn skippable(
        num: u8,
        read_fn: fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>,
    ) -> Self {
        Self {
            num,
            read_fn,
            skippable: true,
        }
    }
}

type BodyChunkReadFn<T, R, I, N> = fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>;

pub fn read_body_chunks<T: BodyChunks>(
    r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
    node: &mut T,
) -> Result<(), Error> {
    let chunk_id = read_body_chunks_inner(r, node)?;

    if let Some(chunk_id) = chunk_id {
        return Err(Error(format!("unknown chunk: 0x{chunk_id:08x}")));
    }

    Ok(())
}

fn read_body_chunks_inner<T: BodyChunks>(
    r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
    node: &mut T,
) -> Result<Option<u32>, Error> {
    // Read parent chunks, if any.
    let mut chunk_id = match node.parent() {
        None => r.u32()?,
        Some(parent) => match read_body_chunks_inner(r, parent)? {
            None => return Ok(None),
            Some(chunk_id) => chunk_id,
        },
    };

    // Read chunks until either an end marker is reached, or an unknown chunk ID is encountered.
    let mut chunks = T::body_chunks().into_iter();

    loop {
        if chunk_id == END_OF_BODY_MARKER {
            break;
        }

        let class_id = chunk_id & 0xffffff00;

        if class_id != T::CLASS_ID {
            return Ok(Some(chunk_id));
        }

        let chunk_num = (chunk_id & 0x000000ff) as u8;

        let chunk = match chunks.find(|chunk| chunk.num == chunk_num) {
            None => todo!(),
            Some(chunk) => chunk,
        };

        if chunk.skippable {
            if r.u32()? != SKIPPABLE_CHUNK_MARKER {
                todo!()
            }

            let size = r.u32()?;

            (chunk.read_fn)(node, r)?;
        } else {
            (chunk.read_fn)(node, r)?;
        }

        chunk_id = r.u32()?;
    }

    Ok(None)
}

pub fn read_node<T: Default + Class + ReadBody>(
    r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
) -> Result<T, Error> {
    let mut node = T::default();
    node.read_body(r)?;
    Ok(node)
}
