use crate::read::readable::{BodyChunk, BodyChunks};

/// TODO.
#[derive(Default)]
pub struct VehicleCarMarksSamples;

impl BodyChunks for VehicleCarMarksSamples {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        [].into_iter()
    }
}
