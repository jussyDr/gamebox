use std::{
    ops::Deref,
    path::{Path, PathBuf},
    rc::Rc,
};

/// A reference counted string.
#[derive(Clone, Default)]
pub struct RcStr(Option<Rc<str>>);

impl RcStr {
    /// Extracts a string slice containing the entire `RcStr`.
    pub fn as_str(&self) -> &str {
        self
    }
}

impl Deref for RcStr {
    type Target = str;

    fn deref(&self) -> &str {
        match self.0 {
            None => "",
            Some(ref rc_str) => rc_str,
        }
    }
}

impl From<&str> for RcStr {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self(None)
        } else {
            Self(Some(s.into()))
        }
    }
}

impl From<Rc<str>> for RcStr {
    fn from(rc_str: Rc<str>) -> Self {
        if rc_str.is_empty() {
            Self(None)
        } else {
            Self(Some(rc_str))
        }
    }
}

impl From<String> for RcStr {
    fn from(string: String) -> Self {
        if string.is_empty() {
            Self(None)
        } else {
            Self(Some(string.into()))
        }
    }
}

/// A reference counted path.
#[derive(Clone, Default)]
pub struct RcPath(Option<Rc<Path>>);

impl RcPath {
    /// Coerces to a `Path` slice.
    pub fn as_path(&self) -> &Path {
        self
    }
}

impl Deref for RcPath {
    type Target = Path;

    fn deref(&self) -> &Path {
        match self.0 {
            None => Path::new(""),
            Some(ref rc_path) => rc_path,
        }
    }
}

impl From<&Path> for RcPath {
    fn from(path: &Path) -> Self {
        if path.as_os_str().is_empty() {
            Self(None)
        } else {
            Self(Some(path.into()))
        }
    }
}

impl From<Rc<Path>> for RcPath {
    fn from(rc_path: Rc<Path>) -> Self {
        if rc_path.as_os_str().is_empty() {
            Self(None)
        } else {
            Self(Some(rc_path))
        }
    }
}

impl From<PathBuf> for RcPath {
    fn from(path_buf: PathBuf) -> Self {
        if path_buf.as_os_str().is_empty() {
            Self(None)
        } else {
            Self(Some(path_buf.into()))
        }
    }
}
