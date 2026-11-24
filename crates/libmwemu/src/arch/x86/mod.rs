pub mod context;
pub mod eflags;
pub mod flags;
pub mod fpu;
pub mod regs;

pub(crate) const XSAVE_X87: u64 = 1;
pub(crate) const XSAVE_SSE: u64 = 1 << 1;
pub(crate) const XSAVE_AVX: u64 = 1 << 2;
pub(crate) const XCR0_ENABLED_MASK: u64 = XSAVE_X87 | XSAVE_SSE | XSAVE_AVX;
pub(crate) const XSAVE_AVX_OFFSET: u64 = 576;
pub(crate) const XSAVE_AVX_BYTES: u64 = 256;
pub(crate) const XSAVE_AREA_BYTES: u64 = XSAVE_AVX_OFFSET + XSAVE_AVX_BYTES;
