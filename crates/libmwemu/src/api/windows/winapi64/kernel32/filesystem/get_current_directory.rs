use crate::emu;
use crate::winapi::common::files::{self, Directory};

pub fn GetCurrentDirectoryA(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernel32!GetCurrentDirectoryA",
        false,
        Directory::Current,
    );
}

pub fn GetCurrentDirectoryW(emu: &mut emu::Emu) {
    files::get_directory(
        emu,
        "kernel32!GetCurrentDirectoryW",
        true,
        Directory::Current,
    );
}
