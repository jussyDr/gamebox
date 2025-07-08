//! Block info mobil.

use crate::{ClassId, ExternalNodeRef};

/// Block info mobil.
#[derive(Default)]
pub struct BlockInfoMobil {
    prefab: Option<ExternalNodeRef>,
}

impl BlockInfoMobil {
    /// Prefab.
    pub fn prefab(&self) -> &Option<ExternalNodeRef> {
        &self.prefab
    }
}

impl ClassId for BlockInfoMobil {
    const CLASS_ID: u32 = 0x03122000;
}

mod read {
    use crate::{
        Delme,
        class::{
            game::ctn::block_info_mobil::BlockInfoMobil,
            plug::{placement_patch::PlacementPatch, prefab::Prefab},
        },
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for BlockInfoMobil {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for BlockInfoMobil {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(2, Self::read_chunk_2),
                BodyChunk::new(3, Self::read_chunk_3),
                BodyChunk::new(4, Self::read_chunk_4),
            ]
        }
    }

    impl BlockInfoMobil {
        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _solid_decals: Vec<()> = r.list_with_version(|r| todo!())?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_3(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 23 {
                return Err(error_unknown_chunk_version(version));
            }

            let _solid_frequency = r.u32()?;

            if r.bool8()? {
                let _geom_translation = r.vec3()?;
                let _geom_rotation = r.vec3()?;
            }

            let solid_fid = r.external_node_ref_or_null::<Delme>()?;

            if solid_fid.is_none() {
                let _old_mobil = r.u32()?;
            }
            self.prefab = r.external_node_ref_or_null::<Prefab>()?;
            let _old_solid_aggreg = r.external_node_ref_or_null::<Delme>()?;
            let _rail_path = r.external_node_ref_or_null::<Delme>()?;
            r.u32()?;
            let _road_chunks: Vec<()> = r.list(|r| todo!())?;
            r.list(|r| r.u32())?;
            let _vfxs = r.u32()?;

            if matches!(r.u8()?, 0 | 1) {
                r.f32()?;
            }
            r.vec3()?;
            r.vec3()?;
            r.f32()?;
            r.list(|r| r.internal_node_ref::<PlacementPatch>())?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_4(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let _dyna_links: Vec<()> = r.list_with_version(|r| todo!())?;

            Ok(())
        }
    }
}
