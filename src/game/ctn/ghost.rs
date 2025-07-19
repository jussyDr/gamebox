use crate::read::ReadNodeRef;

pub struct Ghost;

impl ReadNodeRef for Ghost {
    fn read_from_body(
        body_data: std::sync::Arc<[u8]>,
        node_refs: std::sync::Arc<[std::cell::OnceCell<Box<dyn std::any::Any>>]>,
        body_data_offset: &mut usize,
    ) -> Result<Self, crate::read::Error>
    where
        Self: Sized,
    {
        todo!()
    }
}
