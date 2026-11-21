use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcpyn(emu: &mut Emu) {
    strings::copy_string(emu, "kernel32!lstrcpyn", false, true);
}

pub fn lstrcpynA(emu: &mut Emu) {
    strings::copy_string(emu, "kernel32!lstrcpynA", false, true);
}

pub fn lstrcpynW(emu: &mut Emu) {
    strings::copy_string(emu, "kernel32!lstrcpynW", true, true);
}
