use std::{
    fmt::Debug,
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Reference to a node.
#[derive(Debug)]
pub enum NodeRef<T: ?Sized> {
    /// Internal node referece.
    Internal(Arc<T>),
    /// External node referece.
    External(ExternalNodeRef<T>),
}

impl<T> NodeRef<T> {}

impl<T: Default> Default for NodeRef<T> {
    fn default() -> Self {
        Self::Internal(Default::default())
    }
}

/// Reference to a node in an external file.
pub struct ExternalNodeRef<T: ?Sized> {
    pub(crate) ancestor_level: u8,
    pub(crate) use_file: bool,
    pub(crate) path: Arc<Path>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T> ExternalNodeRef<T> {
    /// Path.
    pub fn path(&self, source_path: &Path) -> PathBuf {
        let mut path = source_path.to_path_buf();

        path.pop();

        for _ in 0..self.ancestor_level {
            path.pop();
        }

        path.push(&self.path);

        path
    }
}

impl<T> Clone for ExternalNodeRef<T> {
    fn clone(&self) -> Self {
        Self {
            ancestor_level: self.ancestor_level,
            use_file: self.use_file,
            path: Arc::clone(&self.path),
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Debug for ExternalNodeRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ExternalNodeRef {{ path: {:?}, ancestor_level: {:?} }}",
            self.path, self.ancestor_level
        )
    }
}

impl<T> Default for ExternalNodeRef<T> {
    fn default() -> Self {
        Self {
            ancestor_level: 0,
            use_file: false,
            path: PathBuf::new().into(),
            phantom: PhantomData,
        }
    }
}
