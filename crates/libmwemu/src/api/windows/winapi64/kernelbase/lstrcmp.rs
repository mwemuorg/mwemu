use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcmp(emu: &mut Emu) {
    strings::compare(emu, "kernelbase!lstrcmp", false, false);
}

pub fn lstrcmpA(emu: &mut Emu) {
    strings::compare(emu, "kernelbase!lstrcmpA", false, false);
}

pub fn lstrcmpW(emu: &mut Emu) {
    strings::compare(emu, "kernelbase!lstrcmpW", true, false);
}
