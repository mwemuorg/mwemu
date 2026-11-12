use crate::emu;
use crate::winapi::common::files::{self, Directory};

pub fn GetTempPathA(emu: &mut emu::Emu) {
    files::get_directory(emu, "kernel32!GetTempPathA", false, Directory::Temporary);
}

pub fn GetTempPathW(emu: &mut emu::Emu) {
    files::get_directory(emu, "kernel32!GetTempPathW", true, Directory::Temporary);
}
