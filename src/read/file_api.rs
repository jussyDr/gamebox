use std::{
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::{
    common::{Compression, FileFormat, GAMEBOX_FILE_SIGNATURE, GAMEBOX_FILE_VERSION, UNKNOWN_BYTE},
    deserialize::Deserializer,
};

use super::{
    take::{take, Take},
    Result,
};

pub struct GbxFile<R> {
    reader: R,
    class_id: u32,
    header_data_start: u64,
    header_data_size: u32,
    num_node_refs: u32,
    external_node_refs: Vec<(u32, PathBuf)>,
    body: Vec<u8>,
}

impl<R> GbxFile<R> {
    pub const fn class_id(&self) -> u32 {
        self.class_id
    }

    pub const fn num_node_refs(&self) -> u32 {
        self.num_node_refs
    }

    pub fn external_node_refs(&self) -> &[(u32, PathBuf)] {
        &self.external_node_refs
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

impl<R: Read + Seek> GbxFile<R> {
    pub fn new(reader: R, assume_no_header_data: bool) -> Result<Self> {
        let mut d = Deserializer::new(reader, (), ());

        if d.byte_array()? != GAMEBOX_FILE_SIGNATURE {
            return Err("not a gamebox file".into());
        }

        if d.u16()? != GAMEBOX_FILE_VERSION {
            return Err("unsupported gamebox version".into());
        }

        let format = FileFormat::read(&mut d)?;

        if let FileFormat::Text = format {
            return Err("text format is not supported".into());
        }

        let ref_table_compression = Compression::read(&mut d)?;

        if let Compression::Compressed = ref_table_compression {
            return Err("compressed reference table is not supported".into());
        }

        let body_compression = Compression::read(&mut d)?;

        if d.u8()? != UNKNOWN_BYTE {
            return Err("invalid unknown byte".into());
        }

        let class_id = d.u32()?;
        let mut header_data_size = d.u32()?;

        let header_data_start = d.position()?;

        if assume_no_header_data {
            header_data_size = 0;
        } else {
            d.skip(header_data_size)?;
        }

        let num_node_refs = d.u32()?;
        let num_external_node_refs = d.u32()?;

        let external_node_refs = if num_external_node_refs > 0 {
            d.u32()?;
            let mut folders = vec![];
            read_folders(&mut d, PathBuf::new(), &mut folders)?;

            d.repeat(num_external_node_refs as usize, |d| {
                d.u32()?;
                let file_name = d.string()?;
                let node_index = d.u32()?;

                if node_index == 0 {
                    return Err("".into());
                }

                d.u32()?;
                let folder_index = d.u32()?;

                let mut file_path = folders[folder_index as usize].clone();
                file_path.push(file_name);

                Ok((node_index - 1, file_path))
            })?
        } else {
            vec![]
        };

        let body = match body_compression {
            Compression::Compressed => {
                let body_size = d.u32()?;
                let compressed_body_size = d.u32()?;
                let compressed_body = d.bytes(compressed_body_size as usize)?;
                let mut body = vec![0; body_size as usize];

                lzo1x::decompress(&compressed_body, &mut body)
                    .map_err(|_| "decompression failed")?;

                body
            }
            Compression::Uncompressed => d.read_to_end()?,
        };

        Ok(Self {
            reader: d.into_reader(),
            class_id,
            header_data_start,
            header_data_size,
            num_node_refs,
            external_node_refs,
            body,
        })
    }

    pub fn header_data(&mut self) -> Result<Take<&mut R>> {
        self.reader.seek(SeekFrom::Start(self.header_data_start))?;
        let reader = take(&mut self.reader, self.header_data_size as u64);

        Ok(reader)
    }
}

fn read_folders<R: Read, I, N>(
    d: &mut Deserializer<R, I, N>,
    path: PathBuf,
    folders: &mut Vec<PathBuf>,
) -> Result<()> {
    folders.push(path.clone());

    d.list(|d| {
        let folder_name = d.string()?;

        let mut path = path.clone();
        path.push(folder_name);

        read_folders(d, path, folders)
    })?;

    Ok(())
}
