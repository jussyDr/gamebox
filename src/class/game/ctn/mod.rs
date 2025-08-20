mod block;
pub use block::Block;

mod block_skin;
pub use block_skin::BlockSkin;

mod challenge;
pub use challenge::Challenge;

mod challenge_parameters;
pub use challenge_parameters::ChallengeParameters;

mod collector_list;
pub use collector_list::CollectorList;

mod ghost;
pub use ghost::Ghost;

use crate::read::{Error, Reader, Result};

enum FileRef {
    Internal {
        path: String,
    },
    External {
        path: String,
        url: String,
        checksum: [u8; 32],
    },
}

impl FileRef {
    fn read(r: &mut impl Reader) -> Result<Option<Self>> {
        let version = r.u8()?;

        if version != 3 {
            return Err(Error::Internal("unknown file reference version".into()));
        }

        let checksum = r.array_u8::<32>()?;
        let path = r.string()?;
        let url = r.string()?;

        if path.is_empty() {
            return Ok(None);
        }

        if url.is_empty() {
            return Ok(Some(FileRef::Internal { path }));
        }

        Ok(Some(FileRef::External {
            path,
            url,
            checksum,
        }))
    }
}
