use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcpyn(emu: &mut Emu) {
    strings::copy_string(emu, "kernelbase!lstrcpyn", false, true);
}

pub fn lstrcpynA(emu: &mut Emu) {
    strings::copy_string(emu, "kernelbase!lstrcpynA", false, true);
}

pub fn lstrcpynW(emu: &mut Emu) {
    strings::copy_string(emu, "kernelbase!lstrcpynW", true, true);
}
