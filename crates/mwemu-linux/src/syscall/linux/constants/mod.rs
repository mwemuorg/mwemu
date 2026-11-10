// The ELF loader is in the core, and so are its constants.
pub use mwemu_core::emu::loaders::elf_constants as elf;
pub mod errors;
pub mod syscalls;
pub mod system;
