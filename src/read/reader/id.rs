use std::sync::Arc;

/// Identifier table.
pub struct IdTable {
    pub seen_id: bool,
    pub ids: Vec<Arc<str>>,
}

impl IdTable {
    /// Create a new `IdTable`.
    pub fn new() -> Self {
        Self {
            seen_id: false,
            ids: vec![],
        }
    }
}

impl AsMut<IdTable> for IdTable {
    fn as_mut(&mut self) -> &mut IdTable {
        self
    }
}

impl Default for IdTable {
    fn default() -> Self {
        Self::new()
    }
}
