use std::io::Write;

use crate::{
    serialize::{IdState, NodeState, Serializer},
    write::{writable::Sealed, Result, Writable},
    MAGIC,
};

use super::Item;

impl Sealed for Item {}

impl Writable for Item {}

pub fn test(writer: impl Write) -> Result<()> {
    let mut id_state = IdState::new();

    let mut chunk_2e001003 = vec![];
    let mut s = Serializer::new(&mut chunk_2e001003, &mut id_state, ());
    s.null_id()?;
    s.u32(26)?;
    s.id("r-brwiQCRnOZ2PIHcM0Q8A")?;
    s.u32(8)?;
    s.string("Items")?;
    s.u32(0xffffffff)?;
    s.u32(8)?;
    s.u16(1)?;
    s.string("New Item")?;
    s.u8(3)?;

    let mut chunk_2e001006 = vec![];
    let mut s = Serializer::new(&mut chunk_2e001006, &mut id_state, ());
    s.u32(0)?;
    s.u32(0)?;

    let mut chunk_2e002000 = vec![];
    let mut s = Serializer::new(&mut chunk_2e002000, &mut id_state, ());
    s.u32(1)?;

    let mut chunk_2e002001 = vec![];
    let mut s = Serializer::new(&mut chunk_2e002001, &mut id_state, ());
    s.u32(0)?;

    let mut user_data = vec![];
    let mut s = Serializer::new(&mut user_data, (), ());
    s.u32(4)?;
    s.u32(0x2e001003)?;
    s.u32(chunk_2e001003.len() as u32)?;
    s.u32(0x2e001006)?;
    s.u32(chunk_2e001006.len() as u32)?;
    s.u32(0x2e002000)?;
    s.u32(chunk_2e002000.len() as u32)?;
    s.u32(0x2e002001)?;
    s.u32(chunk_2e002001.len() as u32)?;
    s.bytes(&chunk_2e001003)?;
    s.bytes(&chunk_2e001006)?;
    s.bytes(&chunk_2e002000)?;
    s.bytes(&chunk_2e002001)?;

    let mut node_state = NodeState::new();
    let mut body = vec![];
    let mut s = Serializer::new(&mut body, IdState::new(), &mut node_state);

    s.u32(0x2e001009)?;
    s.string("Items")?;
    s.u32(0)?;
    s.null_id()?;

    s.u32(0x2e00100b)?;
    s.u32(0xffffffff)?;
    s.u32(26)?;
    s.id("r-brwiQCRnOZ2PIHcM0Q8A")?;

    s.u32(0x2e00100c)?;
    s.string("New Item")?;

    s.u32(0x2e00100d)?;
    s.string("No Description")?;

    s.u32(0x2e001010)?;
    s.u32(4)?;
    s.u32(0xffffffff)?;
    s.u32(0)?;
    s.u32(0xffffffff)?;

    s.u32(0x2e001011)?;
    s.u32(1)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(1)?;
    s.u8(3)?;

    s.u32(0x2e001012)?;
    s.u32(0)?;
    s.u32(1)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0x2e002008)?;
    s.u32(7)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;

    s.u32(0x2e002008)?;
    s.u32(10)?;
    s.u32(0)?;

    s.u32(0x2e00200c)?;
    s.u32(0xffffffff)?;

    s.u32(0x2e002012)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0)?;
    s.f32(-1.0)?;
    s.f32(0.15)?;

    s.u32(0x2e002015)?;
    s.u32(1)?;

    s.u32(0x2e002019)?;
    s.u32(15)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;
    s.u32(0)?;
    s.u32(0)?;
    s.node_index()?;
    s.u32(0x2e026000)?;

    s.u32(0x2e026000)?;
    s.u32(7)?;
    s.u32(1)?;
    s.node_index()?;
    s.u32(0x09003000)?;

    s.u32(0x09003003)?;
    s.u32(2)?;
    s.u32(2)?;

    s.u32(0)?;
    s.node_index()?;
    s.u32(0x090fd000)?;

    s.u32(0x090fd000)?;
    s.u32(11)?;
    s.u8(1)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;
    s.u32(0)?;
    s.u16(22)?;
    s.string("Stadium\\Media\\Material\\TechnicsTrims")?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0xffffffff)?;

    s.u32(0x090fd001)?;
    s.u32(5)?;
    s.u32(0xffffffff)?;
    s.u32(0)?;
    s.u32(0)?;
    s.f32(1.0)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0x090fd002)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0xfacade01)?;

    s.u32(0)?;
    s.node_index()?;
    s.u32(0x090fd000)?;

    s.u32(0x090fd000)?;

    s.u32(0x090fd000)?;
    s.u32(11)?;
    s.u8(1)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;
    s.u32(0)?;
    s.u16(4)?;
    s.string("Stadium\\Media\\Material\\TrackWallClips")?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0)?;
    s.u32(0xffffffff)?;

    s.u32(0x090fd001)?;
    s.u32(5)?;
    s.u32(0xffffffff)?;
    s.u32(0)?;
    s.u32(0)?;
    s.f32(1.0)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0x090fd002)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0xfacade01)?;

    s.u32(0x09003004)?;
    s.u32(0x534b495)?;
    s.u32(12)?;
    s.u32(1)?;
    s.u32(0)?;
    s.u32(1)?;

    s.u32(0x09003005)?;
    // TODO
    s.u32(0x09003006)?;
    // TODO
    s.u32(0x09003007)?;
    // TODO
    s.u32(0xfacede01)?;
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
    s.u32(0x3e8)?;
    s.u32(0xffffffff)?;

    s.u32(0x2e026001)?;
    s.u32(0x534b4950)?;
    s.u32(8)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0xfacade01)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;

    s.u32(0x2e00201a)?;
    s.u32(0xffffffff)?;

    s.u32(0x2e00201c)?;
    s.u32(5)?;
    s.node_index()?;
    s.u32(0x2e020000)?;

    s.u32(0x2e020000)?;
    s.u32(0x534b4950)?;
    s.u32(50)?;
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
    s.f32(1.0)?;
    s.u32(0)?;
    s.f32(-1.0)?;

    s.u32(0x2e020001)?;
    s.u32(0x534b4950)?;
    s.u32(8)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0x2e020004)?;
    s.u32(0x534b4950)?;
    s.u32(8)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0x2e020005)?;
    s.u32(52)?;
    s.node_index()?;
    s.u32(0x09187000)?;

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

    s.u32(0xfacade01)?;

    s.u32(0x2e00201e)?;
    s.u32(7)?;
    s.u32(0)?;
    s.u32(0xffffffff)?;
    s.u32(0)?;
    s.u32(0xffffffff)?;

    s.u32(0x2e00201f)?;
    s.u32(12)?;
    s.u32(3)?;
    s.u32(0)?;
    s.u32(0xffffffff)?;
    s.u8(0)?;
    s.u32(0xffffffff)?;
    s.u32(0xffffffff)?;

    s.u32(0x2e002020)?;
    s.u32(3)?;
    s.u32(0)?;
    s.u8(0)?;

    s.u32(0x2e002025)?;
    s.u32(0x534b4950)?;
    s.u32(8)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0x2e002026)?;
    s.u32(0x534b4950)?;
    s.u32(8)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0x2e002027)?;
    s.u32(0x534b4950)?;
    s.u32(8)?;
    s.u32(0)?;
    s.u32(0)?;

    s.u32(0xfacade01)?;

    let mut buf = vec![0; lzo1x_1::worst_compress(body.len())];

    let compressed_body = lzo1x_1::compress_to_slice(&body, &mut buf);

    let mut s = Serializer::new(writer, (), ());

    s.byte_array(MAGIC)?;
    s.u16(6)?;
    s.u8(b'B')?;
    s.u8(b'U')?;
    s.u8(b'C')?;
    s.u8(b'R')?;
    s.u32(0x2e002000)?;
    s.u32(user_data.len() as u32)?;
    s.bytes(&user_data)?;
    s.u32(0)?;

    s.u32(0)?;

    s.u32(body.len() as u32)?;
    s.u32(compressed_body.len() as u32)?;
    s.bytes(compressed_body)?;

    Ok(())
}
