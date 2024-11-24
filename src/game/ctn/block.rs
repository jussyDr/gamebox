/// A block.
#[derive(Default)]
pub struct Block;

mod read {
    use std::io::Read;

    use crate::{
        game::{
            ctn::{block_skin::BlockSkin, Direction},
            waypoint_special_property::WaypointSpecialProperty,
        },
        read::{
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ReadBody,
        },
    };

    use super::Block;

    impl ReadBody for Block {
        fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            let _name = r.id()?;
            let _direction = r.enum_u8::<Direction>()?;
            let _coord = r.u8()?;
            let _coord = r.u8()?;
            let _coord = r.u8()?;
            let flags = r.u32()?;

            if flags != 0xffffffff {
                if flags & 0x00008000 != 0 {
                    let _author = r.id()?;
                    let _skin = r.internal_node_ref::<BlockSkin>()?;
                }

                if flags & 0x00080000 != 0 || flags & 0x00100000 != 0 {
                    let _waypoint_special_property =
                        r.internal_node_ref::<WaypointSpecialProperty>()?;
                }
            }

            Ok(())
        }
    }
}
