use crate::{
    read::{readable::ReadAbstract, Reader},
    Error,
};

/// A media block.
pub enum MediaBlock {}

impl ReadAbstract for MediaBlock {
    fn read_abstract<R, I, N>(r: &mut Reader<R, I, N>, class_id: u32) -> Result<Self, Error> {
        match class_id {
            _ => return Err(Error),
        }
    }
}
