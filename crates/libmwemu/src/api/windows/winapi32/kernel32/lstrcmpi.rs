use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcmpi(emu: &mut Emu) {
    strings::compare_ignore_case(emu, "lstrcmpi", false);
}

pub fn lstrcmpiA(emu: &mut Emu) {
    strings::compare_ignore_case(emu, "lstrcmpiA", false);
}

pub fn lstrcmpiW(emu: &mut Emu) {
    strings::compare_ignore_case(emu, "lstrcmpiW", true);
}
