mod body;
pub use body::{BodyWriter, BodyWriterImpl};

mod header;
pub use header::{HeaderWriter, HeaderWriterImpl};

use std::io;

pub trait Writer: io::Write {
    fn u8(&mut self, value: u8) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())
    }

    fn u16(&mut self, value: u16) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())
    }

    fn u32(&mut self, value: u32) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())
    }

    fn list_u8(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.u32(bytes.len() as u32)?;
        self.write_all(bytes)
    }
}

impl<T: io::Write> Writer for T {}
