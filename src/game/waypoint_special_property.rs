//! Waypoint special property.

use crate::Class;

/// A waypoint special property.
#[derive(PartialEq, Default, Debug)]
pub struct WaypointSpecialProperty {
    tag: String,
    order: u32,
}

impl Class for WaypointSpecialProperty {
    const CLASS_ID: u32 = 0x2e009000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::WaypointSpecialProperty;

    impl ReadBody for WaypointSpecialProperty {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for WaypointSpecialProperty {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(0, Self::read_chunk_0),
                BodyChunk::skippable(1, Self::read_chunk_1),
            ]
            .into_iter()
        }
    }

    impl WaypointSpecialProperty {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            self.tag = r.string()?;
            self.order = r.u32()?;

            Ok(())
        }

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            if r.bool()? {
                todo!()
            }

            Ok(())
        }
    }
}
