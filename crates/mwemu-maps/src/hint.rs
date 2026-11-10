//! Branch hints for hot paths.

/// `b`, telling the compiler that it is usually true.
#[inline(always)]
pub fn likely(b: bool) -> bool {
    if b {
        true
    } else {
        std::hint::cold_path();
        false
    }
}

/// `b`, telling the compiler that it is usually false.
#[inline(always)]
pub fn unlikely(b: bool) -> bool {
    if b {
        std::hint::cold_path();
        true
    } else {
        false
    }
}
