use std::io::Read;

use crate::read::{
    deserialize::{Deserializer, IdStateMut},
    Result,
};

use super::Collector;

impl Collector {
    pub(crate) fn read_chunk_2e001003<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let _collection = d.id_or_null()?; // null
        d.u32()?; // 26
        let _author = d.id()?; // "r-brwiQCRnOZ2PIHcM0Q8A"
        d.u32()?; // 8
        d.string()?; // "Items"
        d.u32()?; // 0xffffffff
        d.u32()?; // 8
        d.u16()?; // 1
        d.string()?; // "New Item"
        d.u8()?; // 3

        Ok(())
    }

    pub(crate) fn read_chunk_2e001004<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let icon_width = d.u16()?; // 64
        let icon_height = d.u16()?; // 64
        d.bytes(icon_width as usize * icon_height as usize * 4)?;

        Ok(())
    }

    pub(crate) fn read_chunk_2e001006<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    pub(crate) fn read_chunk_2e001009<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.string()?; // "Items"
        d.u32()?; // 0
        d.id_or_null()?; // null

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100b<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let _collection = d.id_or_null()?; // "Fall"
        d.u32()?; // 26
        let _author = d.id()?; // "Nadeo" | "r-brwiQCRnOZ2PIHcM0Q8A"

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100c<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.string()?; // "New Item"

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100d<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.string()?; // "No Description"

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100e<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0

        Ok(())
    }

    pub(crate) fn read_chunk_2e001010<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 4
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0xffffffff

        Ok(())
    }

    pub(crate) fn read_chunk_2e001011<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 1
        d.u8()?; // 3

        Ok(())
    }

    pub(crate) fn read_chunk_2e001012<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 1
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}
