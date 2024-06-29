//! Engines.

/// Game engine.
pub mod game {
    pub mod block_info_classic;
    pub mod block_info_groups;
    pub mod block_info_tree_root;
    pub mod ghost;
    pub mod item_model_tree_root;
    pub mod macroblock;
    pub mod map;
    pub mod zone_genealogy;
}

/// Game data engine.
pub mod game_data {
    pub mod collector;
    pub mod item;
    pub mod waypoint_special_property;
}

/// Meta engine.
pub mod meta {
    pub mod veget_tree_model;
}

/// Plug engine.
pub mod plug {
    pub mod color_table;
    pub mod crystal;
    pub mod ent_record_data;
    pub mod light_user_model;
    pub mod material;
    pub mod material_user_inst;
    pub mod prefab;
    pub mod static_object_model;
    pub mod surface;
    pub mod texture;
    pub mod visual_indexed_triangles;
}

/// Script engine.
pub mod script {
    pub mod traits_metadata;
}
