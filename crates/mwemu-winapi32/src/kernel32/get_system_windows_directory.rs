use crate::emu;
use crate::winapi::common::files::{self, Directory};

pub fn GetSystemWindowsDirectoryA(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernel32!GetSystemWindowsDirectoryA",
        false,
        Directory::Windows,
    );
}

pub fn GetSystemWindowsDirectoryW(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernel32!GetSystemWindowsDirectoryW",
        true,
        Directory::Windows,
    );
}
