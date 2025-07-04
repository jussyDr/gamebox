//! Light.

use crate::SubExtensions;

/// A light.
pub struct Light;

impl SubExtensions for Light {
    const SUB_EXTENSIONS: &[&str] = &["Light"];
}
