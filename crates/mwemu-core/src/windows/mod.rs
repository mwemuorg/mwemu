// The structures, the constants and the export index do not need `Emu`; they
// are in mwemu-wintypes and keep their paths here.
pub use mwemu_wintypes::windows::{constants, export_index, structures};

pub mod module_names;
pub mod native_crt;
