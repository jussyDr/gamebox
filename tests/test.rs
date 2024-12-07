use std::io::Cursor;

use gamebox::{
    game::{
        ctn::{block_info_classic::BlockInfoClassic, challenge::Challenge, decoration::Decoration},
        item_model::ItemModel,
    },
    plug::{bitmap::Bitmap, material::Material, prefab::Prefab, solid_2_model::Solid2Model},
    read, read_file, write,
};

#[test]
fn read_block_grass_remover() {
    read_file::<ItemModel>("tests/files/block/GrassRemover.Block.Gbx").unwrap();
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
fn read_ed_classic_road_tech_straight() {
    read_file::<BlockInfoClassic>("tests/files/ed_classic/RoadTechStraight.EDClassic.Gbx").unwrap();
}

#[test]
fn read_item_cp_stripe() {
    read_file::<ItemModel>("tests/files/item/CP-stripe.Item.Gbx").unwrap();
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
fn read_map_new() {
    read_file::<Challenge>("tests/files/map/New.Map.Gbx").unwrap();
}

#[test]
fn read_map_training_01() {
    read_file::<Challenge>("tests/files/map/Training - 01.Map.Gbx").unwrap();
}

#[test]
fn read_material_decal_paint_2_logo_4x1() {
    read_file::<Material>("tests/files/material/DecalPaint2Logo4x1.Material.Gbx").unwrap();
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
    read_file::<Bitmap>("tests/files/texture/TechnicsTrims_D.Texture.Gbx").unwrap();
}

#[test]
fn write_item() {
    let item = ItemModel::default();

    let mut buf = vec![];
    write(&item, Cursor::new(&mut buf)).unwrap();

    let item_2 = read::<ItemModel>(Cursor::new(buf)).unwrap();
    assert_eq!(item, item_2);
}

#[test]
fn write_map() {
    let map = Challenge::default();

    let mut buf = vec![];
    write(&map, Cursor::new(&mut buf)).unwrap();

    let map_2 = read::<Challenge>(Cursor::new(buf)).unwrap();
    assert_eq!(map, map_2);
}
