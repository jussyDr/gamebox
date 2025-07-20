use std::{any::Any, cell::OnceCell, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    U32Vec3,
    game::ctn::MediaClip,
    read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode},
};

pub struct MediaClipGroup(Inner);

#[self_referencing]
struct Inner {
    body_data: Arc<[u8]>,
    node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    #[borrows(body_data, node_refs)]
    #[covariant]
    chunks: Chunks<'this>,
}

struct Chunks<'a> {
    chunk_3: Chunk3<'a>,
}

struct Chunk3<'a> {
    clips: Box<[Clip<'a>]>,
}

pub struct Clip<'a> {
    clip: &'a MediaClip,
    trigger_coords: Box<[U32Vec3]>,
}

impl MediaClipGroup {
    pub fn clips(&self) -> &[Clip] {
        &self.0.borrow_chunks().chunk_3.clips
    }
}

impl Clip<'_> {
    pub fn clip(&self) -> &MediaClip {
        self.clip
    }

    pub fn trigger_coords(&self) -> &[U32Vec3] {
        &self.trigger_coords
    }
}

impl ClassId for MediaClipGroup {
    const CLASS_ID: u32 = 0x0307a000;
}

impl ReadNode for MediaClipGroup {
    fn read_from_body(
        body_data: Arc<[u8]>,
        body_data_offset: &mut usize,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
        seen_id: &mut bool,
        ids: &mut Vec<(usize, usize)>,
    ) -> Result<Self, Error> {
        let builder = InnerTryBuilder {
            body_data,
            node_refs,
            chunks_builder: |body_data, node_refs| {
                let mut br = BodyReader::new(body_data, body_data_offset, node_refs, seen_id, ids);
                let mut r = BodyChunksReader(&mut br);

                let chunk_3 = r.chunk(0x0307a003, Chunk3::read)?;

                r.end()?;

                Ok(Chunks { chunk_3 })
            },
        };

        builder.try_build().map(Self)
    }
}

impl<'a> Chunk3<'a> {
    fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let clips = r.list_with_version(|r| r.node_ref())?;

        if r.u32()? as usize != clips.len() {
            todo!()
        }

        let clips = clips
            .iter()
            .map(|clip| {
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                let _condition = r.u32()?;
                let _condition_value = r.f32()?;
                let trigger_coords = r.list(|r| r.vec3_u32())?;

                Ok(Clip {
                    clip,
                    trigger_coords,
                })
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self { clips })
    }
}
