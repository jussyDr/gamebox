use std::marker::PhantomData;

use crate::read::{BodyChunksReader, BodyReader, Error};

pub struct WaypointSpecialProperty<'a> {
    body: Body<'a>,
}

pub struct Body<'a> {
    chunks: BodyChunks<'a>,
}

pub struct BodyChunks<'a> {
    chunk_0: Chunk0<'a>,
    chunk_1: Chunk1,
}

struct Chunk0<'a> {
    tag: &'a str,
}

struct Chunk1;

impl WaypointSpecialProperty<'_> {
    pub fn tag(&self) -> &str {
        self.body.chunks.chunk_0.tag
    }
}

impl<'a> WaypointSpecialProperty<'a> {
    pub fn read_from_body(r: BodyReader<'a>) -> Result<WaypointSpecialProperty<'a>, Error> {
        let body = Self::read_body(r)?;

        Ok(WaypointSpecialProperty { body })
    }

    pub fn read_body(r: BodyReader<'a>) -> Result<Body<'a>, Error> {
        let mut r = BodyChunksReader::new(r);

        let chunks = Self::read_body_chunks(&mut r)?;

        Ok(Body { chunks })
    }

    pub fn read_body_chunks(r: &mut BodyChunksReader<'a>) -> Result<BodyChunks<'a>, Error> {
        let chunk_0 = r.chunk(0x2e009000, |r| {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::unknown_chunk_version(version));
            }

            let tag = r.string()?;
            let _order = r.u32()?;

            Ok(Chunk0 { tag })
        })?;

        let chunk_1 = r.skippable_chunk(0x2e009001, |r| {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::unknown_chunk_version(version));
            }

            if r.bool32()? {
                todo!();
            }

            Ok(Chunk1)
        })?;

        Ok(BodyChunks { chunk_0, chunk_1 })
    }
}
