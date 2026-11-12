use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcatA(emu: &mut Emu) {
    strings::concatenate(emu, "kernel32!lstrcatA", false);
}

pub fn lstrcatW(emu: &mut Emu) {
    strings::concatenate(emu, "kernel32!lstrcatW", true);
}
