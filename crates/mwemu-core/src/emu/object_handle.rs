//! The handle tables an emulated process holds. They do not need `Emu`, so
//! they are in mwemu-handles; this module keeps their path and adds the one
//! function that does need it.

pub use mwemu_handles::emu::object_handle::*;

pub mod file_handle {
    pub use mwemu_handles::emu::object_handle::file_handle::*;

    use crate::emu::Emu;
    use std::io;

    /// Default file system for an emulation: the `file_root` jail plus the
    /// main image visible at the guest path its loader entry reports, so code
    /// that opens its own module file (GetModuleFileName + CreateFile) reads
    /// the real bytes. Call after the binary is loaded.
    pub fn init_guest_file_system(emu: &Emu) -> io::Result<()> {
        if emu.cfg.filename.is_empty() {
            return init_file_system_for_image(None);
        }
        let guest = crate::windows::module_names::main_module_path(emu);
        init_file_system_for_image(Some((&emu.cfg.filename, &guest)))
    }
}
