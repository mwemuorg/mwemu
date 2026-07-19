//! Generic PE32/PE64 parser and loader extracted from mwemu.
//! See design/PE_EXTRACTION.md for the architecture.

pub mod export_index;
mod loader;
pub mod pe32;
pub mod pe64;
pub mod readers;
pub mod shared;
pub mod structures;

pub use export_index::{ExportIndexData, ExportTarget, NamedExport, build_export_index};
pub use loader::PeLoader;
pub use shared::{
    IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_I386, pe_machine_type,
};
