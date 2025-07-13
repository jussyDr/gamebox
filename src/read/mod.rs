use std::{
    error::Error as StdError,
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

// ERROR //

pub struct Error(Box<dyn StdError>);

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl StdError for Error {}

fn map_io_error(io_error: io::Error) -> Error {
    Error(Box::new(io_error))
}

// READ //

pub trait Readable {
    fn read(header_data: Vec<u8>, node_refs: NodeRefs, body_data: Vec<u8>) -> Result<Self, Error>
    where
        Self: Sized;
}

pub fn read_file<T: Readable>(path: impl AsRef<Path>) -> Result<T, Error> {
    let file = File::open(path).map_err(map_io_error)?;

    read(file)
}

pub fn read<T: Readable>(reader: impl Read) -> Result<T, Error> {
    let mut r = reader;

    // Read the header.

    if r.byte_array()? != [b'G', b'B', b'X'] {
        return Err(Error("unknown file signature".into()));
    }

    if r.u16()? != 6 {
        return Err(Error("unknown file version".into()));
    }

    if r.u8()? != b'B' {
        return Err(Error("unknown file format".into()));
    }

    if r.u8()? != b'U' {
        return Err(Error("unknown refernce table compression".into()));
    }

    let body_compressed = match r.u8()? {
        b'C' => true,
        b'U' => false,
        _ => return Err(Error("unknown body compression".into())),
    };

    if r.u8()? != b'R' {
        return Err(Error("unknown file format".into()));
    }

    let _class_id = r.u32()?;
    let header_data = r.byte_buf()?;

    // Read the external node references.

    let num_node_refs = r
        .u32()?
        .checked_sub(1)
        .ok_or_else(|| Error("number of node references is zero".into()))?;

    let node_refs = NodeRefs::new(num_node_refs as usize);
    let num_external_node_refs = r.u32()?;

    if num_external_node_refs > 0 {
        let _ancestor_level = r.u32()?;
        let folders = read_folders(&mut r)?;

        for _ in 0..num_external_node_refs {
            let _flags = r.u32()?;
            let file_name = r.string()?;

            let node_index = r
                .u32()?
                .checked_sub(1)
                .ok_or_else(|| Error("node reference index is zero".into()))?;

            let _use_file = r.bool32()?;
            let folder_index = r.u32()?;

            let mut path = folders
                .get(folder_index as usize)
                .ok_or_else(|| Error("folder index exceeds number of folders".into()))?
                .clone();

            path.push(&file_name);
        }
    }

    // Read the body.

    let body_data = if body_compressed {
        let body_data_size = r.u32()?;
        let compressed_body_data = r.byte_buf()?;

        let mut body_data = vec![0; body_data_size as usize];
        lzo1x::decompress(&compressed_body_data, &mut body_data)
            .map_err(|decompress_error| Error(Box::new(decompress_error)))?;

        body_data
    } else {
        let mut body_data = vec![];
        r.read_to_end(&mut body_data).map_err(map_io_error)?;

        body_data
    };

    // Read the node.

    T::read(header_data, node_refs, body_data)
}

fn read_folders(r: &mut impl Reader) -> Result<Vec<PathBuf>, Error> {
    let mut folders = vec![];
    folders.push(PathBuf::new());

    let num_folders = r.u32()?;

    for _ in 0..num_folders {
        let name = r.string()?;
        let sub_folders = read_folders(r)?;

        for sub_folder in sub_folders {
            let mut folder = PathBuf::from(name.clone());
            folder.push(sub_folder);
            folders.push(folder);
        }
    }

    Ok(folders)
}

// READER //

trait Reader: Read {
    fn bytes(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![0; n];
        self.read_exact(&mut bytes).map_err(map_io_error)?;

        Ok(bytes)
    }

    fn byte_array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut byte_array = [0; N];
        self.read_exact(&mut byte_array).map_err(map_io_error)?;

        Ok(byte_array)
    }

    fn u8(&mut self) -> Result<u8, Error> {
        let bytes = self.byte_array()?;

        Ok(u8::from_le_bytes(bytes))
    }

    fn u16(&mut self) -> Result<u16, Error> {
        let bytes = self.byte_array()?;

        Ok(u16::from_le_bytes(bytes))
    }

    fn u32(&mut self) -> Result<u32, Error> {
        let bytes = self.byte_array()?;

        Ok(u32::from_le_bytes(bytes))
    }

    fn bool32(&mut self) -> Result<bool, Error> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error("expected a 32-bit boolean".into())),
        }
    }

    fn byte_buf(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.u32()?;

        Reader::bytes(self, size as usize)
    }

    fn string(&mut self) -> Result<String, Error> {
        let bytes = self.byte_buf()?;

        String::from_utf8(bytes).map_err(|utf8_error| Error(utf8_error.into()))
    }
}

impl<T: Read> Reader for T {}

// NODE REFS //

#[derive(Clone)]
pub struct NodeRefs;

impl NodeRefs {
    fn new(num: usize) -> Self {
        Self
    }
}
