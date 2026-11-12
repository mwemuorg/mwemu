use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcmpA(emu: &mut Emu) {
    strings::compare(emu, "kernel32!lstrcmpA", false, false);
}

pub fn lstrcmpW(emu: &mut Emu) {
    strings::compare(emu, "kernel32!lstrcmpW", true, false);
}
