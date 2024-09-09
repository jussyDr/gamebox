//! Writing GameBox files.

/// Writable to a GameBox file.
pub trait Writable: writable::Sealed {}

mod writable {
    pub trait Sealed {}
}
