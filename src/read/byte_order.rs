//! Endianness conversions.

/// Type which can be converted from little endian to the target's endianness.
pub trait LeToNe {
    /// Converts `self` from little endian to the target's endianness.
    fn le_to_ne(&mut self);
}

impl LeToNe for u8 {
    fn le_to_ne(&mut self) {
        *self = Self::from_le(*self);
    }
}

impl LeToNe for u16 {
    fn le_to_ne(&mut self) {
        *self = Self::from_le(*self);
    }
}

impl LeToNe for u32 {
    fn le_to_ne(&mut self) {
        *self = Self::from_le(*self);
    }
}

impl LeToNe for u64 {
    fn le_to_ne(&mut self) {
        *self = Self::from_le(*self);
    }
}

impl LeToNe for i8 {
    fn le_to_ne(&mut self) {
        *self = Self::from_le(*self);
    }
}

impl LeToNe for i16 {
    fn le_to_ne(&mut self) {
        *self = Self::from_le(*self);
    }
}

impl LeToNe for i32 {
    fn le_to_ne(&mut self) {
        *self = Self::from_le(*self);
    }
}

impl LeToNe for i64 {
    fn le_to_ne(&mut self) {
        *self = Self::from_le(*self);
    }
}

impl LeToNe for f32 {
    fn le_to_ne(&mut self) {
        *self = Self::from_bits(u32::from_le(self.to_bits()));
    }
}

impl LeToNe for f64 {
    fn le_to_ne(&mut self) {
        *self = Self::from_bits(u64::from_le(self.to_bits()));
    }
}

impl<T: LeToNe> LeToNe for Vec<T> {
    fn le_to_ne(&mut self) {
        self.iter_mut().for_each(|value| value.le_to_ne());
    }
}
