#![warn(clippy::todo, clippy::unwrap_used)]

pub mod read;

pub use read::{read, read_file};

mod class {
    pub mod control {
        mod effect_simi;

        pub use effect_simi::EffectSimi;
    }

    pub mod game {
        pub mod ctn;

        mod waypoint_special_property;

        pub use waypoint_special_property::WaypointSpecialProperty;
    }

    pub mod plug {
        mod ent_record_data;

        pub use ent_record_data::EntRecordData;
    }

    pub mod script {
        mod traits_metadata;

        pub use traits_metadata::TraitsMetadata;
    }
}

pub use class::{game, script};
pub use game::ctn::Challenge;

pub struct F32Vec2 {
    x: f32,
    y: f32,
}

pub struct U8Vec3 {
    x: u8,
    y: u8,
    z: u8,
}

pub struct U32Vec3 {
    x: u32,
    y: u32,
    z: u32,
}

pub struct F32Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub struct F32Rgb {
    r: f32,
    g: f32,
    b: f32,
}

pub struct U32Box3 {
    a: U32Vec3,
    b: U32Vec3,
}
