use crate::emu;
use crate::winapi::common::files::{self, Directory};

pub fn GetSystemWindowsDirectoryA(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernelbase!GetSystemWindowsDirectoryA",
        false,
        Directory::Windows,
    );
}

pub fn GetSystemWindowsDirectoryW(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernelbase!GetSystemWindowsDirectoryW",
        true,
        Directory::Windows,
    );
}
