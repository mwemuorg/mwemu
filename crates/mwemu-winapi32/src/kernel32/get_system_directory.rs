use crate::emu;
use crate::winapi::common::files::{self, Directory};

pub fn GetSystemDirectoryA(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernel32!GetSystemDirectoryA",
        false,
        Directory::System,
    );
}

pub fn GetSystemDirectoryW(emu: &mut emu::Emu) {
    files::get_directory(emu, "kernel32!GetSystemDirectoryW", true, Directory::System);
}
