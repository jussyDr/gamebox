mod class;
pub use class::{control, game, plug, scene, script};

pub mod read;
pub use read::{read, read_file};

pub mod write;
pub use write::{write, write_file};

pub use game::ctn::Challenge;

const FILE_SIGNATURE: [u8; 3] = [b'G', b'B', b'X'];
