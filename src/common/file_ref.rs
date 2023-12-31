use std::path::{Path, PathBuf};

const FILE_REF_VERSION: u8 = 3;

const INTERNAL_FILE_REF_CHECKSUM: [u8; 32] = [
    2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// Reference to a file.
pub enum FileRef {
    /// Reference to an internal game file.
    Internal(InternalFileRef),
    /// Reference to an external file.
    External(ExternalFileRef),
}

/// Reference to an internal game file.
pub struct InternalFileRef {
    pub(crate) path: PathBuf,
}

impl InternalFileRef {
    /// Path of the referenced game file.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Reference to an external file.
pub struct ExternalFileRef {
    checksum: [u8; 32],
    path: PathBuf,
    url: String,
}

impl ExternalFileRef {
    /// Checksum of the referenced file.
    pub fn checksum(&self) -> &[u8; 32] {
        &self.checksum
    }

    /// Path of the referenced file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Locator URL of the referenced file.
    pub fn url(&self) -> &str {
        &self.url
    }
}

mod read {
    use std::{io::Read, path::PathBuf};

    use crate::{
        deserialize::Deserializer, read::Result, ExternalFileRef, FileRef, InternalFileRef,
    };

    use super::{FILE_REF_VERSION, INTERNAL_FILE_REF_CHECKSUM};

    impl FileRef {
        pub(crate) fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Option<Self>> {
            if d.u8()? != FILE_REF_VERSION {
                return Err("unknown file reference version".into());
            }

            let checksum = d.byte_array::<32>()?;
            let path = PathBuf::from(d.string()?);
            let url = d.string()?;

            if path.as_os_str().is_empty() {
                return Ok(None);
            }

            if checksum == INTERNAL_FILE_REF_CHECKSUM || url.is_empty() {
                Ok(Some(Self::Internal(InternalFileRef { path })))
            } else {
                Ok(Some(Self::External(ExternalFileRef {
                    checksum,
                    path,
                    url,
                })))
            }
        }
    }

    impl InternalFileRef {
        pub(crate) fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Option<Self>> {
            match FileRef::read(d)? {
                None => Ok(None),
                Some(FileRef::Internal(file_ref)) => Ok(Some(file_ref)),
                Some(FileRef::External(_)) => Err("expected internal file reference".into()),
            }
        }
    }

    impl ExternalFileRef {
        pub(crate) fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Option<Self>> {
            match FileRef::read(d)? {
                None => Ok(None),
                Some(FileRef::Internal(_)) => Err("expected external file reference".into()),
                Some(FileRef::External(file_ref)) => Ok(Some(file_ref)),
            }
        }
    }
}

mod write {
    use std::io::Write;

    use crate::{serialize::Serializer, write::Result, ExternalFileRef, FileRef, InternalFileRef};

    use super::{FILE_REF_VERSION, INTERNAL_FILE_REF_CHECKSUM};

    impl FileRef {
        pub(crate) fn write<W: Write, I, N>(&self, w: &mut Serializer<W, I, N>) -> Result {
            match *self {
                Self::Internal(ref file_ref) => file_ref.write(w),
                Self::External(ref file_ref) => file_ref.write(w),
            }
        }
    }

    impl InternalFileRef {
        pub(crate) fn write<W: Write, I, N>(&self, w: &mut Serializer<W, I, N>) -> Result {
            w.u8(FILE_REF_VERSION)?;
            w.byte_array(INTERNAL_FILE_REF_CHECKSUM)?;
            w.string(&self.path.to_string_lossy())?;
            w.u32(0)?;

            Ok(())
        }
    }

    impl ExternalFileRef {
        pub(crate) fn write<W: Write, I, N>(&self, w: &mut Serializer<W, I, N>) -> Result {
            w.u8(FILE_REF_VERSION)?;
            w.byte_array(self.checksum)?;
            w.string(&self.path.to_string_lossy())?;
            w.string(&self.url)?;

            Ok(())
        }
    }
}
