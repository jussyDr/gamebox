//! Block.

/// A block.
#[derive(Default)]
pub struct Block;

mod read {
    use crate::{
        class::game::{
            ctn::{block::Block, block_skin::BlockSkin},
            waypoint_special_property::WaypointSpecialProperty,
        },
        read::{Error, ReadBody, reader::BodyReader},
    };

    impl ReadBody for Block {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _name = r.id()?;
            let _direction = r.u8()?;
            let _coord = r.repeat(3, |r| r.u8())?;
            let flags = r.u32()?;

            if flags & 0x00008000 != 0 {
                let _author = r.id()?;
                let _skin = r.internal_node_ref::<BlockSkin>()?;
            }

            if flags & 0x00080000 != 0 {
                todo!();
            }

            if flags & 0x00100000 != 0 {
                let _waypoint_special_property =
                    r.internal_node_ref::<WaypointSpecialProperty>()?;
            }

            if flags & 0x00040000 != 0 {
                todo!();
            }

            if flags & 0x00020000 != 0 {
                let _decal_id = r.id()?;
                let _decal_intensity = r.u32()?;
                let _decal_variant = r.u32()?;
            }

            Ok(())
        }
    }
}
