use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcpy(emu: &mut Emu) {
    strings::copy_string(emu, "kernel32!lstrcpy", false, false);
}

pub fn lstrcpyA(emu: &mut Emu) {
    strings::copy_string(emu, "kernel32!lstrcpyA", false, false);
}

pub fn lstrcpyW(emu: &mut Emu) {
    strings::copy_string(emu, "kernel32!lstrcpyW", true, false);
}
