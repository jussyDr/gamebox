//! Item placement class.

use crate::ClassId;

/// Item placement class.
#[derive(Default)]
pub struct ItemPlacementClass;

impl ClassId for ItemPlacementClass {
    const CLASS_ID: u32 = 0x09187000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::item_placement_class::ItemPlacementClass,
        read::{Error, ReadBody, error_unknown_chunk_version, reader::BodyReader},
    };

    impl ReadBody for ItemPlacementClass {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 10 {
                return Err(error_unknown_chunk_version(version));
            }

            let _size_group: Option<Arc<str>> = r.id()?;
            let _compatible_group_ids: Vec<Arc<str>> = r.list(|r| r.id())?;
            let _always_up = r.bool32()?;
            let _align_to_interior = r.bool32()?;
            let _align_to_world_dir = r.bool32()?;
            let _world_dir = r.vec3()?;
            let _patch_layouts = r.list(|r| {
                let _item_count = r.u32()?;
                let _item_spacing = r.f32()?;
                let _fill_align = r.u32()?;
                let _fill_dir = r.u32()?;
                let _normed_pos = r.f32()?;
                r.f32()?;
                let _only_on_groups: Vec<Arc<str>> = r.list(|r| r.id())?;
                let _altitude = r.f32()?;
                r.f32()?;

                Ok(())
            })?;
            let _group_cur_patch_layouts = r.list(|r| r.u32())?;

            Ok(())
        }
    }
}
