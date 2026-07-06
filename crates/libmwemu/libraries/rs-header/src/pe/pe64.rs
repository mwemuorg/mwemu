/*
 * PE64 Structures and loader
 */

mod binding;
mod parser;
mod relocation;
mod resource;
mod types;

pub use super::shared::{
    DelayLoadDirectory, ImageDataDirectory, ImageDosHeader, ImageExportDirectory,
    ImageFileHeader, ImageImportDescriptor, ImageImportDirectory, ImageNtHeaders,
    ImageSectionHeader, IMAGE_DIRECTORY_ENTRY_DELAY_LOAD, IMAGE_DIRECTORY_ENTRY_EXPORT,
    IMAGE_DIRECTORY_ENTRY_IAT, IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_DIRECTORY_ENTRY_TLS,
    IMAGE_NUMBEROF_DIRECTORY_ENTRIES, SECTION_HEADER_SZ,
};
pub(crate) use types::IMAGE_FILE_DLL;
pub use types::{DelayLoadIAT, ImageOptionalHeader64, PE64, TlsDirectory64};
