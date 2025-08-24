use crate::read::{BodyReader, Error, ReadEnum, ReadNode, Result, read_body_chunks};

pub struct MediaBlockCameraGame {
    chunk_7: Chunk7,
}

struct Chunk7;

enum GameCam {
    Default,
    Internal,
    External,
    Helico,
    Free,
    Spectator,
    External2,
}

impl ReadEnum for GameCam {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::Default),
            1 => Ok(Self::Internal),
            2 => Ok(Self::External),
            3 => Ok(Self::Helico),
            4 => Ok(Self::Free),
            5 => Ok(Self::Spectator),
            6 => Ok(Self::External2),
            _ => todo!("{index}"),
        }
    }
}

impl ReadNode for MediaBlockCameraGame {
    const CLASS_ID: u32 = 0x03084000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_7: r.chunk(0x03084007, |r| {
                    if r.u32()? != 4 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _start_time = r.f32()?;
                    let _end_time = r.f32()?;
                    let _game_cam = r.enum32::<GameCam>()?;
                    let _clip_ent_id = r.u32()?;
                    let _cam_position = r.vec3_f32()?;
                    let _cam_yaw_pitch_roll = r.yaw_pitch_roll()?;
                    let _cam_fov = r.f32()?;
                    r.f32()?;
                    r.f32()?;
                    let _cam_near_clip_plane = r.f32()?;
                    let _cam_far_clip_plane = r.f32()?;
                    r.bool32()?;
                    r.bool32()?;
                    r.bool32()?;
                    r.f32()?;
                    r.u32()?;

                    Ok(Chunk7)
                })?,
            })
        })
    }
}
