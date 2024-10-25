use crate::read::{
    readable::{self, BodyChunks, UserDataChunk, UserDataChunks},
    Readable,
};

/// An item model.
#[derive(Default)]
pub struct ItemModel;

impl Readable for ItemModel {}

impl readable::Sealed for ItemModel {}

impl UserDataChunks for ItemModel {
    fn user_data_chunks() -> impl Iterator<Item = UserDataChunk<Self>> {
        [].into_iter()
    }
}

impl BodyChunks for ItemModel {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R, I, N>() -> impl Iterator<Item = readable::BodyChunk<Self, R, I, N>> {
        [].into_iter()
    }
}
