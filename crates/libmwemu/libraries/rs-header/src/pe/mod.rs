//! Generic PE32/PE64 parser and loader extracted from mwemu.
//! See design/PE_EXTRACTION.md for the architecture.

pub mod readers;
pub mod shared;
pub mod structures;
pub mod pe32;
pub mod pe64;
mod loader;

pub use loader::PeLoader;
pub use shared::{
    pe_machine_type, IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_I386,
};
