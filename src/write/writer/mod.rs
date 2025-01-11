//! Low-level GameBox writer.

mod id;
mod node;

pub use id::{IdState, IdStateMut};
pub use node::{NodeState, NodeStateMut};

use std::io::Write;

use bytemuck::{bytes_of, Pod};
use ordered_float::OrderedFloat;

use crate::{Byte3, FileRef, Nat3, Vec2, Vec3, YawPitchRoll};

use super::Error;

pub trait ToLe {
    /// Convert `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op.
    fn to_le(self) -> Self;
}

impl ToLe for u8 {
    fn to_le(self) -> Self {
        self
    }
}

impl ToLe for u16 {
    fn to_le(self) -> Self {
        self.to_le()
    }
}

impl ToLe for u32 {
    fn to_le(self) -> Self {
        self.to_le()
    }
}

impl ToLe for u64 {
    fn to_le(self) -> Self {
        self.to_le()
    }
}

impl ToLe for i32 {
    fn to_le(self) -> Self {
        self.to_le()
    }
}

impl ToLe for f32 {
    fn to_le(self) -> Self {
        Self::from_bits(self.to_bits().to_le())
    }
}

impl<T: ToLe> ToLe for OrderedFloat<T> {
    fn to_le(self) -> Self {
        Self(self.0.to_le())
    }
}

/// Low-level GameBox writer.
pub struct Writer<W, I, N> {
    inner: W,
    id_state: I,
    node_state: N,
}

impl<W, I, N> Writer<W, I, N> {
    /// Create a new writer.
    pub const fn new(inner: W, id_state: I, node_state: N) -> Self {
        Self {
            inner,
            id_state,
            node_state,
        }
    }

    pub fn into_inner(self) -> W {
        self.inner
    }

    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    pub fn to_buf_inline(
        &mut self,
        mut write_fn: impl FnMut(&mut Writer<Vec<u8>, &mut I, &mut N>) -> Result<(), Error>,
    ) -> Result<Vec<u8>, Error> {
        let mut w = Writer::new(vec![], &mut self.id_state, &mut self.node_state);

        write_fn(&mut w)?;

        Ok(w.inner)
    }
}

impl<W: Write, I, N> Writer<W, I, N> {
    /// Write bytes.
    pub fn bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.inner.write_all(bytes).map_err(Error::io)?;

        Ok(())
    }

    pub fn pod<T: Pod + ToLe>(&mut self, value: T) -> Result<(), Error> {
        self.bytes(bytes_of(&value.to_le()))
    }

    /// Write an unsigned 8-bit integer.
    pub fn u8(&mut self, value: u8) -> Result<(), Error> {
        self.pod(value)
    }

    /// Write an unsigned 16-bit integer.
    pub fn u16(&mut self, value: u16) -> Result<(), Error> {
        self.pod(value)
    }

    /// Write an unsigned 32-bit integer.
    pub fn u32(&mut self, value: u32) -> Result<(), Error> {
        self.pod(value)
    }

    /// Write an unsigned 64-bit integer.
    pub fn u64(&mut self, value: u64) -> Result<(), Error> {
        self.pod(value)
    }

    pub fn f32(&mut self, value: f32) -> Result<(), Error> {
        self.pod(value)
    }

    pub fn byte3(&mut self, value: Byte3) -> Result<(), Error> {
        self.pod(value)
    }

    pub fn nat3(&mut self, value: Nat3) -> Result<(), Error> {
        self.pod(value)
    }

    pub fn vec2(&mut self, value: Vec2) -> Result<(), Error> {
        self.pod(value)
    }

    pub fn vec3(&mut self, value: Vec3) -> Result<(), Error> {
        self.pod(value)
    }

    pub fn yaw_pitch_roll(&mut self, value: YawPitchRoll) -> Result<(), Error> {
        self.pod(value)
    }

    pub fn bool(&mut self, value: bool) -> Result<(), Error> {
        self.u32(if value { 1 } else { 0 })
    }

    pub fn bool8(&mut self, value: bool) -> Result<(), Error> {
        self.u8(if value { 1 } else { 0 })
    }

    pub fn byte_buf(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.u32(bytes.len() as u32)?;
        self.bytes(bytes)
    }

    pub fn byte_buf_inline(
        &mut self,
        write_fn: impl FnMut(&mut Writer<Vec<u8>, &mut I, &mut N>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let buf = self.to_buf_inline(write_fn)?;
        self.byte_buf(&buf)
    }

    pub fn string(&mut self, value: &str) -> Result<(), Error> {
        self.byte_buf(value.as_bytes())
    }

    pub fn string_or_empty(&mut self, value: Option<&String>) -> Result<(), Error> {
        match value {
            Some(value) => self.string(value),
            None => self.u32(0),
        }
    }

    pub fn file_ref_or_null(&mut self, file_ref: Option<&FileRef>) -> Result<(), Error> {
        self.u8(3)?;

        match file_ref {
            Some(FileRef::Internal { path }) => {
                let mut checksum = [0; 32];
                checksum[0] = 2;

                self.bytes(&checksum)?;
                self.string(path.to_str().unwrap())?;
                self.string_or_empty(None)?;
            }
            Some(FileRef::External {
                checksum,
                path,
                locator_url,
            }) => {
                self.bytes(checksum)?;
                self.string(path.to_str().unwrap())?;
                self.string(locator_url)?;
            }
            None => {
                self.bytes(&[0; 32])?;
                self.string_or_empty(None)?;
                self.string_or_empty(None)?;
            }
        }

        Ok(())
    }

    pub fn file_ref(&mut self, file_ref: &FileRef) -> Result<(), Error> {
        self.file_ref_or_null(Some(file_ref))
    }

    pub fn list<T>(
        &mut self,
        list: &[T],
        mut write_fn: impl FnMut(&mut Self, &T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.u32(list.len() as u32)?;

        for value in list {
            write_fn(self, value)?;
        }

        Ok(())
    }

    pub fn list_with_version<T>(
        &mut self,
        list: &[T],
        write_fn: impl FnMut(&mut Self, &T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.u32(10)?;
        self.list(list, write_fn)
    }

    pub fn encapsulation(
        &mut self,
        mut write_fn: impl FnMut(&mut Writer<&mut Vec<u8>, IdState, NodeState>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let mut buf = vec![];

        let mut w = Writer {
            inner: &mut buf,
            id_state: IdState::new(),
            node_state: NodeState::new(),
        };

        write_fn(&mut w)?;

        self.byte_buf(&buf)?;

        Ok(())
    }
}
