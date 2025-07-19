use std::{any::Any, cell::OnceCell, sync::Arc};

use crate::read::{ClassId, Error, ReadNode};

pub struct Ghost;

impl ClassId for Ghost {
    const CLASS_ID: u32 = 0x03092000;
}

impl ReadNode for Ghost {
    fn read_from_body(
        body_data: Arc<[u8]>,
        body_data_offset: &mut usize,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
        seen_id: &mut bool,
        ids: &mut Vec<(usize, usize)>,
    ) -> Result<Self, Error> {
        todo!()
    }
}
