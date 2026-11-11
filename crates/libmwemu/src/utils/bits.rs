/// Keep the least significant bits without changing the storage type.
/// Widths at least as large as the storage type leave the value unchanged.
#[inline]
pub(crate) const fn low_bits_u64(value: u64, bits: u32) -> u64 {
    if bits >= u64::BITS {
        value
    } else {
        value & ((1u64 << bits) - 1)
    }
}

/// Keep the least significant bits without changing the storage type.
/// Widths at least as large as the storage type leave the value unchanged.
#[inline]
pub(crate) const fn low_bits_u128(value: u128, bits: u32) -> u128 {
    if bits >= u128::BITS {
        value
    } else {
        value & ((1u128 << bits) - 1)
    }
}
