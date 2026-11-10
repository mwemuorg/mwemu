use crate::emu;
use crate::winapi::common::files::{self, Directory};

pub fn GetWindowsDirectoryA(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernel32!GetWindowsDirectoryA",
        false,
        Directory::Windows,
    );
}

pub fn GetWindowsDirectoryW(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernel32!GetWindowsDirectoryW",
        true,
        Directory::Windows,
    );
}
