use crate::read::{BodyReader, Error, ReadEnum, ReadNode, Reader, Result, read_body_chunks};

pub struct MediaBlockCameraCustom {
    chunk_6: Chunk6,
}

struct Chunk6;

enum Interpolation {
    None,
    Hermite,
    Linear,
    FixedTangent,
}

impl ReadEnum for Interpolation {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::None),
            1 => Ok(Self::Hermite),
            2 => Ok(Self::Linear),
            3 => Ok(Self::FixedTangent),
            _ => todo!(),
        }
    }
}

impl ReadNode for MediaBlockCameraCustom {
    const CLASS_ID: u32 = 0x030a2000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_6: r.chunk(0x030a2006, |r| {
                    if r.u32()? != 3 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _keys = r.list(|r| {
                        let _time = r.f32()?;
                        let _interpolation = r.enum32::<Interpolation>()?;
                        let _anchor_rot = r.bool32()?;
                        let _anchor = r.u32()?;
                        let _anchor_vis = r.bool32()?;
                        let _target = r.u32()?;
                        let _ = read_interp_val(r)?;
                        let _left_tangent = read_interp_val(r)?;
                        let _right_tangent = read_interp_val(r)?;

                        Ok(())
                    })?;

                    Ok(Chunk6)
                })?,
            })
        })
    }
}

fn read_interp_val(r: &mut impl Reader) -> Result<()> {
    let _position = r.vec3_f32()?;
    let _yaw_pitch_roll = r.yaw_pitch_roll()?;
    let _fov = r.f32()?;
    let _target_position = r.vec3_f32()?;
    let _near_z = r.f32()?;

    Ok(())
}
