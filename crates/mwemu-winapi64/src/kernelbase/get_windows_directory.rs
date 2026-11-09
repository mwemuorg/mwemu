use crate::emu;
use crate::winapi::common::files::{self, Directory};

pub fn GetWindowsDirectoryA(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernelbase!GetWindowsDirectoryA",
        false,
        Directory::Windows,
    );
}

pub fn GetWindowsDirectoryW(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernelbase!GetWindowsDirectoryW",
        true,
        Directory::Windows,
    );
}
