use crate::emu::Emu;
use crate::winapi::common::files::set_current_directory;

pub fn SetCurrentDirectoryW(emu: &mut Emu) {
    set_current_directory(emu, "kernel32!SetCurrentDirectoryW", true);
}
