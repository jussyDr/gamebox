use std::io::Write;

use crate::{
    classes::{
        collector::Collector, item::ItemPlacementParam, material_user_inst::MaterialUserInst,
    },
    common::END_OF_NODE_MARKER,
    serialize::{IdStateMut, NodeStateMut, Serializer},
    write::{
        writable::{HeaderChunk, HeaderChunks, Sealed, WriteBody},
        Result, Writable,
    },
    Item,
};

use super::{Crystal, ItemEntityModelEdition, ItemPlacement};

impl Writable for Item {}

impl Sealed for Item {}

impl HeaderChunks for Item {
    #[allow(clippy::redundant_closure)]
    fn header_chunks() -> impl Iterator<Item = HeaderChunk<Self>> {
        [
            HeaderChunk {
                chunk_id: 0x2e001003,
                is_heavy: false,
                write_fn: |n: &Self, s| Collector::write_chunk_3(&n.parent, s),
            },
            HeaderChunk {
                chunk_id: 0x2e001006,
                is_heavy: false,
                write_fn: |n: &Self, s| Collector::write_chunk_6(&n.parent, s),
            },
            HeaderChunk {
                chunk_id: 0x2e002000,
                is_heavy: false,
                write_fn: |n, s| Self::write_chunk_0(n, s),
            },
            HeaderChunk {
                chunk_id: 0x2e002001,
                is_heavy: false,
                write_fn: |n, s| Self::write_chunk_1(n, s),
            },
        ]
        .into_iter()
    }
}

impl<W: Write, I: IdStateMut, N: NodeStateMut> WriteBody<W, I, N> for Item {
    fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result {
        Collector::write_chunk_9(&self.parent, s)?;
        Collector::write_chunk_11(&self.parent, s)?;
        Collector::write_chunk_12(&self.parent, s)?;
        Collector::write_chunk_13(&self.parent, s)?;
        Collector::write_chunk_16(&self.parent, s)?;
        Collector::write_chunk_17(&self.parent, s)?;
        Collector::write_chunk_18(&self.parent, s)?;
        Self::write_chunk_8(self, s)?;
        Self::write_chunk_9(self, s)?;
        Self::write_chunk_12(self, s)?;
        Self::write_chunk_18(self, s)?;
        Self::write_chunk_21(self, s)?;
        Self::write_chunk_25(self, s)?;
        Self::write_chunk_26(self, s)?;
        Self::write_chunk_28(self, s)?;
        Self::write_chunk_30(self, s)?;
        Self::write_chunk_31(self, s)?;
        Self::write_chunk_32(self, s)?;
        Self::write_chunk_37(self, s)?;
        Self::write_chunk_38(self, s)?;
        Self::write_chunk_39(self, s)?;

        s.u32(END_OF_NODE_MARKER)?;

        Ok(())
    }
}

impl Item {
    fn write_chunk_0<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(1)?;

        Ok(())
    }

    fn write_chunk_1<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_8<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e002008)?;
        s.u32(7)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    fn write_chunk_9<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e002009)?;
        s.u32(10)?;
        s.u32(0)?;

        Ok(())
    }

    fn write_chunk_12<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e00200c)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    fn write_chunk_18<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e002012)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.f32(-1.0)?;
        s.f32(0.15)?;

        Ok(())
    }

    fn write_chunk_21<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e002015)?;
        s.u32(1)?;

        Ok(())
    }

    fn write_chunk_25<W: Write, I: IdStateMut, N: NodeStateMut>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result {
        s.u32(0x2e002019)?;
        s.u32(15)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0)?;
        s.u32(0)?;
        s.unique_node_ref(&ItemEntityModelEdition)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    fn write_chunk_26<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e00201a)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    fn write_chunk_28<W: Write, I: IdStateMut, N: NodeStateMut>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result {
        s.u32(0x2e00201c)?;
        s.u32(5)?;
        s.unique_node_ref(&ItemPlacementParam)?;

        Ok(())
    }

    fn write_chunk_30<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e00201e)?;
        s.u32(7)?;
        s.u32(0)?;
        s.u32(0xffffffff)?;
        s.u32(0)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    fn write_chunk_31<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e00201f)?;
        s.u32(12)?;
        s.u32(3)?;
        s.u32(0)?;
        s.u32(0xffffffff)?;
        s.u8(0)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;

        Ok(())
    }

    fn write_chunk_32<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e002020)?;
        s.u32(3)?;
        s.u32(0)?;
        s.u8(0)?;

        Ok(())
    }

    fn write_chunk_37<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e002025)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;

            Ok(())
        })?;

        Ok(())
    }

    fn write_chunk_38<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e002026)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;

            Ok(())
        })?;

        Ok(())
    }

    fn write_chunk_39<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e002027)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl<W: Write, I: IdStateMut, N: NodeStateMut> WriteBody<W, I, N> for ItemEntityModelEdition {
    fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result {
        Self::write_chunk_0(self, s)?;
        Self::write_chunk_1(self, s)?;

        s.u32(END_OF_NODE_MARKER)?;

        Ok(())
    }
}

impl ItemEntityModelEdition {
    fn write_chunk_0<W: Write, I: IdStateMut, N: NodeStateMut>(
        &self,
        s: &mut Serializer<W, I, N>,
    ) -> Result {
        s.u32(0x2e026000)?;
        s.u32(8)?;
        s.u32(1)?;
        s.unique_node_ref(&Crystal::default())?;
        s.u32(0)?;
        s.u32(0xffffffff)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0xffffffff)?;
        s.u32(0xffffffff)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.f32(1.0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.f32(1.0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.f32(1.0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(1)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(1000)?;

        Ok(())
    }

    fn write_chunk_1<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e026001)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;

            Ok(())
        })?;

        Ok(())
    }
}

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
        s.unique_node_ref(&MaterialUserInst)?;

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

impl<W: Write, I, N: NodeStateMut> WriteBody<W, I, N> for ItemPlacementParam {
    fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result {
        Self::write_chunk_0(self, s)?;
        Self::write_chunk_1(self, s)?;
        Self::write_chunk_4(self, s)?;
        Self::write_chunk_5(self, s)?;

        s.u32(END_OF_NODE_MARKER)?;

        Ok(())
    }
}

impl ItemPlacementParam {
    fn write_chunk_0<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e020000)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(1)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u16(0)?;
            s.f32(1.0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.f32(-1.0)?;

            Ok(())
        })?;

        Ok(())
    }

    fn write_chunk_1<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e020001)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;

            Ok(())
        })?;

        Ok(())
    }

    fn write_chunk_4<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e020004)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.u32(0)?;
            s.u32(0)?;

            Ok(())
        })?;

        Ok(())
    }

    fn write_chunk_5<W: Write, I, N: NodeStateMut>(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(0x2e020005)?;
        s.u32(0x534b4950)?;
        s.buffer(|s| {
            s.unique_node_ref(&ItemPlacement)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl<W: Write, I, N> WriteBody<W, I, N> for ItemPlacement {
    fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result {
        s.u32(10)?;
        s.u32(0xffffffff)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(1)?;
        s.u32(0)?;
        s.u32(0)?;
        s.u32(0)?;
        s.f32(1.0)?;
        s.u32(0)?;
        s.u32(0)?;

        Ok(())
    }
}
