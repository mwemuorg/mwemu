use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcmpi(emu: &mut Emu) {
    strings::compare(emu, "kernel32!lstrcmpi", false, true);
}

pub fn lstrcmpiA(emu: &mut Emu) {
    strings::compare(emu, "kernel32!lstrcmpiA", false, true);
}

pub fn lstrcmpiW(emu: &mut Emu) {
    strings::compare(emu, "kernel32!lstrcmpiW", true, true);
}
