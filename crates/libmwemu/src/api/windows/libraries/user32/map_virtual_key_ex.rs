use crate::emu::Emu;
use crate::winapi::common::input;

pub fn MapVirtualKeyExA(emu: &mut Emu) {
    input::map_virtual_key(emu, "MapVirtualKeyExA", true);
}

pub fn MapVirtualKeyExW(emu: &mut Emu) {
    input::map_virtual_key(emu, "MapVirtualKeyExW", true);
}
