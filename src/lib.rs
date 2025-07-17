#![warn(clippy::todo, clippy::unwrap_used)]

pub mod read;

pub mod game {
    pub mod ctn {
        mod block;
        mod block_skin;
        mod challenge;

        pub use block::Block;
        pub use block_skin::BlockSkin;
        pub use challenge::Challenge;
    }

    mod waypoint_special_property;

    pub use waypoint_special_property::WaypointSpecialProperty;
}

pub use game::ctn::Challenge;
pub use read::{read, read_file};

pub struct U8Vec3;

pub struct U32Vec3;
