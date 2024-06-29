//! Types used for reading [Texture] nodes.

use std::{io::Read, path::Path};

use crate::{
    common::{Class, ClassId, EngineId},
    read::{
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
    read::{NodeStateMut, Reader},
    RcPath,
};

/// Node type corresponding to GameBox files with the extension `Texture.Gbx`.
#[derive(Default, Debug)]
pub struct Texture {
    image_path: RcPath,
}

impl Texture {
    /// Path to the texture image file.
    pub fn image_path(&self) -> &Path {
        &self.image_path
    }
}

impl Class for Texture {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 17);
}

impl Readable for Texture {}

impl Sealed for Texture {
    fn read(
        reader: impl Read,
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

impl<R: Read, I, N: NodeStateMut> ReadBody<R, I, N> for Texture {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I, N: NodeStateMut> BodyChunks<R, I, N> for Texture {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x09011019,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011019(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011020,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011020(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011023,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011023(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011025,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011025(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011028,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011028(n, r)),
            },
            BodyChunkEntry {
                id: 0x0901102a,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0901102a(n, r)),
            },
            BodyChunkEntry {
                id: 0x0901102c,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0901102c(n, r)),
            },
            BodyChunkEntry {
                id: 0x0901102d,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0901102d(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011030,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011030(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011032,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011032(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011033,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011033(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011034,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011034(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011035,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011035(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011036,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011036(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011037,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011037(n, r)),
            },
            BodyChunkEntry {
                id: 0x09011038,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09011038(n, r)),
            },
        ]
        .into_iter()
    }
}

impl Texture {
    fn read_chunk_09011019<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0x3f000000

        Ok(())
    }

    fn read_chunk_09011020<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011023<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011025<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.f32()?; // 1.0
        r.f32()?; // 1.0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0xff000000

        Ok(())
    }

    fn read_chunk_09011028<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 1
        r.u32()?; // 1

        Ok(())
    }

    fn read_chunk_0901102a<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0901102c<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0901102d<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 0x3c00

        Ok(())
    }

    fn read_chunk_09011030<R: Read, I, N: NodeStateMut>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 5
        self.image_path = r.external_node_ref()?.into();
        r.u32()?; // 0x2c0000
        r.u32()?; // 0x10004e12
        r.u32()?; // 0x08003680
        r.u32()?; // 0
        r.f32()?; // 1.0
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011032<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 0x48000800

        Ok(())
    }

    fn read_chunk_09011033<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.f32()?; // 1.0

        Ok(())
    }

    fn read_chunk_09011034<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 4
        r.u32()?; // 0xffffffff
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0xffffffff
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011035<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.u16()?; // 0

        Ok(())
    }

    fn read_chunk_09011036<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 1
        r.u32()?; // 0xffffffff
        r.u32()?; // 3
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_09011037<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09011038<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 0xffffffff
        r.u32()?; // 0

        Ok(())
    }
}
