use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn wsprintfA(emu: &mut Emu) {
    strings::wsprintf(emu, "wsprintfA", false);
}

pub fn wsprintfW(emu: &mut Emu) {
    strings::wsprintf(emu, "wsprintfW", true);
}
