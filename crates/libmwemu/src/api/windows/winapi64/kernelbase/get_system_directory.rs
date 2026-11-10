use crate::emu;
use crate::winapi::common::files::{self, Directory};

pub fn GetSystemDirectoryA(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernelbase!GetSystemDirectoryA",
        false,
        Directory::System,
    );
}

pub fn GetSystemDirectoryW(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernelbase!GetSystemDirectoryW",
        true,
        Directory::System,
    );
}
