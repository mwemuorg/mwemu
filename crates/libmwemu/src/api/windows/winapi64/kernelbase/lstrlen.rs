use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrlen(emu: &mut Emu) {
    strings::string_length(emu, "kernelbase!lstrlen", false);
}

pub fn lstrlenA(emu: &mut Emu) {
    strings::string_length(emu, "kernelbase!lstrlenA", false);
}

pub fn lstrlenW(emu: &mut Emu) {
    strings::string_length(emu, "kernelbase!lstrlenW", true);
}
