mod read;
mod reader;
mod write;
mod writer;

use std::io::{Read, Seek, Write};

pub fn test(reader: impl Read + Seek) -> reader::Result<()> {
    read::test(reader)
}

pub fn test2(writer: impl Write) -> writer::Result<()> {
    write::test(writer)
}
