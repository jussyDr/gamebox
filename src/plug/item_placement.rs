//! Item placement.

use crate::Class;

/// Item placement.
#[derive(Default)]
pub struct ItemPlacement;

impl Class for ItemPlacement {
    const CLASS_ID: u32 = 0x09187000;
}

mod read {
    use std::io::Read;

    use crate::read::{
        reader::{IdStateMut, Reader},
        Error, ReadBody,
    };

    use super::ItemPlacement;

    impl ReadBody for ItemPlacement {
        fn read_body<R: Read, I: IdStateMut, N>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 10 {
                return Err(Error::chunk_version(version));
            }

            let _size_group = r.id_or_null()?;
            let _compatible_group_ids = r.list(|r| r.id())?;
            let _always_up = r.bool()?;
            let _align_to_interior = r.bool()?;
            let _align_to_world_dir = r.bool()?;
            let _world_dir = r.vec3::<f32>()?;
            let _patch_layouts: Vec<()> = r.list(|_| todo!())?;
            let _group_cur_patch_layouts = r.list(|r| r.u32())?;

            Ok(())
        }
    }
}
