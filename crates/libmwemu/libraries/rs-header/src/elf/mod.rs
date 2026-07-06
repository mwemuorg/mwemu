//! Generic ELF32/ELF64 parser and loader, extracted from mwemu.
//!
//! Like [`crate::pe`], parsing produces plain data structs and binding into
//! guest memory happens through a backend trait ([`ElfLoader`]) instead of a
//! concrete emulator, so any project can map and relocate an ELF into its own
//! memory. See `design/ARCHITECTURE.md`.

pub mod loader;
pub mod elf32;
pub mod elf64;

pub use loader::{ElfLoader, Perm};
pub use elf64::{EM_386, EM_AARCH64, EM_X86_64};

/// Error returned when an ELF image is too small or malformed to parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfError(pub String);

impl ElfError {
    pub fn new(msg: &str) -> ElfError {
        ElfError(msg.to_string())
    }
}

impl std::fmt::Display for ElfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "elf: {}", self.0)
    }
}

impl std::error::Error for ElfError {}
