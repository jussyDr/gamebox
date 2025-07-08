use gamebox::{
    class::{
        game::ctn::{block_info_classic::BlockInfoClassic, challenge::Challenge},
        plug::{
            bitmap::Bitmap, dyna_object_model::DynaObjectModel, material::Material, prefab::Prefab,
            solid_2_model::Solid2Model, static_object_model::StaticObjectModel, surface::Surface,
        },
    },
    read::read_file,
};

#[test]
fn read_dyna_object_flag() {
    let _dyna_object: DynaObjectModel =
        read_file("tests/files/dyna_object/Flag.DynaObject.Gbx").unwrap();
}

#[test]
fn read_dyna_object_light_ray() {
    let _dyna_object: DynaObjectModel =
        read_file("tests/files/dyna_object/LightRay.DynaObject.Gbx").unwrap();
}

#[test]
fn read_ed_classic_canopy_center_flat_base() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/CanopyCenterFlatBase.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_deco_cliff_10_corner_in() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/DecoCliff10CornerIn.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_deco_cliff_10_dirt_corner_in() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/DecoCliff10DirtCornerIn.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_deco_cliff_dirt_mid_corner_in() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/DecoCliffDirtMidCornerIn.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_deco_wall_arch_slope_2_end() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/DecoWallArchSlope2End.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_deco_wall_water_base() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/DecoWallWaterBase.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_gate_special_boost() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/GateSpecialBoost.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_open_dirt_road_checkpoint() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/OpenDirtRoadCheckpoint.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_platform_dirt_finish() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/PlatformDirtFinish.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_platform_dirt_loop_out_start() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/PlatformDirtLoopOutStart.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_road_tech_straight() {
    let _ed_classic: BlockInfoClassic =
        read_file("tests/files/ed_classic/RoadTechStraight.EDClassic.Gbx").unwrap();
}

#[test]
fn read_hit_shape_stade1536v2() {
    let _hit_shape: Surface = read_file("tests/files/hit_shape/Stade1536v2.HitShape.Gbx").unwrap();
}

#[test]
fn read_map_alive() {
    let _map: Challenge = read_file("tests/files/map/Alive.Map.Gbx").unwrap();
}

#[test]
fn read_material_road_tech() {
    let _material: Material = read_file("tests/files/material/RoadTech.Material.gbx").unwrap();
}

#[test]
fn read_material_track_borders() {
    let _material: Material = read_file("tests/files/material/TrackBorders.Material.gbx").unwrap();
}

#[test]
fn read_mesh_stade1536v2() {
    let _mesh: Solid2Model = read_file("tests/files/mesh/Stade1536v2.Mesh.Gbx").unwrap();
}

#[test]
fn read_prefab_stade1536v2() {
    let _prefab: Prefab = read_file("tests/files/prefab/Stade1536v2.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_straight_air() {
    let _prefab: Prefab = read_file("tests/files/prefab/Straight_Air.Prefab.Gbx").unwrap();
}

#[test]
fn read_static_object_grass4096() {
    let _static_object: StaticObjectModel =
        read_file("tests/files/static_object/Grass4096.StaticObject.Gbx").unwrap();
}

#[test]
fn read_texture_road_tech_d() {
    let _texture: Bitmap = read_file("tests/files/texture/RoadTech_D.Texture.gbx").unwrap();
}
