use crate::emu::Emu;
use crate::winapi::common::input;

pub(super) fn MapVirtualKeyA(emu: &mut Emu) {
    input::map_virtual_key(emu, "MapVirtualKeyA", false);
}

pub(super) fn MapVirtualKeyW(emu: &mut Emu) {
    input::map_virtual_key(emu, "MapVirtualKeyW", false);
}
