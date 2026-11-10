//! Bit helpers for the flag and register code. `mwemu-core` re-exports them.

/// Bit `$count` of `$val`, as 0 or 1.
#[macro_export]
macro_rules! get_bit {
    ($val:expr, $count:expr) => {
        ($val & (1 << $count)) >> $count
    };
}

/// Set bit `$count` of `$val` to `$bit` (0 or 1).
#[macro_export]
macro_rules! set_bit {
    ($val:expr, $count:expr, $bit:expr) => {
        if $bit == 1 {
            $val |= 1 << $count;
        } else {
            $val &= !(1 << $count);
        }
    };
}
