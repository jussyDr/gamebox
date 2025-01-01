//! Material user inst.

use std::sync::Arc;

use crate::{read::reader::FromVariant, Class, RgbNat};

/// User defined material instance.
#[derive(Default, Debug)]
pub struct MaterialUserInst {
    name: Option<Arc<str>>,
    physic_id: u8,
    effect: Option<Effect>,
    link: MaterialLink,
    color: Option<RgbNat>,
}

impl Class for MaterialUserInst {
    const CLASS_ID: u32 = 0x090fd000;
}

impl MaterialUserInst {
    /// Effect.
    pub const fn effect(&self) -> Option<Effect> {
        self.effect
    }

    /// Link.
    pub const fn link(&self) -> &MaterialLink {
        &self.link
    }

    /// Color.
    pub const fn color(&self) -> Option<RgbNat> {
        self.color
    }
}

/// Material link.
#[derive(Debug)]
pub enum MaterialLink {
    /// Id.
    Id(Arc<str>),
    /// Path.
    Path(String),
}

impl Default for MaterialLink {
    fn default() -> Self {
        Self::Id(Default::default())
    }
}

/// Effect.
#[derive(Clone, Copy, Debug)]
pub enum Effect {
    /// Turbo.
    Turbo,
    /// Super turbo.
    Turbo2,
    /// Turbo roulette.
    TurboRoulette,
    /// Free wheeling.
    FreeWheeling,
    /// No grip.
    NoGrip,
    /// No steering.
    NoSteering,
    /// Force acceleration.
    ForceAcceleration,
    /// Reset.
    Reset,
    /// Slow motion.
    SlowMotion,
    /// Bumper.
    Bumper,
    /// Super bumper.
    Bumper2,
    /// Fragile.
    Fragile,
    /// No brakes.
    NoBrakes,
    /// Cruise.
    Cruise,
    /// Reactor boost.
    ReactorBoost,
    /// Super reactor boost.
    ReactorBoost2,
    /// Stadium car.
    VehicleReset,
    /// Snow car.
    VehicleCarSnow,
    /// Rally car.
    VehicleCarRally,
    /// Desert car.
    VehicleCarDesert,
}

impl FromVariant<u8> for Option<Effect> {
    fn from_variant(value: u8) -> Option<Self> {
        match value {
            0 => Some(None),
            1 => Some(Some(Effect::Turbo)),
            2 => Some(Some(Effect::Turbo2)),
            3 => Some(Some(Effect::TurboRoulette)),
            4 => Some(Some(Effect::FreeWheeling)),
            5 => Some(Some(Effect::NoGrip)),
            6 => Some(Some(Effect::NoSteering)),
            7 => Some(Some(Effect::ForceAcceleration)),
            8 => Some(Some(Effect::Reset)),
            9 => Some(Some(Effect::SlowMotion)),
            10 => Some(Some(Effect::Bumper)),
            11 => Some(Some(Effect::Bumper2)),
            12 => Some(Some(Effect::Fragile)),
            13 => Some(Some(Effect::NoBrakes)),
            14 => Some(Some(Effect::Cruise)),
            15 => Some(Some(Effect::ReactorBoost)),
            16 => Some(Some(Effect::ReactorBoost2)),
            17 => Some(Some(Effect::VehicleReset)),
            18 => Some(Some(Effect::VehicleCarSnow)),
            19 => Some(Some(Effect::VehicleCarRally)),
            20 => Some(Some(Effect::VehicleCarDesert)),
            _ => None,
        }
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{MaterialLink, MaterialUserInst};

    impl ReadBody for MaterialUserInst {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MaterialUserInst {
        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::normal(2, Self::read_chunk_2),
            ]
            .into_iter()
        }
    }

    impl MaterialUserInst {
        fn read_chunk_0<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 10 | 11) {
                return Err(Error::chunk_version(version));
            }

            let is_using_game_material = if version >= 11 { r.bool8()? } else { false };
            self.name = r.id_or_null()?;
            let _model = r.id_or_null()?;
            let _base_texture = r.string()?;
            self.physic_id = r.u8()?; // surface physic id (material id).
            self.effect = r.enum_u8()?;

            if version >= 11 && !is_using_game_material {
                self.link = MaterialLink::Id(r.id()?);
            } else {
                self.link = MaterialLink::Path(r.string()?);
            }

            let _csts = r.list(|r| {
                r.id()?;
                r.id()?;
                r.u32()?;

                Ok(())
            })?;
            if r.u32()? == 3 {
                self.color = Some(r.rgb_nat()?);
            }
            let _uv_anims = r.list(|r| {
                r.id()?;
                r.id()?;
                r.f32()?;
                r.u64()?;
                r.id()?;

                Ok(())
            })?;
            r.list(|r| r.id())?;
            let _user_textures = r.list(|r| {
                r.u32()?;
                let _texture = r.string()?;

                Ok(())
            })?;
            let _hiding_group = r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            let _tiling_u = r.u32()?;
            let _tiling_v = r.u32()?;
            let _texture_size_in_meters = r.f32()?;
            r.u32()?;
            let _is_natural = r.bool()?;

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }
    }
}
