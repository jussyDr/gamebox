use gamebox::{
    game::{
        ctn::{block_info_classic::BlockInfoClassic, challenge::Challenge, decoration::Decoration},
        item_model::ItemModel,
    },
    plug::{bitmap::Bitmap, material::Material, prefab::Prefab, solid_2_model::Solid2Model},
    read_file,
};

#[test]
fn read_block_grass_remover() {
    read_file::<ItemModel>("tests/files/block/GrassRemover.Block.Gbx").unwrap();
}

#[test]
fn read_block_grass_remover2() {
    read_file::<ItemModel>("tests/files/block/GrassRemover2.Block.Gbx").unwrap();
}

#[test]
fn read_block_rally_road_ice_high_curve_2_snow() {
    read_file::<ItemModel>("tests/files/block/RallyRoadIceHighCurve2-Snow.Block.Gbx").unwrap();
}

#[test]
fn read_decoration_base_48x48_day() {
    read_file::<Decoration>("tests/files/decoration/Base48x48Day.Decoration.Gbx").unwrap();
}

#[test]
fn read_decoration_day_16x12() {
    read_file::<Decoration>("tests/files/decoration/Day16x12.Decoration.Gbx").unwrap();
}

#[test]
fn read_ed_classic_deco_cliff_mid_corner_in() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/DecoCliffIceMidCornerIn.EDClassic.Gbx")
        .unwrap();
}

#[test]
fn read_ed_classic_deco_wall_slope_2_u_buttom_in_ground() {
    read_file::<BlockInfoClassic>(
        "tests/files/ed_classic/DecoWallSlope2UBottomInGround.EDClassic.Gbx",
    )
    .unwrap();
}

#[test]
fn read_ed_classic_deco_wall_water_base() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/DecoWallWaterBase.EDClassic.Gbx")
        .unwrap();
}

#[test]
fn read_ed_classic_road_tech_straight() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/RoadTechStraight.EDClassic.Gbx").unwrap();
}

#[test]
fn read_item_blue() {
    read_file::<ItemModel>("tests/files/item/blue.Item.Gbx").unwrap();
}

#[test]
fn read_item_cp_stripe() {
    read_file::<ItemModel>("tests/files/item/CP-stripe.Item.Gbx").unwrap();
}

#[test]
fn read_item_piege_gbx() {
    read_file::<ItemModel>("tests/files/item/Piege.Gbx.Item.Gbx").unwrap();
}

#[test]
fn read_iten_pt_triangle_up_right_front_yellow_booster() {
    read_file::<ItemModel>("tests/files/item/PT_Triangle_Up_Right_Front_Yellow_Booster.Item.gbx")
        .unwrap();
}

#[test]
fn read_item_ramp_2k() {
    read_file::<ItemModel>("tests/files/item/Ramp2k.Item.Gbx").unwrap();
}

#[test]
fn read_item_rock_plate_2_gray() {
    read_file::<ItemModel>("tests/files/item/Rock_Plate_2_gray.Item.gbx").unwrap();
}

#[test]
fn read_item_wrh_p_ql_r_3_2() {
    read_file::<ItemModel>("tests/files/item/WRH_P_QL_R_3_2.Item.Gbx").unwrap();
}

#[test]
fn read_map_deco_48x48() {
    read_file::<Challenge>("tests/files/map/Deco48x48.Map.Gbx").unwrap();
}

#[test]
fn read_map_deep_dip_2r1() {
    read_file::<Challenge>("tests/files/map/Deep_Dip_2r1.Map.Gbx").unwrap();
}

#[test]
fn read_map_mindor() {
    read_file::<Challenge>("tests/files/map/Mindor.Map.Gbx").unwrap();
}

#[test]
fn read_map_new() {
    read_file::<Challenge>("tests/files/map/New.Map.Gbx").unwrap();
}

#[test]
fn read_map_training_01() {
    read_file::<Challenge>("tests/files/map/Training - 01.Map.Gbx").unwrap();
}

#[test]
fn read_material_decal_paint_2_logo_4x1() {
    read_file::<Material>("tests/files/material/DecalPaint2Logo4x1.Material.gbx").unwrap();
}

#[test]
fn read_material_light_spot() {
    read_file::<Material>("tests/files/material/LightSpot.Material.Gbx").unwrap();
}

#[test]
fn read_material_road_tech() {
    read_file::<Material>("tests/files/material/RoadTech.Material.Gbx").unwrap();
}

#[test]
fn read_material_technics_trims() {
    read_file::<Material>("tests/files/material/TechnicsTrims.Material.Gbx").unwrap();
}

#[test]
fn read_mesh_stade_4096() {
    read_file::<Solid2Model>("tests/files/mesh/Stade4096.Mesh.Gbx").unwrap();
}

#[test]
fn read_prefab_branch_cross_air() {
    read_file::<Prefab>("tests/files/prefab/BranchCross_Air.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_road_border_spot() {
    read_file::<Prefab>("tests/files/prefab/RoadBorderSpot.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_stade_4096() {
    read_file::<Prefab>("tests/files/prefab/Stade4096.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_straight_air() {
    read_file::<Prefab>("tests/files/prefab/Straight_Air.Prefab.Gbx").unwrap();
}

#[test]
fn read_texture_technics_trims_d() {
    read_file::<Bitmap>("tests/files/texture/TechnicsTrims_D.Texture.gbx").unwrap();
}
