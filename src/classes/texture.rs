//! Types used for reading [Texture] nodes.

use std::{
    io::{BufRead, Read, Seek},
    path::Path,
};

use crate::{
    class::ClassId,
    read::{
        deserialize::{Deserializer, IdStateMut, NodeStateMut},
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
    EngineId, RcPath,
};

/// Node type corresponding to GameBox files with the extension `Texture.Gbx`.
#[derive(Default)]
pub struct Texture {
    image_path: RcPath,
}

impl Texture {
    pub fn image_path(&self) -> &Path {
        &self.image_path
    }
}

impl ClassId for Texture {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x011;
}

impl Readable for Texture {}

impl Sealed for Texture {
    fn read(
        reader: impl BufRead + Seek,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for Texture {
    fn header_chunks<R: Read>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>> {
        [].into_iter()
    }
}

impl ReadBody for Texture {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for Texture {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x09011019,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011019(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011020,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011020(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011023,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011023(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011025,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011025(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011028,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011028(n, d)),
            },
            BodyChunkEntry {
                id: 0x0901102a,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0901102a(n, d)),
            },
            BodyChunkEntry {
                id: 0x0901102c,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0901102c(n, d)),
            },
            BodyChunkEntry {
                id: 0x0901102d,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0901102d(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011030,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011030(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011032,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011032(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011033,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011033(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011034,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011034(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011035,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011035(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011036,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011036(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011037,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011037(n, d)),
            },
            BodyChunkEntry {
                id: 0x09011038,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09011038(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Texture {
    fn read_chunk_09011019<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0x3f000000

        Ok(())
    }

    fn read_chunk_09011020<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011023<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011025<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.f32()?; // 1.0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xff000000

        Ok(())
    }

    fn read_chunk_09011028<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_0901102a<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0901102c<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0901102d<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0x3c00

        Ok(())
    }

    fn read_chunk_09011030<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 5
        self.image_path = d.external_node_ref()?.into();
        d.u32()?; // 0x2c0000
        d.u32()?; // 0x10004e12
        d.u32()?; // 0x08003680
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011032<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0x48000800

        Ok(())
    }

    fn read_chunk_09011033<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.f32()?; // 1.0

        Ok(())
    }

    fn read_chunk_09011034<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 4
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011035<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u16()?; // 0

        Ok(())
    }

    fn read_chunk_09011036<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0xffffffff
        d.u32()?; // 3
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_09011037<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011038<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0

        Ok(())
    }
}
