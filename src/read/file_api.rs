use std::{io::Read, path::PathBuf};

use crate::{
    common::{Compression, FileFormat, GAMEBOX_FILE_SIGNATURE, GAMEBOX_FILE_VERSION, UNKNOWN_BYTE},
    deserialize::Deserializer,
};

use super::Result;

enum BodyData {
    Compressed {
        data: Vec<u8>,
        decompressed_size: u32,
    },
    Decompressed(Vec<u8>),
}

/// Represents a GameBox file.
pub struct GbxFile {
    class_id: u32,
    header_data: Vec<u8>,
    num_node_refs: u32,
    external_node_refs: Vec<(u32, PathBuf)>,
    body_data: BodyData,
}

impl GbxFile {
    /// Read a GameBox file from the given `reader`.
    pub fn read(reader: impl Read, assume_no_header_data: bool) -> Result<Self> {
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
        let header_data_size = d.u32()?;

        let header_data = if assume_no_header_data {
            vec![]
        } else {
            d.bytes(header_data_size as usize)?
        };

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

        let body_data = match body_compression {
            Compression::Compressed => {
                let decompressed_size = d.u32()?;
                let compressed_size = d.u32()?;
                let data = d.bytes(compressed_size as usize)?;

                BodyData::Compressed {
                    data,
                    decompressed_size,
                }
            }
            Compression::Uncompressed => BodyData::Decompressed(d.read_to_end()?),
        };

        Ok(Self {
            class_id,
            header_data,
            num_node_refs,
            external_node_refs,
            body_data,
        })
    }

    /// Class identifier of the node serialized in this GameBox file.
    pub const fn class_id(&self) -> u32 {
        self.class_id
    }

    /// Raw header data of this GameBox file.
    pub fn header_data(&mut self) -> &[u8] {
        &self.header_data
    }

    /// Number of nodes that are referenced.
    pub const fn num_node_refs(&self) -> u32 {
        self.num_node_refs
    }

    /// The external nodes that are referenced.
    pub fn external_node_refs(&self) -> &[(u32, PathBuf)] {
        &self.external_node_refs
    }

    /// Raw uncompressed body data.
    pub fn body_data(&mut self) -> Result<&[u8]> {
        match self.body_data {
            BodyData::Compressed {
                ref data,
                decompressed_size,
            } => {
                let mut decompressed_data = vec![0; decompressed_size as usize];

                lzo1x::decompress(data, &mut decompressed_data)
                    .map_err(|_| "decompression failed")?;

                self.body_data = BodyData::Decompressed(decompressed_data);

                match self.body_data {
                    BodyData::Decompressed(ref data) => Ok(data),
                    BodyData::Compressed { .. } => unreachable!(),
                }
            }
            BodyData::Decompressed(ref data) => Ok(data),
        }
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
