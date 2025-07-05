use crate::{
    ClassId, END_OF_BODY_MARKER, SKIPPABLE_CHUNK_MARKER,
    read::{Error, reader::BodyReader},
};

/// Read body.
pub trait ReadBody {
    /// Read body.
    fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error>;
}

/// Body chunks.
pub trait BodyChunks: ClassId {
    /// Parent.
    fn parent(&mut self) -> Option<&mut impl BodyChunks>
    where
        Self: Sized,
    {
        None::<&mut Self>
    }

    /// Body chunks.
    fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>>;
}

/// Body chunk.
pub struct BodyChunk<T: ?Sized, R> {
    num: u8,
    read_fn: BodyChunkReadFn<T, R>,
    skippable: bool,
}

impl<T, R> BodyChunk<T, R> {
    /// New.
    pub fn new(num: u8, read_fn: BodyChunkReadFn<T, R>) -> Self {
        Self {
            num,
            read_fn,
            skippable: false,
        }
    }

    /// Skippable.
    pub fn skippable(num: u8, read_fn: BodyChunkReadFn<T, R>) -> Self {
        Self {
            num,
            read_fn,
            skippable: true,
        }
    }
}

type BodyChunkReadFn<T, R> = fn(&mut T, &mut R) -> Result<(), Error>;

/// Read body chunks.
pub fn read_body_chunks<T: BodyChunks>(r: &mut impl BodyReader, node: &mut T) -> Result<(), Error> {
    let chunk_id = read_body_chunks_inner(r, node)?;

    if let Some(chunk_id) = chunk_id {
        return Err(Error::new(format!("unknown chunk: 0x{chunk_id:08x}")));
    }

    Ok(())
}

fn read_body_chunks_inner<T: BodyChunks>(
    r: &mut impl BodyReader,
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

    // Read chunks until either an end marker is reached, or a chunk ID of a different class encountered.
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
            None => {
                return Err(Error::new(format!(
                    "unknown chunk number: {chunk_num} of class 0x{class_id:08x}"
                )));
            }
            Some(chunk) => chunk,
        };

        if chunk.skippable {
            if r.u32()? != SKIPPABLE_CHUNK_MARKER {
                return Err(Error::new("expected a skippable chunk"));
            }

            let _size = r.u32()?;

            (chunk.read_fn)(node, r)?;
        } else {
            (chunk.read_fn)(node, r)?;
        }

        chunk_id = r.u32()?;
    }

    Ok(None)
}

/// Read node from body.
pub fn read_node_from_body<T: Default + ReadBody>(r: &mut impl BodyReader) -> Result<T, Error> {
    let mut node = T::default();
    node.read_body(r)?;

    Ok(node)
}
