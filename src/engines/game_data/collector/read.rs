use std::io::Read;

use crate::{
    read::Result,
    read::{IdStateMut, Reader},
};

use super::{Collector, Icon};

impl Collector {
    pub(crate) fn read_chunk_2e001003<R: Read, I: IdStateMut, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        let _collection = r.id_or_null()?;
        r.id_or_null()?; // "Stadium"
        let _author = r.id()?;
        r.u32()?; // 8
        r.string()?; // "Items"
        r.u32()?; // 0xffffffff
        r.u32()?; // 8
        r.u16()?; // 1
        self.name = r.string()?;
        r.u8()?; // 3

        Ok(())
    }

    pub(crate) fn read_chunk_2e001004<R: Read, I, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        let icon_width = r.u16()?;
        let icon_height = r.u16()?;

        if icon_width & 0x8000 != 0 || icon_height & 0x800 != 0 {
            r.u16()?;
            let size = r.u32()?;
            let data = r.bytes(size as usize)?;

            self.icon = Icon::WebP(data);
        } else {
            let icon_data = r.bytes(icon_width as usize * icon_height as usize * 4)?;

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
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }

    pub(crate) fn read_chunk_2e001009<R: Read, I: IdStateMut, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.string()?; // "Items"
        r.u32()?; // 0
        r.id_or_null()?; // null

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100b<R: Read, I: IdStateMut, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        self.collection = r.id_or_null()?;
        r.id_or_null()?;
        let _author = r.id()?;

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100c<R: Read, I, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        self.name = r.string()?;

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100d<R: Read, I, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        self.description = r.string()?;

        Ok(())
    }

    pub(crate) fn read_chunk_2e00100e<R: Read, I, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 1
        r.u32()?; // 0

        Ok(())
    }

    pub(crate) fn read_chunk_2e001010<R: Read, I, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 4
        r.u32()?; // 0xffffffff
        r.u32()?; // 0
        r.u32()?; // 0xffffffff

        Ok(())
    }

    pub(crate) fn read_chunk_2e001011<R: Read, I, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 1
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 1
        r.u8()?; // 3

        Ok(())
    }

    pub(crate) fn read_chunk_2e001012<R: Read, I, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 1
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }
}
