use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrlenA(emu: &mut Emu) {
    strings::string_length(emu, "kernel32!lstrlenA", false);
}

pub fn lstrlenW(emu: &mut Emu) {
    strings::string_length(emu, "kernel32!lstrlenW", true);
}
