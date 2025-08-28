use std::sync::Arc;

use crate::{
    game::ctn::{FileRef, Ident},
    plug::EntRecordData,
    read::{BodyReader, Error, ReadEnum, ReadNode, Result, read_body_chunks},
};

pub struct Entity {
    chunk_0: Chunk0,
}

struct Chunk0 {
    keys: Box<[Key]>,
}

struct Key;

enum Lights {
    Auto,
    On,
    Off,
}

impl ReadEnum for Lights {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::Auto),
            1 => Ok(Self::On),
            2 => Ok(Self::Off),
            _ => todo!("{index}"),
        }
    }
}

impl ReadNode for Entity {
    const CLASS_ID: u32 = 0x0329f000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            let chunk_0 = r.chunk(0x0329f000, |r| {
                if r.u32()? != 6 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                let _record_data = r.node_ref::<Arc<EntRecordData>>()?;
                let _start_offset = r.f32()?;
                let _notice_records = r.list(|r| r.u32())?;
                let _no_damage = r.bool32()?;
                r.bool32()?;
                let _force_light = r.u32()?;
                let _force_hue = r.bool32()?;
                let _player_model = Ident::read(r)?;
                r.vec3_f32()?;
                let _skin_names = r.list(|r| FileRef::read(r))?;

                if r.bool32()? {
                    panic!("{}", r.u32()?)
                }

                let keys = r.list(|r| {
                    let _time = r.f32()?;
                    let _lights = r.enum32::<Lights>()?;
                    r.f32()?;
                    r.u32()?;
                    r.u32()?;
                    let _trail_intensity = r.f32()?;

                    Ok(Key)
                })?;

                Ok(Chunk0 { keys })
            })?;

            Ok(Self { chunk_0 })
        })
    }
}
