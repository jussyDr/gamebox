use std::io::Write;

use crate::{
    common::END_OF_NODE_MARKER,
    engines::plug::material_user_inst::MaterialUserInst,
    serialize::{IdStateMut, NodeStateMut, Serializer},
    write::{writable::WriteBody, Result},
};

use super::Crystal;

impl<W: Write, I: IdStateMut, N: NodeStateMut> WriteBody<W, I, N> for Crystal {
    fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result {
        Self::write_chunk_3(self, s)?;
        Self::write_chunk_4(self, s)?;
        Self::write_chunk_5(self, s)?;
        Self::write_chunk_6(self, s)?;
        Self::write_chunk_7(self, s)?;

        s.u32(END_OF_NODE_MARKER)?;

        Ok(())
    }
}

impl Crystal {
    fn write_chunk_3<W: Write, I: IdStateMut, N: NodeStateMut>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result {
        s.u32(0x09003003)?;
        s.u32(2)?;
        s.u32(1)?;
        s.u32(0)?;
        s.unique_node_ref(&MaterialUserInst::default())?;

        Ok(())
    }

    fn write_chunk_4<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x09003004)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(1)?;
            s.u32(0)?;
            s.u32(1)?;

            Ok(())
        })?;

        Ok(())
    }

    fn write_chunk_5<W: Write, I: IdStateMut, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x09003005)?;
        s.u32(0)?;
        s.u32(1)?;
        s.u32(0)?;
        s.u32(2)?;
        s.u32(0)?;
        s.id("Layer0")?;
        s.string("Geometry")?;
        s.u32(1)?;
        s.u32(1)?;
        s.u32(37)?;
        s.u32(4)?;
        s.u32(3)?;
        s.u32(4)?;
        s.f32(64.0)?;
        s.u32(2)?;
        s.f32(128.0)?;
        s.u32(1)?;
        s.f32(192.0)?;
        s.u32(0)?;
        s.u32(1)?;
        s.u32(0)?;
        s.u8(1)?;
        s.u32(0xffffffff)?;
        s.string("DefaultCube")?;
        s.u32(0xffffffff)?;
        s.u32(0)?;
        s.u8(1)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(8)?;
        s.f32(-2.0)?;
        s.f32(0.0)?;
        s.f32(-2.0)?;
        s.f32(-2.0)?;
        s.f32(0.0)?;
        s.f32(2.0)?;
        s.f32(2.0)?;
        s.f32(0.0)?;
        s.f32(2.0)?;
        s.f32(2.0)?;
        s.f32(0.0)?;
        s.f32(-2.0)?;
        s.f32(-2.0)?;
        s.f32(4.0)?;
        s.f32(-2.0)?;
        s.f32(-2.0)?;
        s.f32(4.0)?;
        s.f32(2.0)?;
        s.f32(2.0)?;
        s.f32(4.0)?;
        s.f32(2.0)?;
        s.f32(2.0)?;
        s.f32(4.0)?;
        s.f32(-2.0)?;
        s.u32(12)?;
        s.u32(0)?;
        s.u32(6)?;
        s.u32(4)?;
        s.f32(4.0)?;
        s.f32(4.0)?;
        s.f32(0.0)?;
        s.f32(4.0)?;
        s.f32(0.0)?;
        s.f32(0.0)?;
        s.f32(4.0)?;
        s.f32(0.0)?;
        s.u32(24)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(2)?;
        s.u8(3)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(2)?;
        s.u8(3)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(2)?;
        s.u8(3)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(2)?;
        s.u8(3)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(2)?;
        s.u8(3)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(2)?;
        s.u8(3)?;
        s.u8(1)?;
        s.u8(0)?;
        s.u8(3)?;
        s.u8(2)?;
        s.u8(1)?;
        s.u8(0)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(5)?;
        s.u8(6)?;
        s.u8(7)?;
        s.u8(4)?;
        s.u8(0)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(4)?;
        s.u8(7)?;
        s.u8(3)?;
        s.u8(0)?;
        s.u8(0)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(5)?;
        s.u8(4)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(0)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(6)?;
        s.u8(5)?;
        s.u8(1)?;
        s.u8(2)?;
        s.u8(0)?;
        s.u8(0)?;
        s.u8(1)?;
        s.u8(7)?;
        s.u8(6)?;
        s.u8(2)?;
        s.u8(3)?;
        s.u8(0)?;
        s.u8(0)?;
        s.u32(0)?;
        s.u32(1)?;
        s.u32(0)?;
        s.u32(1)?;
        s.u32(1)?;

        Ok(())
    }

    fn write_chunk_6<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x09003006)?;
        s.u32(2)?;
        s.u32(24)?;
        s.u32(0x550c0000)?;
        s.u32(0x550c550c)?;
        s.u32(0x0000550c)?;
        s.u32(0x00000000)?;
        s.u32(0xaa855579)?;
        s.u32(0xaa85aa85)?;
        s.u32(0x5579aa85)?;
        s.u32(0x55795579)?;
        s.u32(0xffff0000)?;
        s.u32(0xffff550c)?;
        s.u32(0xaaf2550c)?;
        s.u32(0xaaf20000)?;
        s.u32(0xaa850000)?;
        s.u32(0xaa85550c)?;
        s.u32(0x5579550c)?;
        s.u32(0x55790000)?;
        s.u32(0x550caaf2)?;
        s.u32(0x550cffff)?;
        s.u32(0x0000ffff)?;
        s.u32(0x0000aaf2)?;
        s.u32(0x550c5579)?;
        s.u32(0x550caa85)?;
        s.u32(0x0000aa85)?;
        s.u32(0x00005579)?;
        s.u32(24)?;
        for i in 0..24 {
            s.u8(i)?;
        }

        Ok(())
    }

    fn write_chunk_7<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x09003007)?;
        s.u32(0)?;
        s.u32(3)?;
        s.f32(1.0)?;
        s.f32(1.0)?;
        s.f32(2.0)?;
        s.u32(6)?;
        for _ in 0..6 {
            s.u32(2)?;
        }

        Ok(())
    }
}
