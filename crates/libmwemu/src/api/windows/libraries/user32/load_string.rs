use crate::emu::Emu;
use crate::winapi::common::resources::load_string;

pub fn LoadStringA(emu: &mut Emu) {
    load_string(emu, "user32", "LoadStringA");
}

pub fn LoadStringW(emu: &mut Emu) {
    load_string(emu, "user32", "LoadStringW");
}
