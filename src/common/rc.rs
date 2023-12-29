use std::{ops::Deref, path::Path, rc::Rc};

#[derive(Default)]
pub struct RcStr(Option<Rc<str>>);

impl RcStr {
    pub fn as_str(&self) -> &str {
        self
    }
}

impl Clone for RcStr {
    fn clone(&self) -> Self {
        match self.0 {
            None => Self(None),
            Some(ref rc_str) => Self(Some(Rc::clone(rc_str))),
        }
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

impl From<String> for RcStr {
    fn from(s: String) -> Self {
        if s.is_empty() {
            Self(None)
        } else {
            Self(Some(Rc::from(s)))
        }
    }
}

#[derive(Default)]
pub struct RcPath(Option<Rc<Path>>);

impl Deref for RcPath {
    type Target = Path;

    fn deref(&self) -> &Path {
        match self.0 {
            None => Path::new(""),
            Some(ref rc_path) => rc_path,
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
