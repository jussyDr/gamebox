use std::io::Cursor;

use gamebox::{
    game::{
        ctn::{
            block_info_classic::BlockInfoClassic, challenge::Challenge, decoration::Decoration,
            BlockInfoClip, BlockInfoClipHorizontal, BlockInfoClipVertical, BlockInfoFlat,
            Collection, DecorationMood,
        },
        item_model::ItemModel,
        ManiaTitle,
    },
    plug::{
        bitmap::Bitmap, material::Material, prefab::Prefab, solid_2_model::Solid2Model,
        VegetTreeModel,
    },
    read, read_file, write,
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
fn read_block_test() {
    read_file::<ItemModel>("tests/files/block/Test.Block.Gbx").unwrap();
}

#[test]
fn read_collection_stadium() {
    read_file::<Collection>("tests/files/collection/Stadium.Collection.Gbx").unwrap();
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
fn read_decoration_mood_day() {
    read_file::<DecorationMood>("tests/files/decoration_mood/Day.DecorationMood.Gbx").unwrap();
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
fn read_ed_classic_gate_special_boost() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/GateSpecialBoost.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_open_dirt_road_checkpoint() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/OpenDirtRoadCheckpoint.EDClassic.Gbx")
        .unwrap();
}

#[test]
fn read_ed_classic_platform_dir_curve_2_in() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/PlatformDirtCurve2In.EDClassic.Gbx")
        .unwrap();
}

#[test]
fn read_ed_classic_platform_dirt_finish() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/PlatformDirtFinish.EDClassic.Gbx")
        .unwrap();
}

#[test]
fn read_ed_classic_road_bump_special_boost_slope() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/RoadBumpSpecialBoostSlope.EDClassic.Gbx")
        .unwrap();
}

#[test]
fn read_ed_classic_road_tech_straight() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/RoadTechStraight.EDClassic.Gbx").unwrap();
}

#[test]
fn read_ed_classic_track_wall_chicane_2x_left_pillar() {
    read_file::<BlockInfoClassic>(
        "tests/files/ed_classic/TrackWallChicaneX2LeftPillar.EDClassic.Gbx",
    )
    .unwrap();
}

#[test]
fn read_ed_clip_canopy_border_beam_curve_in_fc_left() {
    read_file::<BlockInfoClip>("tests/files/ed_clip/CanopyBorderBeamCurveInFCLeft.EDClip.Gbx")
        .unwrap();
}

#[test]
fn read_ed_clip_grass_clip() {
    read_file::<BlockInfoClip>("tests/files/ed_clip/GrassClip.EDClip.Gbx").unwrap();
}

#[test]
fn read_ed_clip_open_dirt_border_fc_left() {
    read_file::<BlockInfoClip>("tests/files/ed_clip/OpenDirtBorderFCLeft.EDClip.Gbx").unwrap();
}

#[test]
fn read_ed_clip_open_dirt_road_fc() {
    read_file::<BlockInfoClip>("tests/files/ed_clip/OpenDirtRoadFC.EDClip.Gbx").unwrap();
}

#[test]
fn read_ed_clip_platform_slope_2_end_curve_2_in_fcb() {
    read_file::<BlockInfoClip>("tests/files/ed_clip/PlatformSlope2EndCurve2InFCB.EDClip.Gbx")
        .unwrap();
}

#[test]
fn read_ed_flat_grass() {
    read_file::<BlockInfoFlat>("tests/files/ed_flat/Grass.EDFlat.Gbx").unwrap();
}

#[test]
fn read_ed_horizontal_clip_canopy_center_flat_curve_in_hfc_left() {
    read_file::<BlockInfoClipHorizontal>(
        "tests/files/ed_horizontal_clip/CanopyCenterFlatCurveInHFCLeft.EDHorizontalClip.Gbx",
    )
    .unwrap();
}

#[test]
fn read_ed_horizontal_clip_platform_water_hfc_inside() {
    read_file::<BlockInfoClipHorizontal>(
        "tests/files/ed_horizontal_clip/PlatformWaterHFCInside.EDHorizontalClip.Gbx",
    )
    .unwrap();
}

#[test]
fn read_ed_vertical_clip_deco_cliff_corner_in_left_vfc() {
    read_file::<BlockInfoClipVertical>(
        "tests/files/ed_vertical_clip/DecoCliffCornerInLeftVFC.EDVerticalClip.Gbx",
    )
    .unwrap();
}

#[test]
fn read_ed_vertical_clip_gate_expandable_checkpoint_left_vfc() {
    read_file::<BlockInfoClipVertical>(
        "tests/files/ed_vertical_clip/GateExpandableCheckpointLeftVFC.EDVerticalClip.Gbx",
    )
    .unwrap();
}

#[test]
fn read_item_9_castle_tech_checkpoint() {
    read_file::<ItemModel>("tests/files/item/9_CastleTechCheckpoint.Item.Gbx").unwrap();
}

#[test]
fn read_item_blue() {
    read_file::<ItemModel>("tests/files/item/blue.Item.Gbx").unwrap();
}

#[test]
fn read_item_cactus_medium() {
    read_file::<ItemModel>("tests/files/item/CactusMedium.Item.Gbx").unwrap();
}

#[test]
fn read_item_cp_stripe() {
    read_file::<ItemModel>("tests/files/item/CP-stripe.Item.Gbx").unwrap();
}

#[test]
fn read_item_desert_gate_gameplay() {
    read_file::<ItemModel>("tests/files/item/DesertGateGameplay.Item.Gbx").unwrap();
}

#[test]
fn read_item_gate_checkpoint_center_16_m() {
    read_file::<ItemModel>("tests/files/item/GateCheckpointCenter16m.Item.Gbx").unwrap();
}

#[test]
fn read_item_gate_gameplay_desert_16_m() {
    read_file::<ItemModel>("tests/files/item/GateGameplayDesert16m.Item.Gbx").unwrap();
}

#[test]
fn read_item_piege_gbx() {
    read_file::<ItemModel>("tests/files/item/Piege.Gbx.Item.Gbx").unwrap();
}

#[test]
fn read_item_podium() {
    read_file::<ItemModel>("tests/files/item/Podium.Item.Gbx").unwrap();
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
fn read_map_acp_17_radiant_winter() {
    read_file::<Challenge>("tests/files/map/ACP#17 - Radiant Winter.Map.Gbx").unwrap();
}

#[test]
fn read_map_deco_48x48() {
    read_file::<Challenge>("tests/files/map/Deco48x48.Map.Gbx").unwrap();
}

#[test]
fn read_map_deep_dip() {
    read_file::<Challenge>("tests/files/map/Deep Dip.Map.Gbx").unwrap();
}

#[test]
fn read_map_deep_dip_2r1() {
    read_file::<Challenge>("tests/files/map/Deep_Dip_2r1.Map.Gbx").unwrap();
}

#[test]
fn read_map_midnight_metropolis() {
    read_file::<Challenge>("tests/files/map/MIDNIGHT METROPOLIS.Map.Gbx").unwrap();
}

#[test]
fn read_map_mindor() {
    let challenge_1: Challenge = read_file("tests/files/map/Mindor.Map.Gbx").unwrap();

    let mut buf = vec![];
    write(&challenge_1, Cursor::new(&mut buf)).unwrap();

    let challenge_2: Challenge = read(Cursor::new(buf)).unwrap();

    if challenge_1 != challenge_2 {
        panic!()
    }
}

#[test]
fn read_map_new() {
    read_file::<Challenge>("tests/files/map/New.Map.Gbx").unwrap();
}

#[test]
fn read_maps_pocket_universe() {
    read_file::<Challenge>("tests/files/map/Pocket Universe.Map.Gbx").unwrap();
}

#[test]
fn read_map_thread_of_ariadne() {
    read_file::<Challenge>("tests/files/map/Thread of Ariadne.Map.Gbx").unwrap();
}

#[test]
fn read_map_training_01() {
    read_file::<Challenge>("tests/files/map/Training - 01.Map.Gbx").unwrap();
}

#[test]
fn read_material_canopy() {
    read_file::<Material>("tests/files/material/Canopy.Material.Gbx").unwrap();
}

#[test]
fn read_material_collision_canopy() {
    read_file::<Material>("tests/files/material/CollisionCanopy.Material.Gbx").unwrap();
}

#[test]
fn read_material_decal_paint_2_logo_4x1() {
    read_file::<Material>("tests/files/material/DecalPaint2Logo4x1.Material.gbx").unwrap();
}

#[test]
fn read_material_decal_sponsor_1x1_big_a_on_road_ice() {
    read_file::<Material>("tests/files/material/DecalSponsor1x1BigAOnRoadIce.Material.Gbx")
        .unwrap();
}

#[test]
fn read_material_deco_hill() {
    read_file::<Material>("tests/files/material/DecoHill.Material.Gbx").unwrap();
}

#[test]
fn read_material_grass() {
    read_file::<Material>("tests/files/material/Grass.Material.Gbx").unwrap();
}

#[test]
fn read_material_grass_fence() {
    read_file::<Material>("tests/files/material/GrassFence.Material.Gbx").unwrap();
}

#[test]
fn read_material_light_spot() {
    read_file::<Material>("tests/files/material/LightSpot.Material.Gbx").unwrap();
}

#[test]
fn read_material_podium_screen_155() {
    read_file::<Material>("tests/files/material/PodiumScreen155.Material.Gbx").unwrap();
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
fn read_material_water() {
    read_file::<Material>("tests/files/material/Water.Material.gbx").unwrap();
}

#[test]
fn read_material_waterground() {
    read_file::<Material>("tests/files/material/Waterground.Material.Gbx").unwrap();
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
fn read_prefab_checkpoint_air() {
    read_file::<Prefab>("tests/files/prefab/Checkpoint_Air.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_checkpoint_air_npb() {
    read_file::<Prefab>("tests/files/prefab/Checkpoint_AirNPB.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_checkpoint_center_16m() {
    read_file::<Prefab>("tests/files/prefab/CheckpointCenter16m.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_corner_in_air() {
    read_file::<Prefab>("tests/files/prefab/CornerIn_Air.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_corner_in() {
    read_file::<Prefab>("tests/files/prefab/CornerIn.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_diag_in() {
    read_file::<Prefab>("tests/files/prefab/DiagIn.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_expandable_gameplay_ground() {
    read_file::<Prefab>("tests/files/prefab/ExpandableGameplay_Ground.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_finish_air() {
    read_file::<Prefab>("tests/files/prefab/Finish_Air.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_road_border_spot() {
    read_file::<Prefab>("tests/files/prefab/RoadBorderSpot.Prefab.Gbx").unwrap();
}

#[test]
fn read_prefab_special_ground() {
    read_file::<Prefab>("tests/files/prefab/Special_Ground.Prefab.Gbx").unwrap();
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
fn read_prefab_zone_end_large_left_air_v2() {
    read_file::<Prefab>("tests/files/prefab/ZoneEndLargeLeft_AirV2.Prefab.Gbx").unwrap();
}

#[test]
fn read_texture_technics_trims_d() {
    read_file::<Bitmap>("tests/files/texture/TechnicsTrims_D.Texture.gbx").unwrap();
}

#[test]
fn read_title_tm_stadium() {
    read_file::<ManiaTitle>("tests/files/title/TMStadium.Title.Gbx").unwrap();
}

#[test]
fn read_veget_tree_model_cherry_tree_medium() {
    read_file::<VegetTreeModel>("tests/files/veget_tree_model/CherryTreeMedium.VegetTreeModel.Gbx")
        .unwrap();
}

#[test]
fn read_veget_tree_model_cypress_dirt_tall() {
    read_file::<VegetTreeModel>("tests/files/veget_tree_model/CypressDirtTall.VegetTreeModel.Gbx")
        .unwrap();
}

#[test]
fn read_veget_tree_model_fall_tree_medium() {
    read_file::<VegetTreeModel>("tests/files/veget_tree_model/FallTreeMedium.VegetTreeModel.Gbx")
        .unwrap();
}
