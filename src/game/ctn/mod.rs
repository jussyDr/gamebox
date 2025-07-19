mod anchored_object;
mod block;
mod block_skin;
mod challenge;
mod challenge_parameters;
mod collector_list;
mod ghost;
mod zone_genealogy;

pub use anchored_object::AnchoredObject;
pub use block::Block;
pub use block_skin::BlockSkin;
pub use challenge::Challenge;
pub use challenge_parameters::ChallengeParameters;
pub use collector_list::CollectorList;
pub use ghost::Ghost;
pub use zone_genealogy::ZoneGenealogy;

use crate::read::{BodyReader, Error};

pub enum FileRef<'a> {
    Internal {
        path: &'a str,
    },
    External {
        checksum: [u8; 32],
        path: &'a str,
        locator_url: &'a str,
    },
}

impl<'a> FileRef<'a> {
    pub fn read(r: &mut BodyReader<'a, '_>) -> Result<Option<Self>, Error> {
        let version = r.u8()?;

        if version != 3 {
            return Err(Error::new(format!(
                "unknown file reference version: {version}"
            )));
        }

        let checksum: [u8; 32] = r.u8_array()?;
        let path = r.string()?;
        let locator_url = r.string()?;

        if checksum.iter().all(|&byte| byte == 0) && path.is_empty() && locator_url.is_empty() {
            return Ok(None);
        }

        if checksum[0] == 2 && checksum[1..].iter().all(|&byte| byte == 0) && locator_url.is_empty()
        {
            return Ok(Some(FileRef::Internal { path }));
        }

        Ok(Some(FileRef::External {
            checksum,
            path,
            locator_url,
        }))
    }
}
