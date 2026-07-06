//! GDB target implementations split by architecture.
//!
//! This module keeps the shared GDB target support small while placing the
//! x86_64, x86, and aarch64 implementations in separate files.

mod aarch64;
mod shared;
mod x86;
mod x86_64;

pub use aarch64::MwemuTargetAarch64;
pub use x86::MwemuTarget32;
pub use x86_64::MwemuTarget64;

/// Error type for GDB target operations
#[derive(Debug, Clone, Copy)]
pub struct MwemuGdbError;

impl std::fmt::Display for MwemuGdbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mwemu GDB target error")
    }
}

impl std::error::Error for MwemuGdbError {}
