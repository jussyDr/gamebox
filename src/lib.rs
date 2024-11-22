#![warn(clippy::print_stdout)]

pub mod engines;
pub mod read;

pub use engines::plug::prefab::Prefab;
pub use read::{read, read_file};

pub trait Class: Sized {
    const CLASS_ID: u32;
}

#[derive(Clone, Copy)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
