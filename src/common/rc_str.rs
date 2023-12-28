use std::{ops::Deref, rc::Rc};

/// Reference counted string.
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
