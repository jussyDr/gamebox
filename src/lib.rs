mod reader;

use std::io::{Cursor, Read, Seek};

use reader::{IdState, NodeState, Result};

use crate::reader::Reader;

pub fn test(reader: impl Read + Seek) -> Result<()> {
    let mut reader = Reader::new(reader, (), ());

    if reader.bytes_array()? != [b'G', b'B', b'X'] {
        todo!()
    }

    if reader.u16()? != 6 {
        todo!()
    }

    if reader.u8()? != b'B' {
        todo!()
    }

    if reader.u8()? != b'U' {
        todo!()
    }

    if reader.u8()? != b'C' {
        todo!()
    }

    if reader.u8()? != b'R' {
        todo!()
    }

    let class_id = reader.u32()?;
    let user_data_size = reader.u32()?;
    let user_data = reader.bytes(user_data_size as usize)?;
    let num_nodes = reader.u32()?;

    let num_node_refs = reader.u32()?;

    let body_size = reader.u32()?;
    let compressed_body_size = reader.u32()?;
    let compressed_body = reader.bytes(compressed_body_size as usize)?;
    let mut buf = vec![0; body_size as usize];

    let body = lzo1x_1::decompress_to_slice(&compressed_body, &mut buf).unwrap();

    if reader.u8().is_ok() {
        todo!()
    }

    let reader = Cursor::new(body);
    let mut reader = Reader::new(reader, IdState::new(), NodeState::new(num_nodes));

    loop {
        let chunk_id = reader.u32()?;

        match chunk_id {
            0x2e001009 => read_chunk_2e001009(&mut reader)?,
            0x2e00100b => read_chunk_2e00100b(&mut reader)?,
            0x2e00100c => read_chunk_2e00100c(&mut reader)?,
            0x2e00100d => read_chunk_2e00100d(&mut reader)?,
            0x2e001010 => read_chunk_2e001010(&mut reader)?,
            0x2e001011 => read_chunk_2e001011(&mut reader)?,
            0x2e001012 => read_chunk_2e001012(&mut reader)?,
            0x2e002008 => read_chunk_2e002008(&mut reader)?,
            0x2e002009 => read_chunk_2e002009(&mut reader)?,
            0x2e00200c => read_chunk_2e00200c(&mut reader)?,
            0x2e002012 => read_chunk_2e002012(&mut reader)?,
            0x2e002015 => read_chunk_2e002015(&mut reader)?,
            0x2e002019 => read_chunk_2e002019(&mut reader)?,
            0x2e00201a => read_chunk_2e00201a(&mut reader)?,
            0x2e00201c => read_chunk_2e00201c(&mut reader)?,
            0x2e00201e => read_chunk_2e00201e(&mut reader)?,
            0x2e00201f => read_chunk_2e00201f(&mut reader)?,
            0x2e002020 => read_chunk_2e002020(&mut reader)?,
            0x2e002025 => read_chunk_2e002025(&mut reader)?,
            0x2e002026 => read_chunk_2e002026(&mut reader)?,
            0x2e002027 => read_chunk_2e002027(&mut reader)?,
            0xfacade01 => break,
            _ => todo!(),
        }
    }

    Ok(())
}

fn read_chunk_2e001009<R: Read, N>(r: &mut Reader<R, IdState, N>) -> Result<()> {
    r.string()?; // "Items"
    r.u32()?; // 0
    r.id_or_null()?; // null

    Ok(())
}

fn read_chunk_2e00100b<R: Read, N>(r: &mut Reader<R, IdState, N>) -> Result<()> {
    r.u32()?; // 0xffffffff
    r.u32()?; // 26
    r.id()?; // "r-brwiQCRnOZ2PIHcM0Q8A"

    Ok(())
}

fn read_chunk_2e00100c<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.string()?; // "New Item"

    Ok(())
}

fn read_chunk_2e00100d<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.string()?; // "No Description"

    Ok(())
}

fn read_chunk_2e001010<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 4
    r.u32()?; // 0xffffffff
    r.u32()?; // 0
    r.u32()?; // 0xffffffff

    Ok(())
}

fn read_chunk_2e001011<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 1
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 1
    r.u8()?; // 3

    Ok(())
}

fn read_chunk_2e001012<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 0
    r.u32()?; // 1
    r.u32()?; // 0
    r.u32()?; // 0

    Ok(())
}

fn read_chunk_2e002008<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 7
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff

    Ok(())
}

fn read_chunk_2e002009<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 10
    r.u32()?; // 0

    Ok(())
}

fn read_chunk_2e00200c<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 0xffffffff

    Ok(())
}

fn read_chunk_2e002012<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.f32()?; // -1.0
    r.f32()?; // 0.15

    Ok(())
}

fn read_chunk_2e002015<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 1

    Ok(())
}

fn read_chunk_2e002019<R: Read>(r: &mut Reader<R, IdState, NodeState>) -> Result<()> {
    r.u32()?; // 15
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff
    r.u32()?; // 0
    r.u32()?; // 0
    r.node(0x2e026000, |r| {
        loop {
            let chunk_id = r.u32()?;

            match chunk_id {
                0x2e026000 => read_chunk_2e026000(r)?,
                0x2e026001 => read_chunk_2e026001(r)?,
                0xfacade01 => break,
                _ => todo!(),
            }
        }

        Ok(())
    })?;
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff

    Ok(())
}

fn read_chunk_2e00201a<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 0xffffffff

    Ok(())
}

fn read_chunk_2e00201c<R: Read, I>(r: &mut Reader<R, I, NodeState>) -> Result<()> {
    r.u32()?; // 5
    r.node(0x2e020000, |r| {
        loop {
            let chunk_id = r.u32()?;

            match chunk_id {
                0x2e020000 => read_chunk_2e020000(r)?,
                0x2e020001 => read_chunk_2e020001(r)?,
                0x2e020004 => read_chunk_2e020004(r)?,
                0x2e020005 => read_chunk_2e020005(r)?,
                0xfacade01 => break,
                _ => todo!(),
            }
        }

        Ok(())
    })?;

    Ok(())
}

fn read_chunk_2e00201e<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 7
    r.u32()?; // 0
    r.u32()?; // 0xffffffff
    r.u32()?; // 0
    r.u32()?; // 0xffffffff

    Ok(())
}

fn read_chunk_2e00201f<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 12
    r.u32()?; // 3
    r.u32()?; // 0
    r.u32()?; // 0xffffffff
    r.u8()?; // 0
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff

    Ok(())
}

fn read_chunk_2e002020<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 3
    r.u32()?; // 0
    r.u8()?; // 0

    Ok(())
}

fn read_chunk_2e002025<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // "piks"
    r.u32()?; // 8
    r.u32()?; // 0
    r.u32()?; // 0

    Ok(())
}

fn read_chunk_2e002026<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // "piks"
    r.u32()?; // 8
    r.u32()?; // 0
    r.u32()?; // 0

    Ok(())
}

fn read_chunk_2e002027<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // "piks"
    r.u32()?; // 8
    r.u32()?; // 0
    r.u32()?; // 0

    Ok(())
}

fn read_chunk_2e020000<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // "piks"
    r.u32()?; // 50
    r.u32()?; // 0
    r.u32()?; // 1
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u16()?; // 0
    r.f32()?; // 1.0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.f32()?; // 1.0
    r.u32()?; // 0
    r.f32()?; // -1.0

    Ok(())
}

fn read_chunk_2e020001<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // "piks"
    r.u32()?; // 8
    r.u32()?; // 0
    r.u32()?; // 0

    Ok(())
}

fn read_chunk_2e020004<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // "piks"
    r.u32()?; // 8
    r.u32()?; // 0
    r.u32()?; // 0

    Ok(())
}

fn read_chunk_2e020005<R: Read, I>(r: &mut Reader<R, I, NodeState>) -> Result<()> {
    r.u32()?; // "piks"
    r.u32()?; // 52
    r.node(0x09187000, |r| {
        r.u32()?; // 10
        r.u32()?; // 0xffffffff
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 1
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.f32()?; // 1.0
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    })?;

    Ok(())
}

fn read_chunk_2e026000<R: Read>(r: &mut Reader<R, IdState, NodeState>) -> Result<()> {
    r.u32()?; // 7
    r.u32()?; // 1
    r.node(0x09003000, |r| {
        loop {
            let chunk_id = r.u32()?;

            match chunk_id {
                0x09003003 => read_chunk_09003003(r)?,
                0x09003004 => read_chunk_09003004(r)?,
                0x09003005 => read_chunk_09003005(r)?,
                0x09003006 => read_chunk_09003006(r)?,
                0x09003007 => read_chunk_09003007(r)?,
                0xfacade01 => break,
                _ => todo!(),
            }
        }

        Ok(())
    })?;
    r.u32()?; // 0
    r.u32()?; // 0xffffffff
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.f32()?; // 1.0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.f32()?; // 1.0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.f32()?; // 1.0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 1
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0x3e8
    r.u32()?; // 0xffffffff

    Ok(())
}

fn read_chunk_2e026001<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // "piks"
    r.u32()?; // 8
    r.u32()?; // 0
    r.u32()?; // 0

    Ok(())
}

fn read_chunk_09003003<R: Read, I>(r: &mut Reader<R, I, NodeState>) -> Result<()> {
    r.u32()?; // 2
    r.list(|r| {
        // 2
        r.u32()?; // 0
        r.node(0x090fd000, |r| {
            loop {
                let chunk_id = r.u32()?;

                match chunk_id {
                    0x090fd000 => read_chunk_090fd000(r)?,
                    0x090fd001 => read_chunk_090fd001(r)?,
                    0x090fd002 => read_chunk_090fd002(r)?,
                    0xfacade01 => break,
                    _ => todo!(),
                }
            }

            Ok(())
        })?;

        Ok(())
    })?;

    Ok(())
}

fn read_chunk_09003004<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // "piks"
    r.u32()?; // 12
    r.u32()?; // 1
    r.u32()?; // 0
    r.u32()?; // 1

    Ok(())
}

fn read_chunk_09003005<R: Read, N>(r: &mut Reader<R, IdState, N>) -> Result<()> {
    r.u32()?; // 0
    r.u32()?; // 1
    r.u32()?; // 0
    r.u32()?; // 2
    r.u32()?; // 0
    r.id()?; // "Layer0"
    r.string()?; // "Geometry"
    r.u32()?; // 1
    r.u32()?; // 1
    r.u32()?; // 37
    r.u32()?; // 4
    r.u32()?; // 3
    r.u32()?; // 4
    r.f32()?; // 64.0
    r.u32()?; // 2
    r.f32()?; // 128.0
    r.u32()?; // 1
    r.f32()?; // 192.0
    r.u32()?; // 0
    let num_groups = r.u32()?;
    r.repeat(num_groups as usize, |r| {
        r.u32()?; // 0
        r.u8()?; // 1
        r.u32()?; // 0xffffffff
        r.string()?; // "" | "part"
        r.u32()?; // 0xffffffff
        r.list(|r| {
            r.u32()?;

            Ok(())
        })?;

        Ok(())
    })?;
    r.u8()?; // 1
    r.u32()?; // 1
    r.u32()?; // 35
    let num_vertices = r.u32()?;
    r.repeat(num_vertices as usize, |r| {
        r.f32()?;
        r.f32()?;
        r.f32()?;

        Ok(())
    })?;
    r.u32()?; // 0x330
    r.u32()?; // 0
    let num_faces = r.u32()?; // 0x144
    r.list(|r| {
        r.f32()?;
        r.f32()?;

        Ok(())
    })?;
    let num_face_indices = r.u32()?;
    r.repeat(num_face_indices as usize, |r| {
        read_compact_index(r, num_face_indices)?;

        Ok(())
    })?;
    r.repeat(num_faces as usize, |r| {
        let index_count = r.u8()? + 3;
        r.repeat(index_count as usize, |r| {
            read_compact_index(r, num_vertices)?;

            Ok(())
        })?;
        read_compact_index(r, 64)?;
        read_compact_index(r, num_groups)?;

        Ok(())
    })?;
    r.u32()?; // 0
    r.list(|r| {
        r.u32()?;

        Ok(())
    })?;
    r.u32()?; // 1
    r.u32()?; // 1

    Ok(())
}

fn read_chunk_09003006<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 2
    r.list(|r| {
        r.u32()?;

        Ok(())
    })?;
    let len = r.u32()?;
    r.repeat(len as usize, |r| {
        read_compact_index(r, len)?;

        Ok(())
    })?;

    Ok(())
}

fn read_chunk_09003007<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 0
    r.u32()?; // 3
    r.u32()?; // 0
    r.f32()?; // 1.0
    r.f32()?; // 2.0
    r.list(|r| {
        r.u32()?;

        Ok(())
    })?;

    Ok(())
}

fn read_chunk_090fd000<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 11
    r.u8()?; // 1
    r.u32()?; // 0xffffffff
    r.u32()?; // 0xffffffff
    r.u32()?; // 0
    r.u16()?; // 4 | 22
    r.string()?; // "Stadium\Media\Material\TechnicsTrims" | "Stadium\Media\Material\TrackWallClips"
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0
    r.u32()?; // 0xffffffff

    Ok(())
}

fn read_chunk_090fd001<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 5
    r.u32()?; // 0xffffffff
    r.u32()?; // 0
    r.u32()?; // 0
    r.f32()?; // 1.0
    r.u32()?; // 0
    r.u32()?; // 0

    Ok(())
}

fn read_chunk_090fd002<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<()> {
    r.u32()?; // 0
    r.u32()?; // 0

    Ok(())
}

fn read_compact_index<R: Read, I, N>(r: &mut Reader<R, I, N>, num_items: u32) -> Result<u32> {
    if num_items < u8::MAX as u32 {
        let index = r.u8()?;
        Ok(index as u32)
    } else if num_items < u16::MAX as u32 {
        let index = r.u16()?;
        Ok(index as u32)
    } else {
        r.u32()
    }
}
