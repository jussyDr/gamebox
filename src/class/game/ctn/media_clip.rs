use std::{any::Any, cell::OnceCell, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    game::ctn::MediaTrack,
    read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode},
};

pub struct MediaClip(Inner);

#[self_referencing]
struct Inner {
    body_data: Arc<[u8]>,
    node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    #[borrows(body_data, node_refs)]
    #[covariant]
    chunks: Chunks<'this>,
}

struct Chunks<'a> {
    chunk_13: Chunk13<'a>,
}

struct Chunk13<'a> {
    tracks: Box<[&'a MediaTrack]>,
    name: &'a str,
}

impl MediaClip {
    pub fn tracks(&self) -> &[&MediaTrack] {
        &self.0.borrow_chunks().chunk_13.tracks
    }

    pub fn name(&self) -> &str {
        self.0.borrow_chunks().chunk_13.name
    }
}

impl ClassId for MediaClip {
    const CLASS_ID: u32 = 0x03079000;
}

impl ReadNode for MediaClip {
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

                let chunk_13 = r.chunk(0x0307900d, Chunk13::read)?;

                r.end()?;

                Ok(Chunks { chunk_13 })
            },
        };

        builder.try_build().map(Self)
    }
}

impl<'a> Chunk13<'a> {
    fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        let tracks = r.list_with_version(|r| r.node_ref())?;
        let name = r.string()?;
        let _stop_when_leave = r.bool32()?;
        r.bool32()?;
        let _step_when_respawn = r.bool32()?;
        r.string()?;
        r.f32()?;
        let _local_player_clip_ent_index = r.u32()?;

        Ok(Self { tracks, name })
    }
}
