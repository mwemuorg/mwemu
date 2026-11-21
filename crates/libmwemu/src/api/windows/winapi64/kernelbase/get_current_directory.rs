use crate::emu;
use crate::winapi::common::files::{self, Directory};

pub fn GetCurrentDirectoryA(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernelbase!GetCurrentDirectoryA",
        false,
        Directory::Current,
    );
}

pub fn GetCurrentDirectoryW(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernelbase!GetCurrentDirectoryW",
        true,
        Directory::Current,
    );
}
