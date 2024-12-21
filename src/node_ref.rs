use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Reference to a node.
pub enum NodeRef<T: ?Sized> {
    /// Internal node referece.
    Internal(InternalNodeRef<T>),
    /// External node referece.
    External(ExternalNodeRef<T>),
}

impl<T> NodeRef<T> {}

impl<T: Default> Default for NodeRef<T> {
    fn default() -> Self {
        Self::Internal(Default::default())
    }
}

#[derive(Default)]
pub struct InternalNodeRef<T: ?Sized> {
    pub(crate) node: Arc<T>,
}

/// Reference to a node in an external file.
pub struct ExternalNodeRef<T: ?Sized> {
    pub(crate) path: Arc<Path>,
    pub(crate) ancestor_level: u8,
    pub(crate) phantom: PhantomData<T>,
}

impl<T> ExternalNodeRef<T> {
    pub fn path(&self, source_path: &Path) -> PathBuf {
        let mut path = source_path.to_path_buf();

        path.pop();

        for _ in 0..self.ancestor_level {
            path.pop();
        }

        path.push(self.path.clone());

        path
    }
}

impl<T> Default for ExternalNodeRef<T> {
    fn default() -> Self {
        Self {
            path: PathBuf::new().into(),
            ancestor_level: 0,
            phantom: PhantomData,
        }
    }
}
