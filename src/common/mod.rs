/// Color representation using red, green, and blue components.
///
/// Each component is represented as an 8-bit unsigned integer.
#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Rgb {
    /// Convert to an array with the form `[r, g, b]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # |color: gamebox::Rgb| {
    /// let array = color.into_array();
    /// # };
    /// ```
    pub const fn into_array(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

/// A 3-dimensional vector with components of type `T`.
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy> Vec3<T> {
    /// Convert to an array with the form `[x, y, z]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # |vec3: gamebox::Vec3<f32>| {
    /// let array = vec3.into_array();
    /// # };
    /// ```
    pub const fn into_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}
