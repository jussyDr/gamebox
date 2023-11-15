use std::io::Write;

use crate::writer::{IdState, NodeState, Result, Writer};

pub fn test(writer: impl Write) -> Result<()> {
    let mut id_state = IdState::new();

    let mut chunk_2e001003 = vec![];
    let mut w = Writer::new(&mut chunk_2e001003, &mut id_state, ());
    w.null_id()?;
    w.u32(26)?;
    w.id("r-brwiQCRnOZ2PIHcM0Q8A")?;
    w.u32(8)?;
    w.string("Items")?;
    w.u32(0xffffffff)?;
    w.u32(8)?;
    w.u16(1)?;
    w.string("New Item")?;
    w.u8(3)?;

    let mut chunk_2e001006 = vec![];
    let mut w = Writer::new(&mut chunk_2e001006, &mut id_state, ());
    w.u32(0)?;
    w.u32(0)?;

    let mut chunk_2e002000 = vec![];
    let mut w = Writer::new(&mut chunk_2e002000, &mut id_state, ());
    w.u32(1)?;

    let mut chunk_2e002001 = vec![];
    let mut w = Writer::new(&mut chunk_2e002001, &mut id_state, ());
    w.u32(0)?;

    let mut user_data = vec![];
    let mut w = Writer::new(&mut user_data, (), ());
    w.u32(4)?;
    w.u32(0x2e001003)?;
    w.u32(chunk_2e001003.len() as u32)?;
    w.u32(0x2e001006)?;
    w.u32(chunk_2e001006.len() as u32)?;
    w.u32(0x2e002000)?;
    w.u32(chunk_2e002000.len() as u32)?;
    w.u32(0x2e002001)?;
    w.u32(chunk_2e002001.len() as u32)?;
    w.bytes(&chunk_2e001003)?;
    w.bytes(&chunk_2e001006)?;
    w.bytes(&chunk_2e002000)?;
    w.bytes(&chunk_2e002001)?;

    let mut node_state = NodeState::new();
    let mut body = vec![];
    let mut w = Writer::new(&mut body, IdState::new(), &mut node_state);

    w.u32(0x2e001009)?;
    w.string("Items")?;
    w.u32(0)?;
    w.null_id()?;

    w.u32(0x2e00100b)?;
    w.u32(0xffffffff)?;
    w.u32(26)?;
    w.id("r-brwiQCRnOZ2PIHcM0Q8A")?;

    w.u32(0x2e00100c)?;
    w.string("New Item")?;

    w.u32(0x2e00100d)?;
    w.string("No Description")?;

    w.u32(0x2e001010)?;
    w.u32(4)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u32(0xffffffff)?;

    w.u32(0x2e001011)?;
    w.u32(1)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(1)?;
    w.u8(3)?;

    w.u32(0x2e001012)?;
    w.u32(0)?;
    w.u32(1)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0x2e002008)?;
    w.u32(7)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;

    w.u32(0x2e002008)?;
    w.u32(10)?;
    w.u32(0)?;

    w.u32(0x2e00200c)?;
    w.u32(0xffffffff)?;

    w.u32(0x2e002012)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.f32(-1.0)?;
    w.f32(0.15)?;

    w.u32(0x2e002015)?;
    w.u32(1)?;

    w.u32(0x2e002019)?;
    w.u32(15)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u32(0)?;
    w.node_index()?;
    w.u32(0x2e026000)?;

    w.u32(0x2e026000)?;
    w.u32(7)?;
    w.u32(1)?;
    w.node_index()?;
    w.u32(0x09003000)?;

    w.u32(0x09003003)?;
    w.u32(2)?;
    w.u32(2)?;

    w.u32(0)?;
    w.node_index()?;
    w.u32(0x090fd000)?;

    w.u32(0x090fd000)?;
    w.u32(11)?;
    w.u8(1)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u16(22)?;
    w.string("Stadium\\Media\\Material\\TechnicsTrims")?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0xffffffff)?;

    w.u32(0x090fd001)?;
    w.u32(5)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u32(0)?;
    w.f32(1.0)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0x090fd002)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0xfacade01)?;

    w.u32(0)?;
    w.node_index()?;
    w.u32(0x090fd000)?;

    w.u32(0x090fd000)?;

    w.u32(0x090fd000)?;
    w.u32(11)?;
    w.u8(1)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u16(4)?;
    w.string("Stadium\\Media\\Material\\TrackWallClips")?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0xffffffff)?;

    w.u32(0x090fd001)?;
    w.u32(5)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u32(0)?;
    w.f32(1.0)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0x090fd002)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0xfacade01)?;

    w.u32(0x09003004)?;
    w.u32(0x534b495)?;
    w.u32(12)?;
    w.u32(1)?;
    w.u32(0)?;
    w.u32(1)?;

    w.u32(0x09003005)?;
    // TODO
    w.u32(0x09003006)?;
    // TODO
    w.u32(0x09003007)?;
    // TODO
    w.u32(0xfacede01)?;
    w.u32(0)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.f32(1.0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.f32(1.0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.f32(1.0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(1)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0x3e8)?;
    w.u32(0xffffffff)?;

    w.u32(0x2e026001)?;
    w.u32(0x534b4950)?;
    w.u32(8)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0xfacade01)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;

    w.u32(0x2e00201a)?;
    w.u32(0xffffffff)?;

    w.u32(0x2e00201c)?;
    w.u32(5)?;
    w.node_index()?;
    w.u32(0x2e020000)?;

    w.u32(0x2e020000)?;
    w.u32(0x534b4950)?;
    w.u32(50)?;
    w.u32(0)?;
    w.u32(1)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u16(0)?;
    w.f32(1.0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.f32(1.0)?;
    w.u32(0)?;
    w.f32(-1.0)?;

    w.u32(0x2e020001)?;
    w.u32(0x534b4950)?;
    w.u32(8)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0x2e020004)?;
    w.u32(0x534b4950)?;
    w.u32(8)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0x2e020005)?;
    w.u32(52)?;
    w.node_index()?;
    w.u32(0x09187000)?;

    w.u32(10)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(1)?;
    w.u32(0)?;
    w.u32(0)?;
    w.u32(0)?;
    w.f32(1.0)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0xfacade01)?;

    w.u32(0x2e00201e)?;
    w.u32(7)?;
    w.u32(0)?;
    w.u32(0xffffffff)?;
    w.u32(0)?;
    w.u32(0xffffffff)?;

    w.u32(0x2e00201f)?;
    w.u32(12)?;
    w.u32(3)?;
    w.u32(0)?;
    w.u32(0xffffffff)?;
    w.u8(0)?;
    w.u32(0xffffffff)?;
    w.u32(0xffffffff)?;

    w.u32(0x2e002020)?;
    w.u32(3)?;
    w.u32(0)?;
    w.u8(0)?;

    w.u32(0x2e002025)?;
    w.u32(0x534b4950)?;
    w.u32(8)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0x2e002026)?;
    w.u32(0x534b4950)?;
    w.u32(8)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0x2e002027)?;
    w.u32(0x534b4950)?;
    w.u32(8)?;
    w.u32(0)?;
    w.u32(0)?;

    w.u32(0xfacade01)?;

    let mut buf = vec![0; lzo1x_1::worst_compress(body.len())];

    let compressed_body = lzo1x_1::compress_to_slice(&body, &mut buf);

    let mut w = Writer::new(writer, (), ());

    w.byte_array([b'G', b'B', b'X'])?;
    w.u16(6)?;
    w.u8(b'B')?;
    w.u8(b'U')?;
    w.u8(b'C')?;
    w.u8(b'R')?;
    w.u32(0x2e002000)?;
    w.u32(user_data.len() as u32)?;
    w.bytes(&user_data)?;
    w.u32(node_state.num_nodes())?;

    w.u32(0)?;

    w.u32(body.len() as u32)?;
    w.u32(compressed_body.len() as u32)?;
    w.bytes(compressed_body)?;

    Ok(())
}
