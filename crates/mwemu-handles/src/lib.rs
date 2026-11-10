//! Handle tables of mwemu: the file, registry, heap and mapping handles an
//! emulated process holds, and the jailed file system behind the file
//! handles.
//!
//! This crate knows nothing about the emulator: `mwemu-core` keeps a
//! `HandleManagement` in `Emu` and re-exports the module as
//! `mwemu_core::emu::object_handle`.
//!
//! The module keeps the path it had inside `mwemu-core`
//! (`emu::object_handle`), so the files moved here unchanged.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

pub mod emu {
    pub mod object_handle;
}

// The code names these as it did when it was part of mwemu-core. Keep those
// paths valid inside this crate.
pub(crate) use mwemu_maps::maps;
pub(crate) use mwemu_wintypes::windows;

/// Shared helpers for this crate's unit tests.
#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::OnceLock;
    use tempfile::TempDir;

    /// Keep the global file-system root alive while independent tests use it.
    pub(crate) fn file_system_root() -> &'static Path {
        static ROOT: OnceLock<TempDir> = OnceLock::new();
        ROOT.get_or_init(|| {
            let root = tempfile::tempdir().unwrap();
            crate::emu::object_handle::file_handle::init_file_system(Some(root.path())).unwrap();
            root
        })
        .path()
    }
}
