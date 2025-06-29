pub trait FromLe {
    /// Converts the `value` from little endian to the target's endianness.
    fn from_le(value: Self) -> Self;
}

impl FromLe for u8 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for u16 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for u32 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for u64 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for i8 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for i16 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for i32 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for i64 {
    fn from_le(value: Self) -> Self {
        Self::from_le(value)
    }
}

impl FromLe for f32 {
    fn from_le(value: Self) -> Self {
        Self::from_bits(u32::from_le(value.to_bits()))
    }
}

impl FromLe for f64 {
    fn from_le(value: Self) -> Self {
        Self::from_bits(u64::from_le(value.to_bits()))
    }
}
