use std::{
    cmp::min,
    io::{BufRead, Read, Result, Seek, SeekFrom},
};

pub fn take<R>(reader: R, limit: u64) -> Take<R> {
    Take { reader, limit }
}

/// Adapter which limits the amount of bytes that can be read from a given reader.
pub struct Take<R> {
    reader: R,
    limit: u64,
}

impl<R: Read> Read for Take<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.limit == 0 {
            return Ok(0);
        }

        let max = min(buf.len() as u64, self.limit) as usize;
        let n = self.reader.read(&mut buf[..max])?;
        assert!(n as u64 <= self.limit, "number of read bytes exceeds limit");
        self.limit -= n as u64;

        Ok(n)
    }
}

impl<R: BufRead> BufRead for Take<R> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        if self.limit == 0 {
            return Ok(&[]);
        }

        let buf = self.reader.fill_buf()?;
        let cap = min(buf.len() as u64, self.limit) as usize;
        Ok(&buf[..cap])
    }

    fn consume(&mut self, amt: usize) {
        let amt = min(amt as u64, self.limit) as usize;
        self.limit -= amt as u64;
        self.reader.consume(amt);
    }
}

impl<R: Seek> Seek for Take<R> {
    fn seek(&mut self, _pos: SeekFrom) -> Result<u64> {
        unimplemented!()
    }
}
