use std::io::Read;

use crate::{
    deserialize::{Deserializer, IdStateMut},
    read::Result,
};

use super::{Collector, Icon};

impl Collector {
    pub(crate) fn read_chunk_2e001003<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let _collection = d.id_or_null()?;
        d.id_or_null()?; // "Stadium"
        let _author = d.id()?;
        d.u32()?; // 8
        d.string()?; // "Items"
        d.u32()?; // 0xffffffff
        d.u32()?; // 8
        d.u16()?; // 1
        self.name = d.string()?;
        d.u8()?; // 3

        Ok(())
    }

    pub(crate) fn read_chunk_2e001004<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let icon_width = d.u16()?;
        let icon_height = d.u16()?;

        if icon_width & 0x8000 != 0 || icon_height & 0x800 != 0 {
            d.u16()?;
            let size = d.u32()?;
            let data = d.bytes(size as usize)?;

            self.icon = Icon::WebP(data);
        } else {
            let icon_data = d.bytes(icon_width as usize * icon_height as usize * 4)?;

            self.icon = Icon::Argb {
                width: icon_width,
                height: icon_height,
                data: icon_data,
            };
        }

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
        self.collection = d.id_or_null()?;
        d.id_or_null()?;
        let _author = d.id()?;

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100c<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        self.name = d.string()?;

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100d<R: Read, I, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        self.description = d.string()?;

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
