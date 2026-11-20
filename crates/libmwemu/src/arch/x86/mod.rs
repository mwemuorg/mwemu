pub mod context;
pub mod eflags;
pub mod flags;
pub mod fpu;
pub mod regs;

pub(crate) const XCR0_ENABLED_MASK: u64 = 0x1f;
