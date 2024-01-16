use std::io::Write;

use crate::{
    serialize::{IdStateMut, Serializer},
    write::Result,
};

use super::Collector;

impl Collector {
    pub(crate) fn write_chunk_3<W: Write, I: IdStateMut, N>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result {
        s.null_id()?;
        s.u32(26)?;
        s.id("r-brwiQCRnOZ2PIHcM0Q8A")?;
        s.u32(8)?;
        s.string("Items")?;
        s.u32(0xffffffff)?;
        s.u32(8)?;
        s.u16(1)?;
        s.string("New Item")?;
        s.u8(3)?;

        Ok(())
    }

    pub(crate) fn write_chunk_6<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0)?;
        s.u32(0)?;

        Ok(())
    }

    pub(crate) fn write_chunk_9<W: Write, I: IdStateMut, N>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result {
        s.u32(0x2e001009)?;
        s.string("Items")?;
        s.u32(0)?;
        s.null_id()?;

        Ok(())
    }

    pub(crate) fn write_chunk_11<W: Write, I: IdStateMut, N>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result {
        s.u32(0x2e00100b)?;
        s.u32(0xffffffff)?;
        s.u32(26)?;
        s.id("r-brwiQCRnOZ2PIHcM0Q8A")?;

        Ok(())
    }

    pub(crate) fn write_chunk_12<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e00100c)?;
        s.string("New Item")?;

        Ok(())
    }

    pub(crate) fn write_chunk_13<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e00100d)?;
        s.string("No Description")?;

        Ok(())
    }

    pub(crate) fn write_chunk_16<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e001010)?;
        s.u32(4)?;
        s.u32(0xffffffff)?;
        s.u32(0)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    pub(crate) fn write_chunk_17<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e001011)?;
        s.u32(1)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(1)?;
        s.u8(3)?;

        Ok(())
    }

    pub(crate) fn write_chunk_18<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e001012)?;
        s.u32(0)?;
        s.u32(1)?;
        s.u32(0)?;
        s.u32(0)?;

        Ok(())
    }
}
