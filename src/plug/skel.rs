//! Skel.

use crate::Class;

/// Skeleton.
#[derive(Default)]
pub struct Skel {
    joints: Vec<Joint>,
    sockets: Vec<Socket>,
}

impl Class for Skel {
    const CLASS_ID: u32 = 0x090ba000;
}

impl Skel {
    /// Sockets.
    pub const fn sockets(&self) -> &Vec<Socket> {
        &self.sockets
    }
}

/// Joint.
pub struct Joint;

/// Socket.
pub struct Socket;

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Joint, Skel, Socket};

    impl ReadBody for Skel {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Skel {
        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl Skel {
        fn read_chunk_0<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 19 {
                return Err(Error::chunk_version(version));
            }

            let _name = r.id_or_null()?;
            let joints_length = r.u16()?;
            self.joints = r.repeat(joints_length as usize, |r| {
                let _name = r.id()?;
                let _parent_index = r.u16()?;
                r.iso4()?;

                Ok(Joint)
            })?;
            r.bool()?;
            self.sockets = r.list(|r| {
                let _name = r.id()?;
                r.u16()?;
                r.iso4()?;

                Ok(Socket)
            })?;
            r.bool()?;
            r.byte_buf()?;
            r.byte_buf()?;
            r.byte_buf()?;
            r.u8()?;
            r.u32()?;

            Ok(())
        }
    }
}
