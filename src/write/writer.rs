use std::io::Write;

use crate::Error;

/// Low-level GameBox writer.
pub struct Writer<W> {
    inner: W,
}

impl<W> Writer<W> {
    /// Create a new writer.
    pub const fn new(inner: W) -> Self {
        Self { inner }
    }
}

impl<W: Write> Writer<W> {
    /// Write the given `bytes`.
    pub fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> Result<(), Error> {
        self.inner.write_all(bytes.as_ref()).map_err(|_| Error)
    }

    /// Write an unsigned 8-bit integer.
    pub fn u8(&mut self, value: u8) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write an unsigned 16-bit integer.
    pub fn u16(&mut self, value: u16) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write an unsigned 32-bit integer.
    pub fn u32(&mut self, value: u32) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write an unsigned 64-bit integer.
    pub fn u64(&mut self, value: u64) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }

    /// Write an unsigned 128-bit integer.
    pub fn u128(&mut self, value: u128) -> Result<(), Error> {
        self.bytes(value.to_le_bytes())
    }
}
