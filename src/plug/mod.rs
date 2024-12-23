//! Plug engine.

pub mod bitmap;
pub mod crystal;
pub mod curve_simple_nod;
pub mod dyna_kinematic_contraint;
pub mod dyna_object_model;
pub mod editor_helper;
pub mod ent_record_data;
pub mod entity_spawner;
pub mod fx_hdr_scales_tech_3;
pub mod game_skin;
pub mod game_skin_and_folder;
pub mod index_buffer;
pub mod item_placement;
pub mod item_variant_list;
pub mod light;
pub mod light_user_model;
pub mod material;
pub mod material_custom;
pub mod material_user_inst;
pub mod media_clip_list;
pub mod path;
pub mod placement_patch;
pub mod poly_line_3;
pub mod prefab;
pub mod road_chunk;
pub mod skel;
pub mod solid;
pub mod solid_2_model;
pub mod spawn_model;
pub mod static_object_model;
pub mod surface;
pub mod tree;
pub mod tree_generator;
pub mod vertex_stream;
pub mod visual;
pub mod visual3d;
pub mod visual_indexed;
pub mod visual_indexed_triangles;

#[doc(inline)]
pub use bitmap::Bitmap;
#[doc(inline)]
pub use crystal::Crystal;
#[doc(inline)]
pub use curve_simple_nod::CurveSimpleNod;
#[doc(inline)]
pub use dyna_object_model::DynaObjectModel;
#[doc(inline)]
pub use editor_helper::EditorHelper;
#[doc(inline)]
pub use fx_hdr_scales_tech_3::FxHdrScalesTech3;
#[doc(inline)]
pub use game_skin::GameSkin;
#[doc(inline)]
pub use game_skin_and_folder::GameSkinAndFolder;
#[doc(inline)]
pub use index_buffer::IndexBuffer;
#[doc(inline)]
pub use light::Light;
#[doc(inline)]
pub use light_user_model::LightUserModel;
#[doc(inline)]
pub use material::Material;
#[doc(inline)]
pub use material_custom::MaterialCustom;
#[doc(inline)]
pub use material_user_inst::MaterialUserInst;
#[doc(inline)]
pub use media_clip_list::MediaClipList;
#[doc(inline)]
pub use path::Path;
#[doc(inline)]
pub use placement_patch::PlacementPatch;
#[doc(inline)]
pub use prefab::Prefab;
#[doc(inline)]
pub use road_chunk::RoadChunk;
#[doc(inline)]
pub use skel::Skel;
#[doc(inline)]
pub use solid::Solid;
#[doc(inline)]
pub use solid_2_model::Solid2Model;
#[doc(inline)]
pub use spawn_model::SpawnModel;
#[doc(inline)]
pub use static_object_model::StaticObjectModel;
#[doc(inline)]
pub use surface::Surface;
#[doc(inline)]
pub use tree::Tree;
#[doc(inline)]
pub use tree_generator::TreeGenerator;
#[doc(inline)]
pub use vertex_stream::VertexStream;
#[doc(inline)]
pub use visual::Visual;
#[doc(inline)]
pub use visual3d::Visual3D;
#[doc(inline)]
pub use visual_indexed::VisualIndexed;
#[doc(inline)]
pub use visual_indexed_triangles::VisualIndexedTriangles;

/// Texture coordinate.
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Texcoord {
    /// U component.
    pub u: f32,
    /// V component.
    pub v: f32,
}

impl Texcoord {
    /// From array `[u, v]`.
    pub const fn from_array(array: [f32; 2]) -> Self {
        Self {
            u: array[0],
            v: array[1],
        }
    }

    /// To array `[u, v]`.
    pub const fn to_array(self) -> [f32; 2] {
        [self.u, self.v]
    }
}
