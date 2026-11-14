use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcmpi(emu: &mut Emu) {
    strings::compare(emu, "kernelbase!lstrcmpi", false, true);
}

pub fn lstrcmpiA(emu: &mut Emu) {
    strings::compare(emu, "kernelbase!lstrcmpiA", false, true);
}

pub fn lstrcmpiW(emu: &mut Emu) {
    strings::compare(emu, "kernelbase!lstrcmpiW", true, true);
}
