use std::ops::{Deref, DerefMut};

use crate::read::readable::{BodyChunk, BodyChunks};

use super::MediaBlockTriangles;

/// A 3D triangles media block.
#[derive(Default)]
pub struct MediaBlockTriangles3D {
    media_block_triangles: MediaBlockTriangles,
}

impl Deref for MediaBlockTriangles3D {
    type Target = MediaBlockTriangles;

    fn deref(&self) -> &MediaBlockTriangles {
        &self.media_block_triangles
    }
}

impl DerefMut for MediaBlockTriangles3D {
    fn deref_mut(&mut self) -> &mut MediaBlockTriangles {
        &mut self.media_block_triangles
    }
}

impl BodyChunks for MediaBlockTriangles3D {
    type Parent = MediaBlockTriangles;

    fn parent(&mut self) -> Option<&mut MediaBlockTriangles> {
        Some(self.deref_mut())
    }

    fn body_chunks<R, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        [].into_iter()
    }
}
